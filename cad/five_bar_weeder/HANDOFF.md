# 5-bar weeder: parametric model handoff

This is everything needed to build a parametric CAD model of a two-servo, five-bar linkage weeding tool for the Good Bug under-canopy rover. The model is the first step; firmware notes are included so the geometry supports them.

The files in this folder:

| File | What it is | Status |
|---|---|---|
| `HANDOFF.md` | This document | done |
| `params.json` | Shared parameter sets in OpenSCAD's Customizer format. It is the single source of truth for the CAD model and for the checker. | done |
| `fivebar_check.py` | Kinematics, force, speed and clearance checker. It reads `params.json` and prints `RESULT: OK` or the failing constraints. | done |
| `fivebar_weeder.scad` and the part files | The parametric model | **to build** |

Background research is in `reports/Small under canopy weeding rover.md`, with its notes in `research_notes/Small under canopy weeding rover/`.

---

## 1. What we are building and why

The tool is a planar five-bar linkage mounted at the front of the rover:

- **Plane:** it works in the lateral–vertical plane, i.e. across the direction of travel and up/down.
- **Drive:** two servos with shafts pointing forward drive the two upper ("proximal") links.
- **Joint:** the two lower ("distal") links meet at a pin.
- **Blade:** a short blade sticks out beyond the pin along the **left** distal link.
- **Division of labour:** the linkage aims sideways and strikes down; the rover's wheels handle forward/backward positioning.

The design follows from the research:

- **Pull force is the limit.** A 1–5 kg rover can only pull or push about 2–25 N, while a 15 cm hoe blade in moist or crusted soil needs 20–150 N. The tool must be narrow, targeted and low-force.
- **A blade edge needs far less force than a punch.** A 18 mm × 0.5 mm edge presents ~9 mm² to the soil; a 10 mm flat stamp presents ~78 mm². This is my rough estimate, not from the sources.
  - In moist tilled soil (penetration resistance ~0.5–1 MPa) the blade needs roughly 5–10 N to enter.
  - In a dry crust (2–4 MPa) it needs about 20–35 N.
  - Treat these as rough engineering estimates until the bench test (section 13) measures them.
- **Weeds must be hit small and often.** The target is the stage from just-sprouted seedling to two true leaves, with a visit every 1–3 days.
  - Broadleaf seedlings cut below their seed leaves mostly die.
  - Grasses must be cut about 1 cm below the surface (undercut), or uprooted.
  - Nutsedge and bindweed (perennials) can only be suppressed, not killed.
- **Why a five-bar:**
  - Its lateral reach (±90–100 mm) is wider than the 150 mm body, so the rover can work near pepper stems without driving close to them.
  - Strike paths are programmable: stab, repeated stab, undercut, and touching the ground to find its height.
  - It is fast (no-load blade speed ≥0.25 m/s everywhere in the working zone) and folds compactly under the canopy.
- **The robot's weight caps the down force.** Pushing down ahead of the front axle lifts the front wheels (section 8). The linkage, the servo torque and the robot's mass and centre of gravity must be designed together.

## 2. Coordinate frames and conventions

- **Units:** millimetres and degrees, unless a name says otherwise, e.g. `_Nm` or `_kg`.
- **Rover frame R:** x is lateral (+ to the right, seen from behind the rover looking forward), y is forward (direction of travel), z is up.
  - The origin is on the soil surface, under the midpoint of the front axle.
- **Linkage frame L:** the same axes as R, shifted forward by `plane_ahead_of_front_axle`. The linkage moves in L's x–z plane (y = 0 in L).
  - **Everything in `fivebar_check.py` is 2D in this x–z plane.**
- **Servo shafts:** they point in +y (forward). The left shaft is at A = (−d/2, H) and the right shaft at B = (+d/2, H), where `d = shaft_spacing_d` and `H = shaft_height_H` (shaft height above the soil).
- **Joint names:**
  - C is the left elbow (end of the left proximal link).
  - D is the right elbow.
  - P is the pin where the two distal links meet.
  - T is the tool point: the midpoint of the blade's cutting edge.
