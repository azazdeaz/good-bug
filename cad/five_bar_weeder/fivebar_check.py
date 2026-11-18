#!/usr/bin/env python3
"""Kinematic and force check for the 5-bar weeder (see HANDOFF.md).

Reads the OpenSCAD parameter set in params.json so the CAD model and this check
share one source of truth. All lengths in mm, angles in degrees unless noted.

Frame (rover frame, looking forward): x = lateral (+ right), z = up, z = 0 is the
soil surface. The linkage works in the x-z plane. Shafts at (-d/2, H) and (+d/2, H).
The blade tip T sits on the extension of the LEFT distal link, e mm beyond the
pin P (mirror everything for blade_on_left_link = false).

Usage: python3 -I fivebar_check.py [params.json] [parameter-set-name] [--poses]
Exit code 1 if any constraint fails.
"""
import json, math, sys
import numpy as np

def load(path, name):
    raw = json.load(open(path))["parameterSets"][name]
    out = {}
    for k, v in raw.items():
        try: out[k] = float(v)
        except ValueError: out[k] = {"true": True, "false": False}.get(v, v)
    return out

def two_link_ik(base, target, l1, l2, elbow_sign):
    """Elbow position of a 2-link chain; elbow_sign -1 = elbow to the left of base->target."""
    v = target - base; r = math.hypot(*v)
    c = (l1*l1 + r*r - l2*l2) / (2*l1*r)
    if abs(c) > 1: return None, None
    th = math.atan2(v[1], v[0]) + elbow_sign*math.acos(c)
    return th, base + l1*np.array([math.cos(th), math.sin(th)])

def ik(T, p):
    """Return (theta_left, theta_right, C, D, P) for blade tip T, elbows out."""
    d, H, L1, L2, e = p["shaft_spacing_d"], p["shaft_height_H"], p["proximal_L1"], p["distal_L2"], p["blade_extension_e"]
    A, B = np.array([-d/2, H]), np.array([d/2, H])
    t1, C = two_link_ik(A, T, L1, L2 + e, -1)          # left chain reaches the blade tip
    if C is None: return None
    P = C + (T - C) * L2 / (L2 + e)                      # pin lies on the left distal link
    L2R = p.get("distal_L2_right", L2)                   # right distal link may differ in length
    t2, D = two_link_ik(B, P, L1, L2R, +1)               # right chain reaches the pin
    if D is None: return None
    return t1, t2, C, D, P

def seg_dist(q, a, b):
    """Distance from point q to segment a-b."""
    ab = b - a; t = np.clip(np.dot(q - a, ab) / np.dot(ab, ab), 0, 1)
    return float(np.linalg.norm(q - (a + t*ab)))

def lo(a): return float(np.min(a)) if len(a) else float("nan")
def hi(a): return float(np.max(a)) if len(a) else float("nan")

def angle_between(u, v):
    return math.degrees(math.acos(np.clip(np.dot(u, v)/np.linalg.norm(u)/np.linalg.norm(v), -1, 1)))

