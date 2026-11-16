# Platform engineering of a ~15 x 20 cm under-canopy weeding rover: mobility, traction, terramechanics, energy, coverage

Scope note: the rover studied here is about 15 cm wide x 20 cm long x 10-25 cm high, 1-5 kg, with RGB cameras, IMU, optional GPS, a Raspberry Pi or small Jetson, and a weeding tool. It works between crop rows under the canopy, with bell pepper (Capsicum annuum) as the main crop, in open fields and in polytunnels on soil.

How to read these notes:
- Lines marked **DERIVED** are my own calculations. The inputs and assumptions are written out next to each one, and the script that produced them is described at the end of each section.
- Lines marked **BACKGROUND (unverified in session)** are textbook formulas or values I know but could not re-check against a fetched source this session. Treat them as needing a check before anyone relies on them.
- Some hosts could not be reached from this session: par.nsf.gov, www.researchsquare.com and ag.arizona.edu. Material from those URLs comes from search-result snippets, not full-text reads.
- Part of the sourced material comes from an earlier researcher's saved search results. Their original URLs are cited.

---

## 1. Terramechanics at small scale: sinkage, rolling resistance, traction and drawbar pull for a 1-5 kg rover; wheel size relative to clods, furrows, drip tape and residue

### Takeaway
Classical Bekker/Wong theory does not reliably describe this rover. It was built and validated for heavy vehicles and loses accuracy below about 50 cm wheel diameter and about 45 N wheel load. This rover sits far inside that regime, at roughly 2.5-12 N per wheel and 8-16 cm wheels. Taken literally, Bekker predicts sub-millimetre sinkage and negligible compaction resistance, which is not credible for freshly tilled, cloddy or wet soil.

The practical design quantities are these:
- Net drawbar pull is plausibly about 0.2-0.5 of robot weight on workable soil. That is only about 2-25 N for 1-5 kg.
- Motion resistance on loose tilled soil should be assumed at 0.1-0.3 of weight, driven by bulldozing, clods, residue and slip-sinkage.
- Wheel diameter should be at least about 2-3 times the largest obstacle height, which points to about 10-15 cm wheels and at least 5 cm ground clearance.