- **Angles:** θ_L and θ_R are the proximal link directions, measured as `atan2(z, x)` in the x–z plane viewed from behind (+x right, +z up, counter-clockwise positive).
  - Mapping servo counts to θ (sign and zero offset) is a calibration step; the CAD model only uses θ.
- **Blade axis:** u = unit vector (T − C)/|T − C|, i.e. along the left distal link pointing toward the tip. The blade's cutting edge runs **along y**, the direction of travel. It is perpendicular to the linkage plane so it tolerates the rover stopping ±7–10 mm off target.

## 3. Kinematics (implement these as OpenSCAD functions)

**Assembly mode:**

- Elbows out: the left elbow C sits up and left of A, the right elbow D sits right of B.
- The pin P sits below both elbows.
- Never let the mechanism pass through the singular (straight-arm) configuration between these modes. The checker's limits on elbow and transmission angles keep it away.

**Left chain:** a 2-link arm from A to the tool point T, with lengths L1 and (L2L + e). Here L2L = `distal_L2` (elbow to pin on the left) and e = `blade_extension_e` (pin to tool point along the same link).

```
v   = T - A ;  r = |v|
phi = atan2(v.z, v.x)
alpha = acos((L1^2 + r^2 - (L2L+e)^2) / (2*L1*r))      # unreachable if |arg| > 1
theta_L = phi - alpha                                   # elbow out (left)
C = A + L1*(cos theta_L, sin theta_L)
P = C + (T - C) * L2L/(L2L+e)                           # pin lies on the left distal link
```

**Right chain:** a 2-link arm from B to the pin P, with lengths L1 and L2R = `distal_L2_right`.

```
w   = P - B ;  s = |w|
theta_R = atan2(w.z, w.x) + acos((L1^2 + s^2 - L2R^2) / (2*L1*s))   # elbow out (right)
D = B + L1*(cos theta_R, sin theta_R)
```

**Forward kinematics**, for animating from servo angles:

1. Compute C and D from θ_L and θ_R.
2. P is the intersection of circle(C, L2L) and circle(D, L2R) with the **lower** z.
3. T = P + e·(P − C)/L2L.

**Reference values to test your OpenSCAD functions against.** These are from `python3 -I fivebar_check.py --poses` with the `default` set. Match them to better than 0.01 mm / 0.01°:

```
T=(  +0, +30)  thL=-171.182  thR= -34.015  C=( -74.29,  80.80)  D=(  64.73,  56.44)  P=( -24.76,  46.93)
T=(  +0,  +0)  thL=-150.090  thR= -38.925  C=( -67.01,  60.08)  D=(  61.68,  52.30)  P=( -22.34,  20.03)
T=(  +0, -10)  thL=-143.318  thR= -41.310  C=( -63.12,  54.16)  D=(  60.07,  50.39)  P=( -21.04,  11.39)
T=( -90,  +0)  thL=-178.796  thR= -99.598  C=( -74.99,  88.74)  D=(   5.00,  30.84)  P=( -85.00,  29.58)
T=( +90,  +0)  thL= -68.804  thR=  10.149  C=(   6.69,  34.06)  D=(  74.06, 100.57)  P=(  62.23,  11.35)
T=( -50, +30)  thL=-208.022  thR= -75.834  C=( -67.97, 118.19)  D=(  29.68,  31.82)  P=( -55.99,  59.40)
T=( +50, +30)  thL=-114.455  thR=  13.878  C=( -39.84,  35.38)  D=(  73.25, 104.39)  P=(  20.05,  31.79)
T=( -90, +15)  thL=-192.972  thR=-100.095  C=( -73.47, 103.47)  D=(   4.48,  30.93)  P=( -84.49,  44.49)
T=( +90, +15)  thL= -73.847  thR=  19.592  C=(   1.69,  32.37)  D=(  71.53, 110.12)  P=(  60.56,  20.79)
```