def evaluate(p, verbose=True, grid=(41, 23)):
    """Check one parameter set. Returns (list_of_failed_checks, metrics_dict)."""
    out = print if verbose else (lambda *a, **k: None)
    d, H = p["shaft_spacing_d"], p["shaft_height_H"]
    A, B = np.array([-d/2, H]), np.array([d/2, H])
    w = math.radians(60) / p["servo_noload_s_per_60deg"]
    xs = np.linspace(-p["work_x_half"], p["work_x_half"], grid[0])
    zs = np.linspace(p["work_z_min"], p["work_z_max"], grid[1])
    rows, unreachable = [], 0
    for x in xs:
        for z in zs:
            if abs(x) > p.get("work_x_inner", 1e9) and z > p.get("work_z_max_outer", 1e9):
                continue                                     # lower lift needed near the lateral extremes
            T = np.array([x, z]); s = ik(T, p)
            if s is None: unreachable += 1; continue
            t1, t2, C, D, P = s
            eps = 1e-4
            sx, sz = ik(T + np.array([eps, 0]), p), ik(T + np.array([0, eps]), p)
            if sx is None or sz is None: unreachable += 1; continue   # on the reach boundary
            K = np.column_stack([(np.array(sx[:2]) - [t1, t2]) / eps,
                                 (np.array(sz[:2]) - [t1, t2]) / eps])
            J = np.linalg.inv(K) / 1000.0                    # m per rad
            u = (T - C) / np.linalg.norm(T - C)              # blade axis (stab direction)
            rows.append(dict(
                x=x, z=z,
                f_down_rated=p["servo_rated_torque_Nm"] / np.max(np.abs(J[1])),
                f_down_peak=p["servo_peak_torque_Nm"] / np.max(np.abs(J[1])),
                f_lat_peak=p["servo_peak_torque_Nm"] / np.max(np.abs(J[0])),
                f_axis_peak=p["servo_peak_torque_Nm"] / np.max(np.abs(J.T @ u)),
                v_down=w / np.max(np.abs(K[:, 1])) / 1000.0,
                blade_deg=math.degrees(math.atan2(u[1], u[0])),
                trans=angle_between(C - P, D - P),
                elbowL=angle_between(A - C, P - C), elbowR=angle_between(B - D, P - D),
                elbow_z=min(C[1], D[1]), elbow_top=max(C[1], D[1]),
                elbow_x=max(-C[0], D[0]), pin_z=P[1],
                # layer-crossing clearances (see HANDOFF.md, layer stack):
                clr_D_to_left_distal=seg_dist(D, C, T),       # stack A: right elbow pin passes left-distal layer
                clr_C_to_right_distal=seg_dist(C, D, P),      # stack B: left elbow pin passes right-distal layer
                clr_proximals=min(seg_dist(C, B, D), seg_dist(D, A, C))))
    n = len(rows) + unreachable
    r = {k: np.array([row[k] for row in rows]) for k in rows[0]} if rows else {}
    fails, metrics = [], {"unreachable": unreachable}
    def check(ok, msg):
        out(("  PASS " if ok else "  FAIL ") + msg)
        if not ok: fails.append(msg)
    out(f"geometry: d={d:g} H={H:g} L1={p['proximal_L1']:g} L2(left)={p['distal_L2']:g} L2(right)={p.get('distal_L2_right', p['distal_L2']):g} "
        f"e={p['blade_extension_e']:g} mm, servo {p['servo_model']}")
    out(f"working zone: x=±{p['work_x_half']:g}, z={p['work_z_min']:g}..{p['work_z_max']:g} mm, "
        f"z<={p.get('work_z_max_outer', p['work_z_max']):g} beyond |x|>{p.get('work_x_inner', p['work_x_half']):g} ({n} points)")
    check(unreachable == 0, f"reachable points {n-unreachable}/{n}")
    if not rows: return ["nothing reachable"], metrics
    deep = r["z"] <= 0
    check(r["trans"].min() >= p["min_transmission_angle_deg"] and r["trans"].max() <= 180 - p["min_transmission_angle_deg"],
          f"pin transmission angle {r['trans'].min():.0f}..{r['trans'].max():.0f} deg (limit {p['min_transmission_angle_deg']:g}..{180-p['min_transmission_angle_deg']:g})")
    el = np.concatenate([r["elbowL"], r["elbowR"]])
    check(el.min() >= p["min_elbow_angle_deg"] and el.max() <= 180 - p["min_elbow_angle_deg"],
          f"elbow angles {el.min():.0f}..{el.max():.0f} deg (limit {p['min_elbow_angle_deg']:g}..{180-p['min_elbow_angle_deg']:g})")
    check(r["elbow_z"].min() >= p["min_elbow_height"], f"lowest elbow {r['elbow_z'].min():.0f} mm above soil (min {p['min_elbow_height']:g})")
    check(lo(r["pin_z"][deep]) >= p["min_pin_height_at_depth"], f"lowest pin {lo(r['pin_z'][deep]):.0f} mm when blade below soil (min {p['min_pin_height_at_depth']:g})")
    top = max(H + p["servo_body_above_shaft"], r["elbow_top"].max())
    check(top <= p["max_top_height"], f"top of mechanism {top:.0f} mm (max {p['max_top_height']:g})")
    out(f"  info elbows reach out to x=±{r['elbow_x'].max():.0f} mm")
    need = p.get("min_layer_cross_clearance", 12)
    a, b = lo(r["clr_D_to_left_distal"]), lo(r["clr_C_to_right_distal"])
    check(max(a, b) >= need, f"layer-crossing clearance: stack A {a:.0f} mm, stack B {b:.0f} mm (need {need:g} in the chosen stack)")
    check(lo(r["clr_proximals"]) >= need, f"elbow to opposite proximal link clearance {lo(r['clr_proximals']):.0f} mm (need {need:g})")
    metrics.update(stack="A" if a >= b else "B", clr_A=a, clr_B=b)
    for name, key in (("down force @rated torque", "f_down_rated"), ("down force @peak torque", "f_down_peak"),
                      ("lateral force @peak", "f_lat_peak"), ("force along blade axis @peak", "f_axis_peak")):
        out(f"  info {name:30s} min {r[key].min():5.1f}  median {np.median(r[key]):5.1f} N")
    out(f"  info vertical speed (no load)        min {r['v_down'].min():.2f}  median {np.median(r['v_down']):.2f} m/s")
    for xr in (50, 100):
        m = (np.abs(r["x"]) <= xr) & (r["z"] <= 5)
        out(f"  info blade axis angle, |x|<={xr} mm near soil: {lo(r['blade_deg'][m]):.0f}..{hi(r['blade_deg'][m]):.0f} deg from +x")
    W = p["robot_mass_kg"] * 9.81
    lever = p["wheelbase"] + p["plane_ahead_of_front_axle"]
    f_lift = W * p["cog_ahead_of_rear_axle"] / lever
    out(f"  info max down push before the front wheels lift: {f_lift:.1f} N "
          f"(= m*g * {p['cog_ahead_of_rear_axle']:g}/{lever:g}, {p['robot_mass_kg']:g} kg robot)")
    for z0 in (-10, 0, 30):
        ok = [x for x in np.linspace(-250, 250, 1001) if ik(np.array([x, z0]), p) is not None]
        out(f"  info reach at z={z0:+d} mm: x {min(ok):.0f}..{max(ok):.0f} mm" if ok else f"  info z={z0}: unreachable")
    metrics.update(f_down_rated_min=r["f_down_rated"].min(), f_down_peak_min=r["f_down_peak"].min(),
                   f_down_peak_median=float(np.median(r["f_down_peak"])), v_down_min=r["v_down"].min(),
                   top=top, pin_min=lo(r["pin_z"][deep]), f_lift=f_lift,
                   blade_span_100=hi(r["blade_deg"][r["z"] <= 5]) - lo(r["blade_deg"][r["z"] <= 5]))
    out("RESULT:", "OK" if not fails else f"{len(fails)} constraint(s) failed")
    return fails, metrics