### Cited Findings
**Limits of Bekker/Wong for small, light wheels**
- Meirion-Griffith and Spenko found that Bekker's pressure-sinkage equations lose accuracy for wheels below about 50 cm diameter and loads below about 45 N. Their modified pressure-sinkage model explicitly includes a dependence on wheel diameter and was validated in 160 experiments using 5 wheel diameters and 3 soil types (J. Terramechanics vol. 48, 2011; IROS 2011) — [ISTVS publication news](https://istvs.org/publication-news/2012/7/3/a-pressure-sinkage-model-for-small-diameter-wheels-on-compactive-deformable-terrain); [ISTVS 2014 entry](https://www.istvs.org/publication-news/2014/1/15/a-pressure-sinkage-model-for-small-diameter-wheels-on-compactive-deformable-terrain)
- In 2013 (J. Terramechanics vol. 50) the model was extended to include wheel width. That work used compactive clay/silt soil, 125 tests over 35 wheel geometries, and X-ray imaging of soil deformation below the surface — [ISTVS](https://www.istvs.org/publication-news/2014/1/15/a-pressure-sinkage-model-for-small-diameter-wheels-on-compactive-deformable-terrain)
- A 2014 version covers dilative soils (sand, Martian regolith) from more than 120 tests. It was used in A* path planning for a UGV and field-validated on moist sandy terrain — [ISTVS 2014](https://www.istvs.org/publication-news/2014/1/15/development-and-experimental-validation-of-an-improved-pressure-sinkage-model-for-small-wheeled-vehicles-on-dilative-deformable-terrain). A separate paper simulated a small-wheeled UGV with the same sinkage and compaction models — [Int. J. Vehicle Design (Inderscience)](https://www.inderscience.com/filter.php?aid=58499)
- Another source says Bekker theory "was developed and validated for heavy vehicles" and that small robots often deviate from it. It adds that slip-induced sinkage, where a robot "digs itself in" by excavating friable terrain, is less often considered — [par.nsf.gov (search snippet; host unreachable)](https://par.nsf.gov/servlets/purl/10304353)
- A 2013 DTIC report on scaling classical terramechanics to lightweight vehicles compares predicted drawbar force, torque and sinkage with measurements. It notes that its soil parameters come from fitted Bekker pressure-sinkage and Janosi shear data, "each with an inherent uncertainty due to test variability" — [DTIC ADA592072](https://apps.dtic.mil/sti/pdfs/ADA592072.pdf)
- Granular scaling laws, which predict a large wheel from tests on a small one, were tested for lunar gravity in 2022. Drawbar pull was negative for both wheels at every slip tested, meaning thrust stayed below resistance — [ASCE Earth & Space 2022](https://ascelibrary.com/doi/10.1061/9780784484470.004)
- Irani et al. extended Bekker-Wong into a dynamic pressure-sinkage model for lightweight robots. It reproduces the oscillation of drawbar pull and sinkage seen with rigid wheels in loose sand — [Carleton/Irani](https://carleton.ca/mdl/wp-content/uploads/Irani_Application-of-a-Dynamic-Pressure-Sinkage.pdf); [Irani dynamic wheel-soil model](https://carleton.ca/mdl/wp-content/uploads/Irani_Dynamic-Wheel-Soil-Model-for-Lightweight-Mobile.pdf)
- The Bekker parameters are kc (cohesive modulus of deformation), kφ (frictional modulus of deformation) and n (deformation exponent). Reported values vary widely between sources — [arXiv 2506.22579](https://arxiv.org/pdf/2506.22579). One thesis using Bekker-Wong found estimated internal friction angles from 16.5° to 37.5° for the *same* soil — [Univ. Pretoria thesis](https://repository.up.ac.za/bitstreams/d0d755e0-8835-4f1e-a070-0d7fc164094c/download)

**Grousers, slip and turning on loose soil**
- Grousers can raise drawbar pull for rigid wheels in loose granular soil. With too few grousers, soil flows forward ahead of the wheel and adds rolling resistance — [Skonieczny et al., IROS 2012 (CMU)](https://publications.ri.cmu.edu/storage/publications/pub_files/2012/10/SkoniecznyIROS2012.pdf)
- Grousers that engage soil *ahead of* the wheel matter far more than those under it. A wheel with grousers only at the front achieved more than 80% of the improvement of a full-grouser wheel — [ISTVS publication news](https://www.istvs.org/publication-news/grousers-improve-drawbar-pull-by-reducing-resistance-and-generating-thrust-at-the-front-of-a-wheel)
- Skid-steer power models generally assume hard ground and break down on loose soil, where turning costs extra power through bulldozing and wheel sinkage (Clearpath Husky A200 tests) — [Fiset, Effati, Skonieczny 2019 (EasyChair)](https://easychair.org/publications/preprint/MsZP)
- One Southampton group proposes failure-envelope soil mechanics instead of Bekker to estimate sinkage and motion resistance for *small wheeled robots* — [Univ. Southampton](https://eprints.soton.ac.uk/471683)

**Agricultural traction models (Brixius)**
- Brixius's equations give gross traction ratio (GTR) and motion resistance ratio (MRR) as functions of a mobility number Bn and slip s, with NTR = GTR − MRR. They were fitted to about 2,500 field tests covering 121 soil-tyre combinations. Bn depends on cone index, tyre width, diameter, section height, deflection and load. Tractive efficiency of a properly set up ag tyre peaks near NTR ≈ 0.40 — [Virginia Tech traction chapter](https://vtechworks.lib.vt.edu/bitstream/handle/10919/93254/Traction.pdf); [core.ac.uk](https://core.ac.uk/download/153797635.pdf)

**Soil surface: clods and roughness**
- A seedbed's random roughness comes from its cloddy structure. Clod size is measured by sieving, or by image and elevation (DEM, stereo-photogrammetry) analysis — [HAL, clod identification](https://hal.archives-ouvertes.fr/hal-00660386); [Comparison of methods for cloddiness (Univ. Padova)](https://research.unipd.it/retrieve/e14fb267-6088-3de1-e053-1705fe0ac030/Comparison%20of%20methods%20for%20determining%20cloddiness%20in%20seedbed%20preparation.pdf)
- One contour-based clod detection method could not resolve clods smaller than about 7 mm diameter — [AGRIS record](https://agris.fao.org/search/ar/records/65df01457c7033e84bec6c38)
- Soil moisture, texture, tillage implement and soil density largely determine clod size and frequency. USDA wind-erosion work uses 0.84 mm (and in some comparisons 6.4 mm) sieve thresholds — [USDA-ARS](https://infosys.ars.usda.gov/WindErosion/publications/Andrew_pdf/646.pdf)
- Random roughness is usually summarised as the standard deviation of surface heights, an index of tillage fineness going back to Kuipers (1957) — [USDA-ARS](https://infosys.ars.usda.gov/WindErosion/publications/Andrew_pdf/680.pdf)

**Reference small field robots**
- **TerraSentia** (under-canopy phenotyping robot): 13 in (33 cm) wide, about the same height, 20.5 in (52 cm) long, four BLDC hub motors — [UST Magazine](https://ust-media.com/ust-magazine/UST024/76). It weighs "less than 30 pounds" (about 13.6 kg) — [Farm Progress](https://www.farmprogress.com/technology/autonomous-compact-robots-hold-potential-for-growers); [HPJ](https://hpj.com/2019/10/28/bots-on-the-ground-2/)
- **Tertill** (garden weeder): 210 x 210 x 121 mm, about 1.1 kg (2.4-2.5 lb), four extreme-camber wheels — [agtecher](https://agtecher.com/product/tertill-robot); [Gearbrain](https://www.gearbrain.com/devices/7FB41792BB6D11EC8E8341B1B269F696.html). Reviewers say the wheels cope with uneven garden terrain. Tertill will not cut weeds taller than its wheel level and needs a physical border, since it does no mapping — [Modern Castle](https://moderncastle.com/tertill-review/); [Pro Tool Reviews](https://www.protoolreviews.com/tertill-solar-powered-robotic-garden-weeder/)

### Inferences
**BACKGROUND (unverified in session): classical equations, for orientation only**
- Pressure-sinkage: p = (kc/b + kφ)·zⁿ.
- Rigid-wheel static sinkage: z₀ = [3W / ((3−n)(kc/b + kφ)√D)]^(2/(2n+1)).
- Compaction resistance: Rc = b(kc/b + kφ)·z^(n+1)/(n+1).
- Janosi-Hanamoto shear: τ = (c + σ tanφ)(1 − e^(−j/K)).
- Maximum thrust: H_max = A·c + W·tanφ.
- Drawbar pull: DP = H − ΣR (compaction, bulldozing, internal and other resistances).
- My recollection is that the Meirion-Griffith and Spenko model has the form σ = k̂·zⁿ/D^m, adding a diameter exponent m. This is not confirmed; check it against the 2011 J. Terramech paper.

**DERIVED (classical Bekker for this rover)**
- Inputs: soil values from Wong's textbook, recalled and not verified this session.
  - Dry sand: n = 1.1, kc = 0.99 kN/m^(n+1), kφ = 1528 kN/m^(n+2), c = 1.04 kPa, φ = 28°.
  - LETE sandy loam: n = 0.7, kc = 5.27, kφ = 1515, c = 1.72 kPa, φ = 29°.
  - Thai clayey soil at 38% moisture: n = 0.5, kc = 13.2, kφ = 692, c = 4.14 kPa, φ = 13°.
- Result: a 3 kg rover on four 12 x 4 cm wheels (7.4 N per wheel) is predicted to sink only about 1.2 mm (dry sand), 0.1 mm (sandy loam) or 0.02 mm (clay). Compaction resistance comes out at Rc/W ≈ 0.0005-0.003.
- Across 1.5-5 kg and 8-16 cm wheels the results stay in the same range: 0.01-1.9 mm sinkage, Rc/W below 0.004.
- These figures are implausibly low for freshly tilled soil. That fits the published finding that classical Bekker under-predicts small-wheel behaviour. Do not use Bekker sinkage to size this rover; use field tests or the Meirion-Griffith model.

**DERIVED (Mohr-Coulomb thrust ceiling)**
- Inputs: four contact patches of 4 cm x 3 cm (48 cm² in total) and the soil values above.
- Result: H_max ≈ 20-27 N for a 3 kg rover, i.e. 0.70-0.91 of weight. For 5 kg it is 31-35 N (0.63-0.72 W); for 1.5 kg, 13-23 N (0.87-1.6 W).
- On light robots the cohesion term A·c is a large share of thrust. On loose, dry, freshly tilled soil c ≈ 0, so thrust falls to about W·tanφ ≈ 0.5-0.6 W, before any motion resistance is subtracted.

**DERIVED (Brixius extrapolation, out of its fitted range)**
- Inputs: the standard Brixius coefficients (BACKGROUND): GTR = 0.88(1−e^(−0.1Bn))(1−e^(−7.5s)) + 0.04 and MRR = 1/Bn + 0.04 + 0.5s/√Bn, for a rigid 12 x 4 cm wheel at 7.4 N.
- Result: Bn ≈ 33-260 for cone index 100-800 kPa, which gives NTR ≈ 0.41-0.46 at 10% slip and 0.61-0.67 at 20% slip.
- These are optimistic. Brixius was fitted to pneumatic tractor tyres, and very light rigid wheels sit far outside its data. The model does not include clods, residue or c ≈ 0 surface layers.

**DERIVED (design figures for drawbar pull)**
- Assumption: net traction NTR ≈ 0.2 (wet or loose), 0.35 (typical) and 0.5 (firm, moist, good grousers).

| Mass | Weight | DP at 0.2 W | DP at 0.35 W | DP at 0.5 W |
|---|---|---|---|---|
| 1 kg | 9.8 N | 2.0 N | 3.4 N | 4.9 N |
| 2 kg | 19.6 N | 3.9 N | 6.9 N | 9.8 N |
| 3 kg | 29.4 N | 5.9 N | 10.3 N | 14.7 N |
| 5 kg | 49 N | 9.8 N | 17 N | 24.5 N |

  The available pull is *single-digit to low-tens of newtons*.

**DERIVED (ground pressure)**
- A 3 kg rover on four 4 x 3 cm patches presses at about 6 kPa.
- Two 4 x 18 cm tracks would give about 2 kPa.
- Either way, compaction is negligible. The risk is not sinkage under static load but slip-sinkage, bulldozing of loose soil and clods, and sticky clay building up on the wheels.

**DERIVED (wheel size heuristics)**
- For a single driven wheel meeting a step with friction μ, the climbable height is h ≈ r(1 − cos(arctan μ)). That is only about 0.14r at μ = 0.6 and 0.29r at μ = 1.0.
- All-wheel drive with the rear axle pushing raises this substantially, but a robust rule is wheel diameter ≥ 2-3 x the largest obstacle height.
- For clods of 3-5 cm, ridged drip tape of about 1.5-2 cm and residue, that means D ≈ 10-15 cm.
- In a 20 cm long chassis this allows either four wheels of about 10 cm with a wheelbase of about 10 cm, or two large drive wheels plus a skid or caster.
- Ground clearance of at least 5 cm (ideally 6-8 cm) is needed to avoid high-centering on clods and the crowns of beds.

### Gaps
- I could not open the full Meirion-Griffith and Spenko equations, or the DTIC, NSF and McGill full texts. No measured drawbar-pull-versus-slip curves for 1-5 kg rovers on *agricultural* soil were obtained. Most small-wheel data is on sand or regolith simulants.
- No measured rolling-resistance coefficients for small robot wheels on tilled soil, grass or mulch were found. The 0.1-0.3 MRR is a design assumption.
- No published typical clod-size distribution (mm) for a cultivated vegetable inter-row was found. The sources describe how to measure it but give no representative values. Typical heights for drip tape, furrows and bed crowns were not found either.
- Behaviour on plastic mulch, straw mulch, crusted soil and wet clay (wheel clogging) was not quantified in any source retrieved.

---

## 2. Wheels vs tracks vs legs vs whegs at shoebox scale; ground clearance; failure modes

### Takeaway
Every small ag robot I found with field deployment uses wheels:
- Tertill: four cambered wheels, 1.1 kg.
- TerraSentia: four BLDC hub wheels, 33 cm wide, less than 13.6 kg.

Even TerraSentia, more than twice as wide and roughly 3-10x heavier than the target rover, completes only about 90% of autonomous row segments and is not intervention-free. That says mobility and navigation reliability in real rows is the binding constraint, not peak traction. I found no field evidence for legs or whegs on agricultural soil at this scale.

### Cited Findings
- TerraSentia: 13 in wide, 20.5 in long, about 13 in high, four brushless hub motors. Its width lets it fit between corn and sorghum rows — [UST Magazine](https://ust-media.com/ust-magazine/UST024/76)
- TerraSentia weighs under 30 lb and runs about 4 h (2019 source) — [Farm Progress](https://www.farmprogress.com/technology/autonomous-compact-robots-hold-potential-for-growers). Another source claims about 8.5 h per charge — [HPJ "Bots on the ground"](https://hpj.com/2019/10/28/bots-on-the-ground-2/). The two runtime figures conflict.
- TerraSentia under-canopy navigation using a single 2D LiDAR without GNSS logged more than 6 km of autonomous runs in cluttered real fields (JFR 2019) — [JFR preprint (Illinois)](https://terra-mepp.illinois.edu/sites/terra-mepp.illinois.edu/files/2021-06/JFR_under_canopy_lidar_preprint.pdf); [dblp](https://dblp.uni-trier.de/rec/journals/jfr/HigutiVMBC19.html)
- Higuti's 2021 thesis reports 72 km of autonomous operation by multiple robots in corn and sorghum from 2017-2020. In a continuous lane with at least one side row, robots finished the intended autonomous segment 89.76% of the time. The approach "does not provide a fully intervention-free navigation within crop rows" — [USP thesis](https://www.teses.usp.br/teses/disponiveis/18/18149/tde-31052021-194235/publico/2021_higuti_thesis_perception_lidar.pdf)
- An image-based TerraSentia navigation system ran without interventions in fields "without significant gaps in the crop rows" — [Illinois Experts](https://experts.illinois.edu/en/publications/under-canopy-navigation-for-an-agricultural-rover-based-on-image-/)
- CropFollow++ (keypoint-based under-canopy navigation) was deployed on multiple cover-crop planting robots for 25 km in total — [arXiv 2404.17718](https://arxiv.org/abs/2404.17718v1)
- Tertill uses four extreme-camber wheels with a spinning nylon string trimmer underneath. It cannot handle weeds taller than wheel level and needs physical borders — [Pro Tool Reviews](https://www.protoolreviews.com/tertill-solar-powered-robotic-garden-weeder/); [Modern Castle](https://moderncastle.com/tertill-review/)
- Grousers are the main tool for raising traction on loose soil, and front-engaging grousers give most of the benefit — [CMU](https://publications.ri.cmu.edu/storage/publications/pub_files/2012/10/SkoniecznyIROS2012.pdf); [ISTVS](https://www.istvs.org/publication-news/grousers-improve-drawbar-pull-by-reducing-resistance-and-generating-thrust-at-the-front-of-a-wheel)
- A small testbed rover on loose soil compared "push-pull" (inchworm) locomotion with ordinary rolling — [ISTVS 2023](https://2023.istvs.org/submissions/papers/1453). Berkeley work on push-pull locomotion notes that mass-scaled tests on Earth cannot fully reproduce soil looseness under reduced gravity — [eScholarship](https://escholarship.org/content/qt1zp5c78w/qt1zp5c78w.pdf)
- Turning on loose soil adds bulldozing and sinkage losses that hard-ground skid-steer models leave out — [EasyChair, Fiset et al.](https://easychair.org/publications/preprint/MsZP)

### Inferences
- **Wheels (4WD skid-steer, 10-15 cm, narrow with grousers)** are the default choice:
  - simplest, best energy efficiency, proven on Tertill and TerraSentia;
  - in-place turning fits the narrow inter-row (see section 7).
  - Failure modes:
    - high-centering on clods or bed crowns when clearance is under about 5 cm;
    - wet clay packing into the grousers and building up on the wheels (with narrow track and short wheelbase there is no room for scrapers);
    - tangling in residue, plastic mulch edges or drip tape;
    - slip-sinkage digging in during skid-steer turns on loose soil;
    - dropping a wheel into a furrow or irrigation channel.
- **Tracks** on a 15 x 20 cm body would lower ground pressure (DERIVED about 2 kPa) and bridge furrows and clods better. The costs:
  - higher internal friction;
  - poor skid-steer turning on soil, with high bulldozing loss, which matters given the many turns needed in narrow rows and at headlands;
  - mud and stones packing into idlers;
  - narrow tracks on a narrow body are prone to de-tracking.
- **Whegs/legs** handle step-like obstacles (clods, drip tape, residue) better per unit size. They give uneven speed and vibration, which hurts camera-based weed detection and tool accuracy. They are also weak at generating steady drawbar pull for a soil-engaging tool, and much more complex. No ag field evidence was found.
- Ground clearance of at least 5-8 cm and wheel diameter of 10-15 cm are DERIVED requirements (section 1 heuristic). In a 10-25 cm height envelope, that leaves roughly 5-15 cm of vertical space for the body, battery and electronics.

### Gaps
- I found no published field mobility statistics for robots of 1-5 kg, 15 cm width, on tilled vegetable soil: stuck events per km, slip, or wet-soil failure rates. TerraSentia statistics are for a robot more than twice as wide and heavier, mostly in corn and sorghum.
- I found no data on wheg or legged ag robots at this scale, and no Tertill teardown (motor, gearbox, battery capacity, speed). A targeted search for a Tertill teardown returned nothing relevant.

---

## 3. Tool forces: draft and vertical force for shallow mechanical weeding versus drawbar pull from a 1-5 kg robot

### Takeaway
Tractor-scale hoeing data scales down poorly, but every estimate points the same way:
- A *full-width* soil-engaging blade (about 15 cm wide at 2 cm depth in moist, crusted or cohesive soil) needs roughly 20-150 N of draft. A 1-5 kg robot can supply only about 2-25 N.
- Feasible tools are those that need little reaction force:
  - a single narrow tine or small blade, about 1-2 cm wide and 1-2 cm deep, in loose soil (DERIVED about 0.1-10 N);
  - rotary or string tools, whose reaction is torque rather than drawbar pull (Tertill proves a string trimmer works on a 1.1 kg robot);
  - stamping or punching, which is limited by robot weight: at most about 10-50 N downward with no extra ballast;
  - stop-and-treat tools that use the robot's weight while stationary.

### Cited Findings
- **ASABE D497 draft model:** D = F_i [A + B·S + C·S²]·W·T. F_i is the soil-texture multiplier (D497 Table 3) — [CableTract arXiv 2604.09938](https://arxiv.org/pdf/2604.09938)
- **D497 S-tine row-crop cultivator:** A = 140, B = 7.0, C = 0 per row (SI units). Texture multipliers are F1 = 1.0 (fine), F2 = 0.85 (medium), F3 = 0.65 (coarse), with an expected range of ±15%. These were read from a table that was hard to parse, so the column assignment should be confirmed — [Arizona extension table (search snippet; host unreachable)](https://ag.arizona.edu/crop/equipment/agmgttable1.pdf). The standard was reaffirmed in 2020 — [ANSI webstore](https://webstore.ansi.org/standards/asabe/asaed497mar2011r2020)
- Field checks of D497.7 draft estimates were inconclusive. Some implements were within 20% of the ASABE estimate — [NCSU thesis](https://repository.lib.ncsu.edu/server/api/core/bitstreams/65ee8f95-d623-4fa1-9b21-a50e4323db4a/content)
- **Hoe share soil-bin study** (Znova et al. 2018, Acta Agric. Scand. B 68(2):139-148): depths 30, 50 and 70 mm at speeds 0.84, 1.67 and 2.31 m/s. Going from 30 to 50 mm raised longitudinal force Fx by 63%; going from 50 to 70 mm raised it a further 71%. Greater speed and depth generally increased draught and soil movement — [informahealthcare](https://informahealthcare.com/doi/full/10.1080/09064710.2017.1367837); [DOAJ](https://doaj.org/article/562e9162ed6f4eb58a5c209a7bf666d7)
- An Iowa State thesis on automated tine weeding measured horizontal draft and torque on a 6.35 mm diameter tine at 70 mm working depth — [Iowa State DR](https://dr.lib.iastate.edu/etd/18161)
- Push force for manual push-pull weeders and wheel hoes has been modelled as a function of blade width, weeding depth, friction and cutting resistance. The push stroke cuts and the pull stroke repositions. No measured values appeared in the snippet — [CIGR Journal](https://cigrjournal.org/index.php/Ejounral/article/download/4966/2793/22274)
- Tertill cuts weeds with a small spinning nylon string trimmer under the chassis, on a 1.1 kg robot — [Pro Tool Reviews](https://www.protoolreviews.com/tertill-solar-powered-robotic-garden-weeder/); [agtecher](https://agtecher.com/product/tertill-robot)

### Inferences
**DERIVED (D497 scaled down)**
- At 0.36-1.8 km/h (0.1-0.5 m/s) the S-tine cultivator gives D ≈ 93-153 N *per row unit*.
- Such a unit normally carries several tines working several centimetres deep, so a single S-tine at normal depth is plausibly about 25-50 N.
- From Znova's depth sensitivities (Fx ∝ depth^0.96 between 30 and 50 mm, ∝ depth^1.59 between 50 and 70 mm), cutting depth from about 50 mm to 20 mm divides force by roughly 2.5-4.
- So one tine at 2 cm is plausibly about 6-20 N. This is a highly uncertain chain of extrapolation.

**DERIVED (McKyes-type earthmoving estimate)**
- Formula (BACKGROUND, unverified in session): F ≈ (γ·d²·Nγ + c·d·Nc)·w, with γ = 15 kN/m³, Nγ ≈ 2-5 and Nc ≈ 3-6. It ignores adhesion, surcharge, root cutting and speed effects.

| Tool | Loose soil (c ≈ 0) | Moderately cohesive (c ≈ 2 kPa) | Moist or crusted (c ≈ 8 kPa) |
|---|---|---|---|
| 2 cm wide, 2 cm deep | 0.2-0.6 N | 2.6-5.4 N | 10-20 N |
| 5 cm wide, 2 cm deep | 0.6-1.5 N | 7-14 N | 25-50 N |
| 15 cm wide, 2 cm deep | 2-5 N | 20-40 N | 74-150 N |
| 15 cm wide, 1 cm deep | not computed | 9-19 N | 36-73 N |

- Conclusion: a full 15 cm blade is feasible only on loose, dry, freshly tilled soil. That is exactly when weeds matter least, because the soil was just disturbed. On crusted or moist cohesive soil it is beyond a robot of 5 kg or less.

**Matching tools to available pull** (section 1: about 6-15 N for 3 kg, 10-25 N for 5 kg):
- A **single narrow tine or finger, about 1-2 cm wide and 1-2 cm deep**: feasible in most conditions, marginal on crust.
- **Rotary or string tools**: the reaction is torque about a vertical axis, plus a small lateral or drag force. That suits a light robot, and the rotary motor's own power (likely 5-20 W, not sourced) becomes the energy cost.
- **Stamping, punching or pressing** (vertical): the reaction must come from robot weight, so peak downforce is at most about mg = 10-50 N, minus the force needed to keep wheels gripping. A 3 kg robot can deliver at most about 20-25 N per stamp unless it uses inertia (a spring or hammer) or anchors itself.
- **Pulling or uprooting**: needs vertical force comparable to stamping, so it is weight-limited in the same way.
- The robot can raise usable reaction force by:
  - stopping and bracing (static friction, typically above sliding traction);
  - using impact or inertia (a stored spring);
  - adding ballast, at the cost of energy and compaction;
  - working only on shallow, freshly loosened soil.
- Lateral tool forces on a 15 cm track robot also produce yaw and steering disturbance. With about 3-15 N of grip per side, lateral forces above a few newtons will deflect the heading.

### Gaps
- No measured draft values (N) were found for hoe blades or tines at 1-2 cm depth, or at robot speeds of 0.1-0.5 m/s. The Znova paper gives only percentage changes in the abstract, and absolute values sit behind a paywall.
- No measured forces were found for string-trimmer or rotary weeders, stamping tools (needed punch force per weed), or the force to uproot small weeds. No full ASABE D497 table could be read.
- The McKyes N-factors and the Wong soil values are BACKGROUND and were not checked against a source this session.

---

## 4. Energy budget: driving, compute, cameras and tool; battery runtime; solar under canopy; docking and swap

### Takeaway
Total electrical draw is about 8-25 W:
- driving: about 1-10 W at 0.1-0.5 m/s including tool draft (DERIVED);
- compute: Pi 5 about 3-9 W, Orin Nano about 5-15 W (or more);
- cameras: about 1-4 W (not sourced);
- tool: about 0-20 W.

A 100 Wh pack is about 6 x 21700 cells, about 0.4 kg of cells and a layer about 1 cm thick over the 15 x 20 cm footprint. It gives about 4-12 h. A 15 x 20 cm solar panel collects about 30 Wh/day in full summer sun, but only about 3-10 Wh/day under a closing pepper canopy. Solar alone cannot sustain continuous operation; it supports a Tertill-style duty cycle of about 1-2 h/day at most. Docking or battery swap is needed for multi-hour daily work.

### Cited Findings
**Compute power**
- Raspberry Pi 5:
  - about 3.0 W idle (headless, Wi-Fi only), about 3.6 W with Ethernet, HDMI and USB attached;
  - about 8.8 W with CPU stress on all 4 cores;
  - up to about 16 W at extreme load (CPU, USB SSD and 4K video) — [raspberry.tips 2026 comparison](https://raspberry.tips/en/faq/raspberry-pi-power-consumption-update-2026-all-models-compared)
  - another review measured 4.84 W idle (8 GB) and 11.6 W full load — [LinuxLinks](https://www.linuxlinks.com/raspberry-pi5-mini-pc-power-consumption/)
  - wall-meter idle readings differed by up to 1 W between power supplies — [Jeff Geerling](https://www.jeffgeerling.com/blog/2023/answering-some-questions-about-raspberry-pi-5/)
  - Attribution caveat: these per-URL figures were taken from an aggregated search summary.
- Jetson Orin Nano:
  - reference modes are 7 W and 15 W; JetPack 6.2 added a 25 W mode and an uncapped "MAXN SUPER" mode — [NVIDIA developer blog](https://developer.nvidia.com/blog/nvidia-jetpack-6-2-brings-super-mode-to-nvidia-jetson-orin-nano-and-jetson-orin-nx-modules)
  - according to NVIDIA forum replies, the 8 GB module (p3767-0003) is expected to have only 7 W and 15 W modes unless the devkit-super configuration is used, and some users report the 25 W mode missing after JetPack 7.2 — [NVIDIA forums](https://forums.developer.nvidia.com/t/jetson-orin-nano-missing-power-modes/326506); [NVIDIA forums (JetPack 7.2)](https://forums.developer.nvidia.com/t/25w-and-maxn-super-not-seen-in-jetpack-7-2/372627)
  - a third-party benchmark (not official) gives 4.5-5.5 W idle, 8-12 W during typical inference, and about 13-16.5 W under load in 15 W mode — [edgeaistack](https://edgeaistack-site.pages.dev/blog/jetson-orin-nano-power-consumption/)

**Battery cells**
- A Volklec 21700 5 Ah NMC cell is rated at 247 Wh/kg and 689 Wh/L, and measured at 264 Wh/kg and 700 Wh/L — [Batemo](https://www.batemo.com/?p=113656)
- The Amprius SA17 silicon-anode cell is rated at 300 Wh/kg and 830 Wh/L — [Amprius datasheet (KULR)](https://kulr.ai/wp-content/uploads/2025/08/Amprius-SA-17-Datasheet.pdf)
- Typical 21700 cells: NCM about 250-280 Wh/kg, LFP about 160-180 Wh/kg — [Zbotic](https://zbotic.in/21700-vs-18650-which-lithium-cell-to-choose/)
- Pack-level density is lower because of dead volume (thermal parts, spacers, interconnects) — [PatSnap](https://www.patsnap.com/resources/blog/rd-blog/cylindrical-vs-prismatic-battery-energy-density-patsnap-eureka/)

**Solar**
- One peak sun hour = 1 kWh/m², so daily peak sun hours equal daily insolation in kWh/m². A summer example is about 6 h/day — [Footprint Hero](https://footprinthero.com/peak-sun-ho); [SurgePV glossary](https://www.surgepv.com/glossary/peak-sun-hours)
- Quick sizing derate factors are 0.75-0.85. Summer insolation is about 3-4x winter at mid-latitudes — [1KOMMA5](https://1komma5.com/au/solar/the-role-of-peak-sun-hours-in-estimating-your-solar-panel-output/); [Renogy](https://www.renogy.com/blogs/general-solar/what-are-the-average-peak-sun-hours-by-state)
- Canopy light interception is measured as 1 − (PAR below / PAR above) at solar noon on clear days, or modelled with Beer's law from LAI and an extinction coefficient — [MSU Solar Radiation notes](https://www.spar.msstate.edu/class/epp/Solar%20Radiation.pdf); [HortScience (Campillo et al. 2008)](https://journals.ashs.org/downloadpdf/view/journals/hortsci/43/6/article-p1780.pdf)
- Sweet pepper canopies are spatially heterogeneous, so 3D ray-tracing captures shading better than exponential models — [Kim et al. 2016, Front. Plant Sci. (PMC)](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5016622/)

**Solar-powered robot benchmarks**
- Tertill: solar panel plus Li-ion battery, micro-USB backup charging, "daily operation time 1-2 hours (in 2-5 minute intervals)". The panel needs occasional cleaning — [agtecher](https://agtecher.com/product/tertill-robot); [Pro Tool Reviews](https://www.protoolreviews.com/tertill-solar-powered-robotic-garden-weeder/)
- Aigen Element (original): a 205 W solar panel, up to 14 h/day of operation — [PR Newswire](https://prnewswire.com/news-releases/aigen-unveils-the-worlds-first-ai-driven-solar-powered-agricultural-robotics-service-301868463.html). Gen2 adds 50% more power from upgraded panels and battery storage — [Sugar Producer](https://www.sugarproducer.com/2025/04/aigen-launches-next-generation-element)

### Inferences
**DERIVED (driving power)**
- Formula: P_elec = (MRR·m·g + F_tool)·v / η_drive, with η_drive = 0.5 assumed for small brushed gearmotors plus wheel slip.

| Mass | Speed | MRR | Tool draft | Electrical power |
|---|---|---|---|---|
| 3 kg | 0.3 m/s | 0.1 | 0 | 1.8 W |
| 3 kg | 0.3 m/s | 0.25 | 0 | 4.4 W |
| 3 kg | 0.3 m/s | 0.25 | 10 N | 10.4 W |
| 3 kg | 0.5 m/s | 0.25 | 10 N | 17.4 W |
| 5 kg | 0.5 m/s | 0.25 | 10 N | 22.3 W |
| 2 kg | 0.1 m/s | 0.1 | 0 | 0.4 W |

  Over the full range of 0.1-0.5 m/s, 2-5 kg and 0-10 N draft, driving needs about 0.4-22 W. Tool draft dominates once present.

**DERIVED (system totals; camera wattage is an assumption)**
- Minimum, Pi 5 configuration: Pi 5 at 4-6 W + 1-2 cameras at about 1-2 W + driving at 2-4 W + IMU/GPS/MCU at about 0.5-1 W ≈ 8-12 W.
- Heavier, Orin Nano configuration: 8-15 W + 2-4 W + 4-10 W + 1 W + a rotary tool duty-cycled at 2-10 W ≈ 17-40 W.

**DERIVED (runtime)**
- 50 Wh: about 2 h at 25 W, about 4 h at 12 W, about 6 h at 8 W.
- 100 Wh: about 4 h at 25 W, about 8 h at 12 W, about 12 h at 8 W.
- 150 Wh: about 6 h at 25 W, about 12.5 h at 12 W.
- Multiply by about 0.9 for usable depth of discharge and about 0.8-0.9 in cold or with ageing.

**DERIVED (pack size)**
- Each 21700 5 Ah NMC cell holds about 18 Wh and weighs about 70 g.
- 100 Wh ≈ 6 cells ≈ 0.40 kg of cells (about 0.67 kg as a pack at about 150 Wh/kg), about 143 cm³ of cells, about 290 cm³ packed.
- 150 Wh ≈ 8-9 cells, about 1.0 kg as a pack, about 430 cm³.
- 300 cm² of footprint gives about 300 cm³ per cm of height, so 100-150 Wh fits easily. For a robot under 3 kg, mass is the real constraint: a 150 Wh pack would be about 33% of total mass.

**DERIVED (solar)**
- Panel: 0.15 x 0.20 m = 0.03 m², 20-22% efficient, so about 6-6.6 W peak at 1000 W/m².
- Daily energy with 0.8 derate:
  - 6 kWh/m²/day, open sky: about 29-32 Wh/day;
  - same insolation under polytunnel film (assumed 85% transmission): about 25-27 Wh/day;
  - under canopy at 30% transmission: about 9 Wh/day;
  - under canopy at 10% transmission: about 3 Wh/day;
  - a dull day at 2 kWh/m²: about 10 Wh open sky, 1-3 Wh under canopy.
- Against a 10-15 W draw, open-sky summer solar gives about 2-3 h/day of operation. Under a closed pepper canopy it gives about 0.2-0.9 h/day. This matches Tertill's 1-2 h/day of short bursts on a far simpler robot, with no SBC and a garden left mostly unshaded.
- Solar becomes "meaningful" only early in the season, before canopy closure, or as trickle top-up during long idle periods.
- The canopy transmission fractions of 10-30% are assumptions, not pepper measurements.

**Charging and swap (engineering judgement, not sourced)**
- Pogo-pin or contact-plate docks at row ends or headlands, reached by row-following plus a visual fiducial.
- A swappable 50-100 Wh pack (for example a 4S/6S 21700 pack), which a worker swaps once or twice a day for a fleet.
- A mobile "mothership" or cart carrying charged packs.
- With about 13-25 km of inter-row per hectare (section 5), a robot may be up to about 100 m from the headland. Return trips at 0.3 m/s take about 5-6 min, a small overhead.

### Gaps
- No measured current draw was found for a small (1-5 kg) rover driving on soil, nor for small IP-rated gearmotors under load. The 0.5 drivetrain efficiency and the 0.1-0.25 MRR are assumptions.
- No pepper canopy PAR transmission fraction at the soil surface was found: sources give methods (Beer's law, ray-tracing) but no number. No site-specific PVGIS insolation values were retrieved.
- Camera power (for example Pi Camera Module 3, global-shutter cameras, USB cameras) was not sourced.
- Tertill battery capacity, motor power and speed were not found.

---

## 5. Coverage rate: m²/h, ha/day, passes per inter-row; comparison with commercial robots and with weeding frequency; robots per hectare

### Takeaway
A 15 cm tool at 0.1-0.5 m/s sweeps about 54-270 m²/h. Fields need repeated passes across the critical period, which for pepper is about 3-9 weeks after transplanting. So the two operating modes give very different fleet sizes:

| Mode | Field rate | ha/day (10 h) | Robots per ha (5-7 day cycle) |
|---|---|---|---|
| Full-width inter-row cultivation | ≈ 40-200 m²/h | 0.04-0.2 | about 1-5 |
| One pass per inter-row, targeted weed treatment within camera reach | ≈ 110-1,350 m²/h | 0.1-1.35 | about 0.1-1.3 (≈ 0.25-0.6 at 0.3 m/s) |

Commercial weeding robots claim about 1-8 ha/day each, roughly 10-100x one micro-rover. Tertill is rated for only about 9-19 m² of garden.

### Cited Findings
**Commercial robots**
- FarmDroid FD20: up to 6.5 ha/day at 950 m/h; other figures are 5-6.5 ha/day and "up to 6 ha/day"; about 20 ha per robot per season for sowing and weeding — [Future Farming](https://futurefarming.com/farmdroid-fd20-automatic-seeding-and-weeding); [FAO Agritech Observatory](https://agritechobservatory.review.fao.org/en/farmdroid-fd20); [Cornell FD20 factsheet](https://hemp.cals.cornell.edu/files/2023/09/Factsheet-FD20-USA_web.pdf)
- Naïo Oz: about 1 ha/day, "might be insufficient for very large-scale operations"; intended for market gardens of up to about 10 ha — [agtecher](https://agtecher.com/product/naio-oz); [Modern Farmer](https://modernfarmer.com/2020/02/futuristic-farming-has-arrived-with-weeding-robots/)
- Odd.Bot Maverick: about 1-12 ha/day per module depending on configuration and conditions; up to 2 weeds/s — [AgriExpo](https://www.agriexpo.online/prod/oddbot-bv/product-192286-190300.html)
- Aigen Element:
  - Gen2 is said to cover up to 20 acres (about 8 ha) autonomously (period not stated clearly), selling for $50,000 per unit plus a $20,000 per-fleet fee — [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops)
  - the earlier version covered up to 3 acres/day (about 1.2 ha) — [TechCrunch](https://techcrunch.com/?p=2258249); about 30 acres/week — [Fast Company](https://www.fastcompany.com/91033135/aigen-most-innovative-companies-2024)
  - drives at about 2 mph (about 0.9 m/s) — [Saur Energy](https://www.saurenergy.com/solar-energy-news/us-agri-startup-aigen-brings-solar-powered-robotics-to-farm-cleanup)
- Tertill: recommended coverage of 100 sq ft (about 9.3 m²) per robot according to Reviewed.com testing — [Reviewed](https://reviewed.com/smarthome/features/tertill-review-its-like-a-roomba-for-garden-beds); "up to 200 sq ft" (about 18.6 m²) in a product listing — [agtecher](https://agtecher.com/product/tertill-robot). The two conflict, and no manufacturer figure was found.
- TerraSentia runs about 4 h per charge (2019) — [Farm Progress](https://www.farmprogress.com/technology/autonomous-compact-robots-hold-potential-for-growers); about 8.5 h according to another source — [HPJ](https://hpj.com/2019/10/28/bots-on-the-ground-2/)

**Critical period of weed control (pepper)**
- Transplanting to first bloom, about 3 weeks after transplanting (WAT), is the most critical period — [SARE Bell Pepper IPM Weed Field Guide](https://projects.sare.org/media/pdf/B/e/l/Bell-Pepper-IPM-Weed-Field-Guide.pdf)
- A California study found peppers need at least 8 weeks weed-free for full yield and tolerate at most about 3 weeks of early weed competition — [California Agriculture](https://dx.doi.org/10.3733/ca.v045n01p11)
- The critical period ends at early fruit set, 5-7 WAT, for most troublesome weeds, but extends to about 9 WAT with purple nutsedge — [UF/IFAS IPM Chapter 6](https://ipm.ifas.ufl.edu/pdfs/Chapter6-Weed_Mgmt.pdf)
- Ohio State: keep bell peppers weed-free until at least fruit set — [Ohio State (archived)](https://webarchive.library.unt.edu/eot2008/20090116000726/http://ohioline.osu.edu/b672/pdf/Weed.pdf)
- An Egyptian trial estimated the critical weed-free period at 8.3 and 7.0 weeks from transplanting — [EKB journal](https://journals.ekb.eg/article_79106_b5a92890d47a8d529fc4a7f2176f56fa.pdf)
- For the 'Dahra' cultivar at 5% yield-loss tolerance, control from 11-89 days after transplanting — [Planta Daninha (DOAJ)](https://doaj.org/article/2679da1463d34004a90557382385cc5e)

### Inferences
**DERIVED (bare swath rate = v x w)**

| Speed | w = 10 cm | w = 15 cm | w = 20 cm |
|---|---|---|---|
| 0.1 m/s | 36 m²/h | 54 m²/h | 72 m²/h |
| 0.2 m/s | 72 m²/h | 108 m²/h | 144 m²/h |
| 0.3 m/s | 108 m²/h | 162 m²/h | 216 m²/h |
| 0.5 m/s | 180 m²/h | 270 m²/h | 360 m²/h |

**DERIVED (inter-row length per hectare = 10,000 m² / row spacing s)**
- 25 km at s = 0.4 m; 16.7 km at 0.6 m; 13.3 km at 0.75 m; 10 km at 1.0 m.
- One pass along every inter-row of 1 ha at 0.75 m spacing and 0.3 m/s is 12.3 h of pure driving, before headland turns.

**DERIVED (field capacity)**
- Formula: ha/h = v·s/n·e·0.36, where n = passes per inter-row and field efficiency e = 0.75 is assumed for turns, stops and docking.
- Full-width mode (w = 15 cm, n = ceil(s/w), i.e. 3 passes at s = 0.4 m, 4 at 0.6, 5 at 0.75, 7 at 1.0):
  - 0.1 m/s: about 36-41 m²/h, 0.04 ha per 10 h day;
  - 0.3 m/s: about 108-122 m²/h, about 0.11-0.12 ha/day;
  - 0.5 m/s: about 180-202 m²/h, about 0.18-0.20 ha/day.
- Single-pass targeted mode (n = 1):
  - 0.3 m/s: about 324 m²/h at s = 0.4 m, 607 m²/h at s = 0.75 m, 810 m²/h at s = 1.0 m, i.e. 0.32-0.81 ha per 10 h day;
  - 0.5 m/s: up to 1,350 m²/h (1.35 ha/day) at s = 1.0 m.
  - In this mode stop-and-treat time per weed, not driving speed, will likely limit throughput at high weed density (not quantified).

**DERIVED (robots per hectare, 10 h/day)**
- Full-width mode:
  - 0.3 m/s: about 1.2-1.3 robots/ha on a 7-day cycle, about 1.65-1.85 on a 5-day cycle;
  - 0.1 m/s: about 3.5-5.6 robots/ha.
- Targeted mode at 0.3 m/s: about 0.18-0.44 robots/ha (7-day cycle) and 0.25-0.62 (5-day cycle).
- Running 20 h/day (night operation with battery swaps) halves these.
- For the pepper critical period of about 3-9 WAT, a 5-10 day revisit means about 4-12 passes per season.

**DERIVED (energy per hectare)**
- At 12 W and 0.3 m/s with s = 0.75 m: one pass per inter-row costs about 150 Wh/ha; full-width mode (5 passes) about 740 Wh/ha.
- A 100 Wh robot therefore treats about 0.6 ha per charge in targeted mode, or about 0.13 ha in full-width mode.

**Comparison**
- A FarmDroid (about 6 ha/day) does the work of roughly 30-50 full-width micro-rovers, or about 8-20 targeted-mode micro-rovers.
- The micro-rover only makes sense if:
  - unit cost is very low (hundreds of dollars, not thousands);
  - it works under canopy where large robots cannot;
  - it treats targeted weeds near plants rather than cultivating the full inter-row;
  - it runs long hours (night and day) with automated charging.

### Gaps
- No published row spacing for bell pepper in open field or polytunnel was retrieved, so the 0.4-1.0 m range is the brief's assumption.
- No data was found on weed-treatment time per weed for small robots, or on weed density per m² in pepper, which govern targeted-mode throughput.
- Aigen gen2's "20 acres" is not clearly per day. Odd.Bot's 1-12 ha/day is a manufacturer range. All commercial work rates are vendor claims.

---

## 6. Environmental robustness: dust, mud, irrigation water, dew, heat in polytunnels; IP ratings, motors and gearboxes; BOM cost

### Takeaway
I found little direct evidence on how small robots fare against dust, mud and dew. The sourced constraints are:
- heat: polytunnels can reach about 42 °C air temperature, and the Raspberry Pi throttles at 80-85 °C SoC temperature;
- price reference points: TerraSentia about $5k at launch to about $15k now (aggregator figure), Aigen $50k.

A shoebox rover must be sealed to about IP65-IP67, with sealed gearmotors and conformal-coated electronics, and must be able to shed heat while sealed. That is a non-trivial thermal-design problem for a 5-15 W computer in a 3-5 L sealed box sitting in direct sun or at 40 °C+.

### Cited Findings
- Optimum growth for warm-season tunnel crops (peppers, eggplant, tomatoes, cucumbers) is about 21-24 °C (70-75 °F). Growth slows and stress appears above about 29 °C (85 °F) — [UVM Extension "High and Hot"](https://www.uvm.edu/d10-files/documents/2026-08/High-and-Hot.pdf)
- A UK grower reports a polytunnel reaching 42 °C before vents or doors were opened. Pepper fruit set stops above about 26 °C day / 21 °C night (anecdotal) — [Allotment Garden diary](https://www.allotment-garden.org/garden-diary/5235/polytunnel-temperatures/)
- Shade cloth (30-50%) can lower high-tunnel air temperature by about 3-5 °C (6-9 °F). Misting cut temperature by about 5 °C in 10 minutes in one grower's tunnel — from a US extension source and a UK grower, per search summary; exact attribution is among [SARE high tunnel temperature management](https://projects.sare.org/media/pdf/9/2/5/925063high-tunnel-temperature-management.pdf) and [UMN Extension blog](https://blog-fruit-vegetable-ipm.extension.umn.edu/2024/04/prepare-your-high-tunnel-for-hot-summer.html), not confirmed
- All Raspberry Pi boards throttle at 80 °C SoC temperature and throttle further at the 85 °C maximum — [Raspberry Pi news](https://www.raspberrypi.com/news/heating-and-cooling-raspberry-pi-5/)
- The Pi 5 product brief warns it should be operated "in a well ventilated environment" and that a case should not be covered — [Pi 5 product brief (SparkFun mirror)](https://cdn.sparkfun.com/assets/9/9/0/d/e/raspberry-pi-5-product-brief.pdf)
- Compute Module 5 is rated for −20 °C to +85 °C operating temperature, a wider range suited to sealed enclosures — [CM5 product brief](https://pip-assets.raspberrypi.com/categories/944-raspberry-pi-compute-module-5/documents/RP-008181-DS-5-cm5-product-brief.pdf)
- Raspberry Pi has published a white paper on use-case-specific thermal performance of its SBCs — [Raspberry Pi whitepaper](https://pip-assets.raspberrypi.com/categories/685-app-notes-guides-whitepapers/documents/RP-010139-WP-1-Use-case-specific%20thermal%20performance%20of%20Raspberry%20Pi%20SBCs.pdf)
- Tertill's solar panel needs occasional cleaning: dust and soil on the panel reduce charging — [Pro Tool Reviews](https://www.protoolreviews.com/tertill-solar-powered-robotic-garden-weeder/)
- TerraSentia was priced at $4,999 for early adopters in 2017 — [CSL Illinois](https://csl.illinois.edu/news/teachable-ultra-compact-autonomous-phenotyping-robot-introduced-investors-market). An aggregator lists $15,000 currently, with a 5-year TCO estimate of $21,000, but it is low-reliability with placeholder text — [Robotomated](https://robotomated.com/explore/agricultural/earthsense-terrasentia)
- Aigen Element gen2 costs $50,000 per unit plus a $20,000 per-fleet fee — [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops)

### Inferences (engineering judgement, unsourced unless noted)
**Sealing**
- Drive units and electronics should be at least IP65 (jets of water, dust-tight), preferably IP67 (temporary immersion), to survive irrigation splash, standing water in furrows, dew, and hosing-down to remove mud.
- Use sealed or o-ringed gearmotors, or brushless outrunners with sealed bearings. Shaft lip seals on wheel axles are the main ingress point.
- Breathable membrane vents (ePTFE) prevent pumping of moist air during daily temperature cycles, which otherwise causes condensation.

**Thermal**
- In a sealed box at 40-45 °C ambient (polytunnel), with solar load on the lid, the internal air can exceed 55-60 °C. A Pi 5 under sustained load will likely throttle.
- Remedies:
  - conductive heat-spreading from the SoC to an aluminium chassis plate that faces the shaded soil side;
  - choosing CM5 (−20 to +85 °C rated) or the Orin Nano module on a carrier with a heat path;
  - duty-cycling inference;
  - operating at night or early morning in tunnels.
- Li-ion cells should not be charged above about 45 °C. This favours charging at docks in shade or at night (BACKGROUND, unverified in session).

**Mud and dust**
- Cameras under canopy need a lens window that resists soil splash from the tool and dew. A small hood, a hydrophobic coating, or placing the camera away from the tool help.
- Wheel scrapers, open-spoke wheels and widely spaced grousers reduce clay build-up.

**BOM cost (rough, unsourced estimate)**
- Pi 5 at $60-80 or Orin Nano Super devkit at about $250.
- 1-2 cameras at $25-60 each.
- 4 small IP-rated gearmotors with encoders at $15-60 each.
- 100 Wh 21700 pack with BMS at about $40-80.
- IMU/GPS at $20-200 (RTK modules more).
- Chassis, seals and wheels at $50-150.
- Tool motor and actuator at $20-80.
- Total about $300-900 per unit at hobby or low volume. This is far below TerraSentia or Aigen, and that cost gap is the main argument for a swarm.

### Gaps
- No sourced data was found on IP ratings, motor or gearbox choices, or failure rates of existing small ag robots (Tertill, TerraSentia) in mud, dust or wet conditions.
- No Jetson Orin Nano operating temperature spec was retrieved this session, and the Pi 5 ambient operating range was not confirmed.
- No sourced BOM for a small rugged ag rover was found. Tertill's retail price was not retrieved.
- No measured soil-surface temperature under pepper canopy was found.

---

## 7. Stability and geometry: 15 cm track on side slopes, ridges, furrows and raised beds; centre of gravity; turning in narrow inter-rows and at headlands

### Takeaway
Static stability is adequate if the centre of gravity (CG) is kept low:
- With a wheel track of about 11-13 cm and CG height of 6 cm, the static side-tip angle is about 43-47°.
- With CG at 10-12 cm it falls to about 25-33°.

The real risks are dynamic and come from geometry:
- one wheel dropping into a furrow, or riding up a bed shoulder or a clod: a 5 cm step across a 13 cm track gives about 21° of roll;
- high-centering;
- dragging the tool sideways.

The 15 x 20 cm body (25 cm diagonal) can spin in place in inter-rows of about 30 cm or more. However, skid-steer turns on loose soil cost extra energy and dig in.

### Cited Findings
- TerraSentia's 33 cm width is what lets it travel between corn and sorghum rows — [UST Magazine](https://ust-media.com/ust-magazine/UST024/76)
- Under-canopy navigation success depends on row continuity: about 90% segment completion overall and intervention-free only where crop rows lack significant gaps — [USP thesis](https://www.teses.usp.br/teses/disponiveis/18/18149/tde-31052021-194235/publico/2021_higuti_thesis_perception_lidar.pdf); [Illinois Experts](https://experts.illinois.edu/en/publications/under-canopy-navigation-for-an-agricultural-rover-based-on-image-/)
- Skid-steer turning on loose soil adds bulldozing and sinkage power that hard-ground models miss — [EasyChair, Fiset et al.](https://easychair.org/publications/preprint/MsZP)
- Tertill needs physical borders and does no mapping, so it is not designed for row and headland navigation — [Modern Castle](https://moderncastle.com/tertill-review/)

### Inferences
**DERIVED (static tip angle = arctan((track/2)/h_CG))**

| Track | h_CG 4 cm | 6 cm | 8 cm | 10 cm | 12 cm |
|---|---|---|---|---|---|
| 11 cm | 54° | 43° | 35° | 29° | 25° |
| 13 cm | 58° | 47° | 39° | 33° | 28° |
| 15 cm | 62° | 51° | 43° | 37° | 32° |

- Mounting the battery (0.4-1 kg, often the heaviest part) in the floor of the chassis is the main lever for keeping CG at 6 cm or lower.
- A camera mast reaching 20-25 cm should be light.

**DERIVED (roll from a one-side drop over a 13 cm track)**
- A 3 cm drop gives 13° of roll, 5 cm gives 21°, 8 cm gives 32°.
- Combined with a bed side slope of about 10-15° (an assumption), an 8 cm furrow edge could approach the tip limit for a CG of 10 cm or more.
- Driving centred on a flat furrow bottom, or straddling nothing, is preferable to riding along bed shoulders.

**DERIVED (pitch)**
- With a wheelbase of about 10-12 cm and CG at 6 cm, static pitch-over is at about arctan(5.5/6) ≈ 42°. Pitch is not limiting on agricultural slopes.
- The short wheelbase does make the body pitch sharply over clods. A 3 cm clod under one axle of a 10 cm wheelbase gives about 17° of pitch, which is a camera-stabilisation issue.

**DERIVED (turning)**
- A 15 x 20 cm rectangle needs a circle of about 25 cm diameter to spin in place. In 40-100 cm inter-rows a point turn is geometrically possible even with plants leaning in.
- At headlands, a robot can go row-to-row without a loop turn. One option is a 90° spin, a lateral move of one row spacing, and another 90° spin. Another is to reverse into the next row.
- Each point turn on loose soil scuffs and digs (energy plus slip-sinkage). Differential-drive with two large wheels plus a caster or skid, or 4WD with slight wheel camber like Tertill, reduces scrubbing compared with 4WD skid-steer on a short wheelbase.

**DERIVED (raised beds and plastic mulch)**
- If peppers are grown on plastic-mulched raised beds, the rover would drive in the furrow between beds. Its working zone (the bed edge and planting holes) would then sit above it, and the tool would have to reach up and sideways.
- Alternatively the rover drives on the mulch, where grip on plastic is poor when wet. The plant-hole weeds would then be within reach.
- Either choice should be settled before fixing the wheel geometry.

### Gaps
- No sourced bed or furrow dimensions were found for pepper (bed height, shoulder slope, furrow width), nor typical side slopes of vegetable fields.
- No measured tip-over or rollover incidents for small ag robots were found.
- I found no studies of headland-turn energy or time for small skid-steer robots on tilled soil. The extra-power evidence is for the larger Clearpath Husky on sand.