The angles are not wrapped, so −208° is the same as +152°. Wrap them however your servo mapping needs.

**The blade angle changes across the working zone.** The blade is fixed to a link, so its angle follows the link:

- about −75° to −19° (from +x) across the central ±50 mm near the soil;
- about −100° to −16° across the full width.

Consequences:

- Strikes should move **along the blade axis u**, never straight down at an arbitrary blade angle, or the blade pushes broadside through the soil.
- At the right-hand extreme the blade is shallow, so a stab there is effectively an undercut.
- If a constant blade angle turns out to matter, the fallback is a micro wrist servo at the tip (not in scope).
- The design is asymmetric: setting `blade_on_left_link=false` mirrors everything (x → −x).

## 4. Parameters

There are two parameter sets in `params.json`. The `default` set fits under a late-season pepper canopy (top ≤120 mm). The `wide_reach_early_season` set reaches the full ±100 mm but stands up to 140 mm tall.

**Geometry (mm):**

| Parameter | `default` | `wide_reach_early_season` | Meaning |
|---|---|---|---|
| `shaft_spacing_d` | 30 | 50 | Lateral distance between the servo shafts. It must be ≥ 2 × (shaft to inner end of servo case) + 2; **measure it** (section 12). |
| `shaft_height_H` | 90 | 100 | Shaft axis height above flat soil, with the rover standing on it |
| `proximal_L1` | 60 | 60 | Shaft to elbow, both sides |
| `distal_L2` | 60 | 80 | Left distal: elbow C to pin P |
| `distal_L2_right` | 90 | 110 | Right distal: elbow D to pin P |
| `blade_extension_e` | 30 | 30 | Pin P to tool point T along the left distal link (pin boss + clamp + exposed blade) |
| `blade_on_left_link` | true | true | Mirror switch |
| `plane_ahead_of_front_axle` | 30 | 30 | Linkage plane's distance ahead of the front axle (y) |

**Working zone (mm), in the linkage frame relative to the soil:**

| Parameter | `default` | `wide_reach_early_season` | Meaning |
|---|---|---|---|
| `work_x_half` | 90 | 100 | Required lateral reach, ± |
| `work_z_min` / `work_z_max` | −10 / 30 | −10 / 30 | Deepest cut and highest lift |
| `work_x_inner` / `work_z_max_outer` | 50 / 15 | 50 / 15 | Beyond \|x\| > 50, lift only to 15 mm, which keeps the elbows low |