def pose_table(p, poses):
    """Joint values for given tool-tip poses, to validate a CAD implementation of the IK."""
    print("reference poses (tip x,z -> theta_left, theta_right [deg, atan2(z,x) of proximal link], elbows C, D, pin P [mm]):")
    for x, z in poses:
        s = ik(np.array([x, z], float), p)
        if s is None: print(f"  T=({x:+.0f},{z:+.0f}): unreachable"); continue
        t1, t2, C, D, P = s
        print(f"  T=({x:+4.0f},{z:+4.0f})  thL={math.degrees(t1):8.3f}  thR={math.degrees(t2):8.3f}  "
              f"C=({C[0]:7.2f},{C[1]:7.2f})  D=({D[0]:7.2f},{D[1]:7.2f})  P=({P[0]:7.2f},{P[1]:7.2f})")

def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    path = args[0] if args else __file__.rsplit("/", 1)[0] + "/params.json"
    p = load(path, args[1] if len(args) > 1 else "default")
    if not p.get("blade_on_left_link", True):
        print("note: blade on right link -> results are the mirror image (x -> -x)")
    fails, _ = evaluate(p)
    if "--poses" in sys.argv:
        pose_table(p, [(0, 30), (0, 0), (0, -10), (-90, 0), (90, 0), (-50, 30), (50, 30), (-90, 15), (90, 15)])
    return 1 if fails else 0

if __name__ == "__main__":
    sys.exit(main())