**Servo**, with STS3215 values from DFRobot/Core Electronics listings ([DFRobot](https://www.dfrobot.com/product-2961.html), [datasheet](https://core-electronics.com.au/attachments/uploads/sts3215-smart-servo-datasheet-translated.pdf)):

| Parameter | Value | Note |
|---|---|---|
| `servo_model` | STS3215 | 45.2 × 24.7 × 35 mm, 55 g, 25-tooth spline, 345:1 gearing, 12-bit magnetic encoder |
| `servo_rated_torque_Nm` | 0.49 | 5 kg·cm continuous at 7.4 V |
| `servo_peak_torque_Nm` | 0.95 | About half the 19.5 kg·cm (1.91 N·m) stall torque at 7.4 V; only for short strikes |
| `servo_noload_s_per_60deg` | 0.192 | At 7.4 V |
| `servo_body_above_shaft` | 14 | Half of the 24.7 mm case height plus bracket, with the case's long side lateral |

**Robot and check limits:**

| Parameter | Value | Meaning |
|---|---|---|
| `robot_mass_kg`, `wheelbase`, `cog_ahead_of_rear_axle` | 3.0, 140, 70 | Used for the front-wheel lift limit (section 8). Placeholders until the chassis exists. |
| `max_top_height` | 120 / 140 | Top of the mechanism above the soil. Late-season pepper branches hang 10–25 cm up, so the research suggests ≤10–12 cm. |
| `min_elbow_height` | 20 | Elbows stay clear of clods and weeds |
| `min_pin_height_at_depth` | 3 | Pin and bearings stay above the soil while the blade is in it |
| `min_transmission_angle_deg` | 25 | Angle between the distal links at P; keeps force transmission and stiffness sane |
| `min_elbow_angle_deg` | 20 | Angle at each elbow, away from 0° and 180° |
| `min_layer_cross_clearance` | 12 | In-plane clearance where an elbow pin passes another link's layer (section 6) |

**To add in the CAD** (they don't affect the checker): link widths and thicknesses, bearing sizes, blade and clamp dimensions, horn bolt pattern, servo case details, chassis mounting pattern, guard geometry. Suggested names and defaults are in section 6.

## 5. Verified behaviour of the parameter sets

Output of `python3 -I fivebar_check.py` (`default`), all checks passing:

- **Reach:** ±90 mm guaranteed. At −10 mm the blade reaches x = −126…+96 mm, and the elbows stick out to ±75 mm.
- **Down force at the tool point:**
  - at rated servo torque: min 5.9 N, median 7.3 N;
  - at peak torque (short strikes): min 11.5 N, median 14.1 N.
- **Other forces at peak torque:** lateral min 10.6 N; along the blade axis min 15.8 N.
- **No-load blade speed:** vertical min 0.25 m/s, median 0.44 m/s.
- **Heights:** top of mechanism 118 mm; lowest elbow 30 mm; lowest pin 3 mm with the blade in the soil.
- **Angles:** transmission angle 65–140°; elbow angles 37–150°.
- **Front-wheel lift limit:** 12.1 N for a 3 kg robot. The servos and the robot are evenly matched at about 12 N, which is enough for moist tilled soil but not for a dry crust.

The `wide_reach_early_season` set: ±100 mm reach, down force 6.6 N at rated torque and 12.8 N at peak (minimums), top 138 mm.

How the `default` set reacts to changes (each run separately):

| Change | Result |
|---|---|
| `d` = 35 | OK; peak force falls to 10.4 N |
| `d` = 40 | FAIL: elbow angle reaches 164° |
| `H` = 85 | FAIL: the pin goes into the soil and the top reaches 119 mm |
| MG996R servos (~0.3 N·m rated, ~0.55 N·m peak, guessed; no published rating) | 3.6 / 6.6 N; too weak, use only for dry-run tests |
| Any servo with k× torque | Forces scale ×k; the geometry checks don't change |

If the measured servo dimensions force a larger `d`, re-run the sweep: try H, L1, L2L, L2R and e in steps of 5–10 mm, keep the sets that pass, and pick the one with the highest minimum down force. A sweep takes about a minute.

## 6. Parts to model

All parts are driven by `params.json` plus the CAD-only parameters below. Use one `fivebar_weeder.scad` with a `part` selector (`"assembly"`, `"bracket"`, `"proximal"`, `"distal_left"`, `"distal_right"`, `"blade_clamp"`, `"guard"`, `"bench_stand"`), and tool-point pose parameters `tip_x` and `tip_z`, or `$t` animation.

### 6.1 Servo mock: `servo(model)`
- **STS3215:** 45.2 × 24.7 × 35 mm case.
- **MG996R** for dry runs: 40.7 × 19.7 × 42.9 mm, 54 mm across the mounting ears ([Core Electronics](https://core-electronics.com.au/mg996r-metal-gear-servo-11-kg-cm.html)).
- **Shaft position, horn bolt pattern and mounting-hole positions are not published in anything we found.** Make them parameters (`servo_shaft_from_end`, `servo_horn_pcd`, `servo_mount_holes`) and fill them in from calipers or the Feetech drawing.

### 6.2 Servo bracket: `bracket()`
- Holds both servos with their shafts at (±d/2, H) in the linkage plane, pointing +y.
- **Orientation:** long side of each case lateral, the cases extending outward from the shafts. This keeps the top at H + 14 mm. With d = 30 and roughly 11–14 mm from shaft to case end, the pair is about 95–100 mm wide.
- **Chassis interface:** a separate parametric plate (`mount_pattern`, `mount_hole_d`) bolted to the rover's front. The current Good Bug chassis (`cad/BaseChassis.stl`, 113 × 77 × 44 mm, Ø50 mm wheels) is lower and narrower than the 150 × 200 mm target, so this plate is where the two meet. For the first tests, mount the bracket on the bench stand (6.8), not the rover.
- **Stiffness:**
  - Any sideways or forward (y) load at the blade bends the shafts and the bracket.
  - Brace both servo cases to a common plate.
  - If the servo case has a boss opposite the output, add a back support bearing for it.
  - Ribs: at least 3 mm walls.

### 6.3 Proximal links ×2: `proximal()`
- Length L1 between the shaft axis and the elbow bearing axis.
- Horn end: bolts to the servo's metal disc horn (`servo_horn_pcd`, M2/M3 per the horn).
- Elbow end: bearing seat (6.6).
- **Section:** in-plane width `link_w` ≥ 12 mm near the horn, thickness `link_t` = 6 mm.
  - Check, in PETG printed flat: peak stall torque 1.91 N·m as bending at the horn gives σ = M/(t·w²/6) = 1910/(6·12²/6) ≈ 13 MPa.
  - That is roughly a 3–4× margin on PETG's ~50 MPa (an approximate value).
  - Thinner sections creep and flex, and flex shows up directly as tool-point error.

### 6.4 Left distal link with blade mount: `distal_left()`
- Bearing at C, bearing at P (L2L apart), then the blade clamp extending to T, a total of L2L + e from C. T is the midpoint of the cutting edge.
- Unlike the right distal link, this link is loaded in bending at T. Moment at P ≈ 16 N × 30 mm ≈ 0.5 N·m, so the same section as the proximal links is fine.
- **Blade clamp** (`blade_clamp()`):
  - Grips a piece of standard **18 mm snap-off blade**, 0.5–0.55 mm thick ([Stanley FatMax](https://www.stanleytools.co.uk/product/0-11-718/stanley-fatmax-18mm-snap-blades-5-pk)).
  - Edge along y, `blade_edge_len` = 24 mm (two segments; one segment is ~12.5 mm, my estimate from 100–110 mm / 8 segments).
  - `blade_grip` = 7 mm clamped, `blade_exposed` = 11 mm, which leaves ≥1 mm beyond the 10 mm maximum depth.
  - Two M3 screws through a clamp plate, with a locating step so a new blade drops in at the same T.
  - **e = pin boss radius + clamp length + exposed blade; keep it consistent with `blade_extension_e`.**
  - Centre the blade's y-extent on the middle of the layer stack to reduce twisting.
- **Optional compliance:** a TPU pad or flexure behind the clamp (`compliance_travel` = 0–3 mm), so a stone strike doesn't go straight into the servo gears. The primary protection is the servo's torque limit (section 10); keep the default at 0 if the servo limit proves enough.

### 6.5 Right distal link: `distal_right()`
- Bearings at D and P, L2R apart.
- Pinned at both ends, so it carries mainly tension or compression; slimmer is fine (w ≥ 10 mm).

### 6.6 Joints
- Three passive joints: C, D and P.
- **Default:** M3 shoulder screw (3 mm shoulder) through 2× 683-2RS bearings (3 × 7 × 3 mm), rubber-sealed against dust and mud.
  - Lighter alternative: MR63-2RS (3 × 6 × 2.5 mm).
  - Parameters: `bearing_id`, `bearing_od`, `bearing_w`, `bearing_count_per_joint`.
- **Play budget:** less than about 0.1 mm per joint, so total tip play stays under about 1 mm.
  - For scale, servo resolution is 4096 steps/rev = 0.088°, i.e. ~0.09 mm at 60 mm.
- Pin bosses: radius ≥ bearing OD/2 + 2 mm.

### 6.7 Layer stack (along y)
Every link lives on a y-layer of thickness `link_t`, with `layer_gap` = 0.5 mm washers between layers:

| Layer | Contents |
|---|---|
| 1 (on the servo horns) | Left and right proximal links |
| 2 | Left distal link plus the blade clamp |
| 3 | Right distal link |

- The pin P joins layers 2 and 3.
- The left elbow C joins layers 1 and 2.
- The right elbow D joins layers 1 and 3, so its shoulder screw passes through layer 2. **That screw must never touch the left distal link.**
  - The checker reports this as "stack A" clearance: min 70 mm in `default`, against 12 mm needed.
  - The swapped stack (B: right distal on layer 2, left distal on layer 3) also clears, at 54 mm. Either works; A keeps the blade closer to the servos.
- The blade and clamp lie beyond P along u, where no other link reaches, so they may span all layers in y.

### 6.8 Bench stand: `bench_stand()`
- Holds the bracket at a set H (default 90) above a soil tray, e.g. 400 × 250 × 80 mm, with the linkage plane across the tray.
- Optional y-rail so the "rover" position can be stepped by hand.
- This is the first thing to print: it supports every test in section 13 without the rover.

### 6.9 Guard and park pose: `guard()`
- **Park pose:** T = (0, +30). Inside the checked zone, so the top stays ≤118 mm.
- A fixed sleeve on the bracket covers the cutting edge in the park pose, so people and plants are safe while the rover drives.

### 6.10 Visual-only envelopes (assembly view)
- Soil plane at z = 0.
- Canopy plane at `max_top_height`.
- Rover body box (150 × 200 mm target) and wheels.
- Pepper stem keep-out cylinders at ±(inter-row/2) (`row_spacing` = 750 mm default; `stem_keepout_r` = 25 mm).
- Optional camera frustum:
  - Pi Camera Module 3 Wide, 102° × 67° field of view (from the research notes).
  - Mounted above and behind the linkage plane, looking down at the strip.
  - Use it to check the linkage doesn't hide the target strip in the park pose.

## 7. Motion primitives the model should animate

Render each primitive as a `$t` animation and as key-pose PNGs. The same definitions become firmware later.

| Primitive | Path, in the linkage frame relative to the measured soil height z0 | Defaults |
|---|---|---|
| `park` | T = (0, +30) | — |
| `probe` (find the soil) | From (x, +15), move −z at ≤20 mm/s with the servo torque capped at ~30 %; stop when the load rises or the position error grows; record z0 | v = 20 mm/s |
| `stab` | Start at T = W − 25·u above the weed W; move along +u to depth δ below z0; dwell; retract along −u. Repeat n times with ±3 mm lateral jitter. | δ = 8 mm, dwell 50 ms, n = 2–3 |
| `undercut` | Plunge along u at (x_w − s, z0 − δ) on the side **away from the nearest crop stem**; sweep laterally to x_w + s at depth δ; lift | s = 15 mm, δ = 8–10 mm |
| `travel` | Park pose; the rover may move in y only now | — |

Rules the model and firmware must respect:

- **Never move the rover in y while the blade is below z0.** The blade edge and the links are not designed for forward loads.
- Lift and approach heights only up to 15 mm when |x| > 50 mm, which keeps the elbows low.
- Plan on ~1 s per weed: lateral move plus 2–3 stabs. At 10–20 weeds/m² in an 18 cm strip, that is 2–4 weeds per metre of travel.
- Each weed costs roughly 5–10 J of electrical energy, which is well inside the battery budget.

## 8. Force limit from the robot

Pushing down at the tool point lifts the rover's front about the rear axle. The maximum push before the front wheels lift is:

```
F_max = m·g · (CoG distance ahead of rear axle) / (wheelbase + plane_ahead_of_front_axle)
```

With the placeholders (3 kg, CoG 70 mm ahead of the rear axle, 140 mm wheelbase, plane 30 mm ahead of the front axle), F_max = 12.1 N. To raise it:

- put the battery forward;
- add mass;
- move the linkage plane back toward the front axle.

The front wheels also unload progressively before they lift, which costs traction, so stay well below F_max during undercuts. The checker prints this number; once the chassis is designed, update the three robot parameters.

## 9. Constraints

**`fivebar_check.py` already checks:**

- reach over the whole working zone;
- transmission and elbow angle limits;
- elbow height;
- pin height while the blade is in the soil;
- top height;
- layer-crossing and elbow-to-link clearances;
- forces, speeds and blade-angle ranges (reported);
- front-wheel lift force.

**The CAD model must still check:**

- 3D interference between parts with real widths: bosses, screw heads, horns, the clamp, servo cases. Sweep the working zone and the `park` → `stab` → `undercut` paths. Use OpenSCAD `assert`s on joint distances, or render collision checks with `intersection()` at a sampled set of poses.
- Proximal links don't hit the servo cases or bracket at their angle extremes. The reference poses span θ_L from −208° to −69° and θ_R from −100° to +20°, so allow at least 160° of horn rotation clearance per side.
- Nothing except the blade goes below z = 0 at any pose in the zone.
- The total stack depth in y and the overhang ahead of the chassis fit inside the rover's front envelope.

## 10. Integration notes (for the firmware after the model)

- **Servo bus:**
  - STS3215 servos are daisy-chained on a half-duplex TTL serial bus through a bus-servo adapter board to the Pi's UART.
  - They report position, speed, load, voltage, temperature and current, and accept a torque limit.
  - Use that to:
    - set a torque cap (protects the gears on stones);
    - find the soil by touch (`probe`), which also fixes the single-camera height error (1 cm of soil-height error ≈ 1 cm of aiming error at 10 cm camera height);
    - control cut depth;
    - detect a stone (stall before reaching depth: abort and shift 5 mm).
  - Confirm the protocol details and default baud rate against the Feetech manual.
- **Power:**
  - 7.4 V (2S Li-ion) for the servos, separate from the Pi's 5 V supply, with a bulk capacitor.
  - Stall current is 2.5 A per servo at 7.4 V, so size the supply for ≥5 A.
- **Existing code:** `drivers/src/weeder.rs` drives a single PWM motor on GPIO 24 (`Msg::SetWeederSpeed`). The five-bar needs a new driver and new messages, e.g. `WeederStrike { x, z, primitive }`. Port the IK from section 3 to Rust and test it against the reference poses.
- **Vision interface:** detections come from the camera in the rover frame. The rover stops when the weed's y matches the linkage plane (wheel odometry), then the linkage targets x. Put a colour marker on the blade clamp so the camera can confirm the tool point sits over the weed before striking.
- **Agronomy and safety rules:**
  - No blade point within 25 mm of a detected pepper stem. A blade that touches the crop can spread mechanically transmitted viruses such as ToBRFV and PMMoV.
  - Depth ≤10 mm within 50 mm of the row; pepper yield was unaffected by 3 cm cultivation but hurt by 9 cm.
  - Never strike on drip tape or plastic mulch.
  - Blade in the guard whenever the rover drives.
  - Strike in moist (not muddy) soil: penetration is easier then. Dry crust may exceed the ~12 N available.

## 11. Materials and bill of materials (per tool)

| Item | Qty | Notes |
|---|---|---|
| Feetech STS3215 servo, 7.4 V | 2 | About $15–20 each; check the voltage when comparing torque figures |
| Bus-servo adapter board | 1 | UART to half-duplex bus |
| 683-2RS bearings (3 × 7 × 3) | 6 | 2 per joint |
| M3 shoulder screws, washers, nylock nuts | 3 sets | Joint pins |
| M2/M3 screws for the horns | 8 | Per the horn pattern |
| 18 mm snap-off blades | consumable | Replace when dull or rusted |
| 2S Li-ion supply, ≥5 A | 1 | Separate from the Pi's 5 V |
| Printed parts | — | PETG or ASA. Not PLA: polytunnels reach ~42 °C and dark parts in sun run hotter. TPU for the optional compliance pad. Print links flat. |

## 12. Open items to measure or decide

1. **STS3215 shaft position relative to the case, horn bolt circle, mounting holes:** calipers or a Feetech drawing. If the shaft is more than ~16 mm from the inner case end, `d` = 30–35 is not buildable side by side; re-run the sweep (section 5).
2. **Soil penetration and undercut force** against depth, in moist and in dry soil, with the 18 mm blade: this decides whether ~6–12 N is enough (first bench test, section 13).
3. **Joint play and link flex** under 10–15 N at the tool point. Target under 1 mm of tool-point error.
4. **Rover mass and centre of gravity** once the chassis exists: update `robot_mass_kg`, `wheelbase` and `cog_ahead_of_rear_axle`.
5. **Real canopy clearance in pepper**, measured every two weeks, to decide between the `default` and `wide_reach_early_season` sets and whether 120 mm is the right cap.
6. **Whether the changing blade angle matters in practice.** If stabs at extreme x perform poorly, consider a micro wrist servo.

## 13. Bench tests the model should enable (in order)

1. **Kinematic accuracy:** command the reference poses and measure the tool point on graph paper. Target ≤1 mm.
2. **Penetration force:** stab into tray soil in steps of depth and log the servo load readings. Do it in moist tilled soil and in dry crusted soil.
3. **Kill rate:** seed a mustard or radish (broadleaf) and a grass in the tray. At cotyledon to two-leaf stage, compare one stab, three stabs and one undercut; count survivors after 7 days.
4. **Robustness:** 1,000 strike cycles with a buried stone in the path. Check gears, joint play and blade edge.

## 14. Suggested file layout and commands

```
cad/five_bar_weeder/
  HANDOFF.md            this file
  params.json           shared parameter sets (OpenSCAD Customizer format)
  fivebar_check.py      checker: python3 -I fivebar_check.py [params.json] [set] [--poses]
  fivebar_weeder.scad   main file: top-level variables named exactly as in params.json, plus part/tip_x/tip_z
  lib/kinematics.scad   ik_left(), ik_right(), fk() from section 3
  lib/parts.scad        servo(), bracket(), proximal(), distal_left(), distal_right(), blade_clamp(), guard(), bench_stand()
  stl/                  exported parts (consider adding to .gitignore)
```

- OpenSCAD is not installed in the cloud container: `apt-get install openscad`, or use the AppImage.
- **Parameter sets:** `openscad -p params.json -P default -D 'part="assembly"' -D tip_x=0 -D tip_z=0 -o assembly.png fivebar_weeder.scad`.
- **STL export:** `-D 'part="proximal"' -o stl/proximal.stl`.
- OpenSCAD only applies a parameter-set value to a top-level variable that is declared with a default in the `.scad` file. Values in the JSON are strings, and OpenSCAD converts them using the declared default's type.

**Acceptance checklist:**

- [ ] `python3 -I fivebar_check.py` and `python3 -I fivebar_check.py params.json wide_reach_early_season` both print `RESULT: OK` (they do today).
- [ ] The OpenSCAD IK reproduces the reference-pose table (section 3) to 0.01 mm.
- [ ] The assembly renders without interference at the nine reference poses and along the `stab` and `undercut` animations, for both parameter sets.
- [ ] Changing any length in `params.json` regenerates consistent parts: bearing seats move with L1, L2L and L2R; the clamp moves with e.
- [ ] Every printable part exports as a manifold STL.
- [ ] The bench stand holds the shafts at H ± 1 mm above the tray rim.
