# Weeding tool / actuator technologies for a tiny under-canopy rover (15 x 20 cm, 1-5 kg, 50-150 Wh, tool budget ~5-50 W) — state of the art, Nov 2026

Scope note for the report writer: these notes combine (a) material gathered by an earlier researcher who hit a usage limit (its searches and extracts are reused here; their original URLs are cited) and (b) about 19 new searches and fetches in this session. Several primary full texts could NOT be fetched because these hosts are outside the session's network allowlist: bioone.org, core-prod.cambridgecore.org, www.mdpi.com, pmc.ncbi.nlm.nih.gov, grdc.com.au, projects.sare.org. Many numbers below therefore come from search-engine extracts of abstracts or snippets, not from the full papers. Each such case is flagged. "DERIVED" marks my own arithmetic. "ASSUMPTION" marks an input I chose that has no source.

Reference budget used throughout (DERIVED): 50 / 100 / 150 Wh = 180 / 360 / 540 kJ. If ~50 % goes to the tool (ASSUMPTION; the rest drives locomotion and compute), the tool gets **90 / 180 / 270 kJ per charge**. Joules available per weed = tool energy / weeds treated. At 180 kJ that is 180 J/weed for 1,000 weeds and 18 J/weed for 10,000 weeds. Reference weed density used in the literature: 5 plants/m2 at the 2-leaf stage = 50,000 weeds/ha (Coleman et al. 2019).

---

## Q1. Energy per weed and per hectare across all methods (Coleman 2019 and others), and what a 50-150 Wh rover can afford

### Takeaway
When weeds are treated one at a time (site-specific), hoeing, electrocution, laser and herbicide all come out at roughly 15-20 MJ/ha (lasers up to ~250 MJ/ha), dominated by the ~14.4 MJ/ha cost of imaging. Broadcast thermal methods (flaming, infrared) need 1,000-4,300 MJ/ha, which a battery cannot supply. On a 100 Wh pack the realistic tool budget is about 10-200 J per weed. That fits stamping (≤40 J), small-seedling electrical pulses (~5-10 J), low-power diode laser on seedlings (~10-60 J), micro-dosing (≈0 J) and blind shallow soil disturbance. It rules out hot water, steam, microwave, UV-C and broadcast flaming.

### Cited Findings
- Coleman et al. 2019, *Weed Technology* 33(4):633-650, doi 10.1017/wet.2019.32 (open access) compared energy per hectare of alternative weed control methods — [Cambridge](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E); [BioOne](https://bioone.org/journals/Weed-Technology/volume-33/issue-4/wet.2019.32/Using-energy-requirements-to-compare-the-suitability-of-alternative-methods/10.1017/wet.2019.32.full)
- Site-specific figures as extracted from the paper: electrocution ~15-19 MJ/ha, laser pyrolysis ~15-249 MJ/ha, hoeing ~17 MJ/ha, herbicides ~15 MJ/ha. Site-specific treatment "resulted in similar energy requirements for current and alternative technologies." (Search extract; full tables not retrieved.) — [Cambridge](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E)
- Broadcast thermal: flaming 1,008-4,334 MJ/ha; infrared 2,000-3,887 MJ/ha; thermal methods need 100- to 1,000-fold more energy than tillage — [Cambridge abstract](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E); [AHRI blog](https://www.ahri.uwa.edu.au/?p=16169)
- For 2-leaf broadleaf weeds at 5 plants/m2, site-specific treatment cut energy use by 97 % (herbicides), 99 % (thermal) and 97 % (mechanical) versus broadcast — [Cambridge](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E)
- Image collection and processing for site-specific control was costed at 14.4 MJ/ha — [Grain Central](https://www.graincentral.com/?p=162322); [AHRI](https://www.ahri.uwa.edu.au/?p=16169)
- Thermal weed control with UV light "can require up to 100,000 MJ/ha" for some species (thermal weed-control review) — [Cambridge review](https://www.cambridge.org/core/product/65ADA897B679D840205CDD3843C13A33/core-reader)
- An energy evaluation criterion for autonomous sugar-beet electrical weeding: < 2 kJ per metre of a 50-cm-wide row (cited in an electrical-weeding review) — [Agronomy 2023 review](https://mdpi.com/2073-4395/13/11/2694)

### Inferences
- DERIVED, energy per weed at 5 weeds/m2 (50,000/ha): imaging alone is 14.4 MJ/ha ÷ 50,000 = **288 J/weed**. If the site-specific totals include imaging (likely, since they all sit near 15 MJ/ha, but not confirmed), the tool-only energy is roughly herbicide ~12 J, hoeing ~52 J, electrocution ~12-92 J and laser ~12-4,700 J per weed.
- DERIVED, imaging cost on this rover: a 5-15 W RPi or Jetson at 0.015 m2/s (15-cm swath at 0.1 m/s, ASSUMPTION) costs 333-1,000 J/m2 = **3.3-10 MJ/ha**. That is the same order as Coleman's 14.4 MJ/ha, so at low weed density the computer, not the actuator, may dominate energy use. At high density the actuator dominates.
- DERIVED, site-specific flaming: (1,008-4,334 MJ/ha × 1 %) ÷ 50,000 = **~200-870 J per weed** as fuel energy. That is tolerable as gas fuel but not as battery energy. Broadcast flaming at 100.8-433 kJ/m2 would empty a 100 Wh (360 kJ) pack after only ~0.8-3.6 m2.
- DERIVED, UV at up to 100,000 MJ/ha = 10 MJ/m2 = 1 kJ per cm2 of seedling. With UV-C emitters at a few % efficiency (ASSUMPTION), that is tens of kJ of electricity per weed, so UV-C is infeasible.
- DERIVED, ranking by J per weed of electrical (battery) energy, using later sections: micro-dosing (~0.01-0.1 J valve pulse, ASSUMPTION) << electrical pulse on broadleaf seedlings (~6-10 J) < diode laser on cotyledon-stage seedlings (~10-60 J) ≈ stamp (≤40 J) < electrical on grass seedlings (~250-400 J) ≈ thulium-dose laser (≥120-600 J) << microwave (~100-4,000 J) ≈ hot water (~700-3,500 J) << UV-C (≥10 kJ).
- DERIVED, weeds per 180 kJ tool budget: micro-dose >10^6 (limited by liquid, not energy); electric broadleaf ~18,000-30,000; blue-diode laser at 30 J ~6,000; stamp at 40 J ~4,500; electric grass ~450-700; hot water ~50-250.

### Gaps
- Coleman 2019 full tables (per-method J/plant inputs, whether imaging is included in each site-specific total) could not be fetched (bioone.org and core-prod.cambridgecore.org blocked). Bauer et al. and Fennimore et al. comparative energy papers were not reached.
- No measured locomotion power for 1-5 kg rovers on tilled soil was found. The 50 % tool share is an assumption.

---

## Q2. Mechanical soil-engaging (blind) tools: hoe and knife blades, sweeps, finger/torsion/spring tines, brush, rotary/spiked wheels, drag chains, rolling cultivators. Draft, white-thread principle, and whether a 1-5 kg robot can do it

### Takeaway
Small seedlings are most vulnerable from the white-thread stage to the first true leaves. Uprooting in dry soil kills best (36 % → 91 % mortality as soil gets drier); burial needs ~6 cm of soil to kill reliably. Draft rises steeply with depth (+63 % going from 30 to 50 mm), so a 1-2 cm-deep tool is the only realistic choice for a robot that can pull only ~3-30 N (DERIVED). Tertill, a 1.1 kg, ~21 cm robot, is the closest existing precedent: it relies on grouser wheels that disturb the surface plus a string trimmer. The evidence that its wheels suppress weeds is observational only.

### Cited Findings
- Seedlings are most sensitive to harrowing from the white-thread stage until their first true leaves emerge (Kurstjens & Kropff and Kurstjens & Perdok body of work, via search extracts) — [WUR edepot](https://edepot.wur.nl/16866); [AGRIS: Mechanisms of selective mechanical weed control by harrowing](https://agris.fao.org/search/en/records/669f6943a4dac527f930bb92)
- In laboratory harrowing, uprooting was the main lethal mechanism for weakly anchored seedlings. Soil covering killed comparatively few plants. Uprooted plants died at ~47-61 % over 6 days without irrigation — [WUR edepot](https://edepot.wur.nl/16866); [AGRIS](https://agris.fao.org/search/en/records/669f6943a4dac527f930bb92)
- Drier soil at the time of harrowing raised mortality of uprooted seedlings from 36 % to 91 % — [AGRIS](https://agris.fao.org/search/en/records/669f6943a4dac527f930bb92)
- As roots anchor seedlings more firmly, burial becomes the more important mortality source. A review states that ~6 cm of soil cover kills most plants regardless of species or stage — [OAPEN book](https://library.oapen.org/bitstream/20.500.12657/61502/1/9781801462464_web.pdf); [Weed Science: To uproot or bury](https://www.cambridge.org/core/journals/weed-science/article/to-uproot-or-bury-modeling-selectivity-of-inrow-mechanical-cultivation/82BC844EE34FDC407F35372BEFD66910)
- Kurstjens, Kropff and Perdok developed a method that predicts selective uprooting of crop and weeds from measured anchorage forces. The "Kurstjens model" estimates the maximum weed mortality achievable at a given crop mortality — [Weed Science: anchorage forces](https://resolve.cambridge.org/core/journals/weed-science/article/method-for-predicting-selective-uprooting-by-mechanical-weeders-from-plant-anchorage-forces/B8CE274FF93F914957BDAE75DD2B320F); [Weed Science: To uproot or bury](https://www.cambridge.org/core/journals/weed-science/article/to-uproot-or-bury-modeling-selectivity-of-inrow-mechanical-cultivation/82BC844EE34FDC407F35372BEFD66910)
- Caveat: most of these mortality data come from laboratory harrowing on sandy soil — [WUR edepot](https://edepot.wur.nl/16866)
- Hoe-share draft: raising cultivation depth from 30 to 50 mm increased longitudinal force by 63 %, and 50 to 70 mm added another 71 %. Depth should be held within ±10 mm of target, and the horizontal blade should control ≥98-99 % of weeds without crop injury — [Acta Agric. Scand. B, new hoe share design](https://informahealthcare.com/doi/full/10.1080/09064710.2017.1367837)
- In a tillage-tool comparison, draft, energy and soil disturbance increased with forward speed, depth and blade width, and decreased with higher soil moisture. Sweep blades had the lowest draft of the tested cultivators. Reported maxima (e.g., ~297 N at 3.82 km/h in dry soil) are for a larger, deeper setup (attribution to one of these two papers per search extract) — [Agriculture 10(5):145](https://www.mdpi.com/2077-0472/10/5/145); [CIGR Journal](https://cigrjournal.org/index.php/Ejounral/article/download/9663/4371/44217)
- Small, light robots "have problems generating interaction forces high enough to modify the environment (lift a stone, cultivate the soil...)". An interlocking-spike drive makes "the shear strength of the soil ... the limiting factor for the draft force, not the weight of the vehicle" (search extracts) — [Agriculture 2023, high-traction low-compaction platform](https://mdpi.com/2077-0472/13/8/1510); [arXiv 1912.00413](https://arxiv.org/pdf/1912.00413)
- Small-scale automated weeding tools are normally characterised in soil bins with a tool trolley. One rotary-tine study used 70 mm depth and 6.35 mm tines and measured draft and torque — [Iowa State thesis: soil-tine interaction](https://dr.lib.iastate.edu/etd/18161)
- Tertill (Franklin Robotics): solar-charged Li-ion, ~1.1 kg (2.4-2.7 lb reported), ~8.25 in (21 cm) across. Its "specially designed wheels churn up the top layer of dirt disrupting many weed seeds from ever growing", and a spinning nylon string cuts emerged weeds. It works 2-5 min at a time, then rests to recharge — [Tertill FAQ](https://tertill.com/pages/faqs); [Bob Vila review](https://www.bobvila.com/articles/tertill-weeding-robot-review/); [AgTecher](https://agtecher.com/product/tertill-robot); [ASME](https://asme.org/topics-resources/content/a-robot-to-weed-your-garden)
- A Weed Technology paper on Tertill trials observed that the grousers "caused considerable shallow soil disturbance, possibly resulting in the uprooting or burial of weed seedlings". Control of small seedlings came from the plant sensor plus "temporally random activation of the weed whacker" (snippet only; quantitative results not seen) — [Cambridge core reader](https://www.cambridge.org/core/product/07DF9F4329CB64D749E822168A097D92)
- User reviews of Tertill are mixed: one tester's bed grew weeds >14 in tall, the robot has a 3.2-star Amazon rating, and one reviewer's units failed. A claim that Cornell showed it "as effective as hand-weeding" could not be verified — [House Digest](https://www.housedigest.com/1866284/tertill-garden-weed-robot-tool-worth-buying-reviews); [Pro Tool Reviews](https://www.protoolreviews.com/tertill-solar-powered-robotic-garden-weeder/)

### Inferences
- DERIVED, traction limit: a 1-5 kg robot weighs 9.8-49 N. With a tractive coefficient of 0.3-0.6 for small grouser wheels on tilled soil (ASSUMPTION), the maximum drawbar pull is **~3-30 N**. Because draft scales strongly with depth (+63 % from 30 to 50 mm), only a very shallow tool (≤1-2 cm deep), a narrow tool (a few cm wide) or a self-propelled one (rotary or spiked wheels driven by the robot's own motors) is plausible. Spikes or grousers that key into the soil, as in the interlock-drive concept, raise available pull beyond the friction limit.
- A tiny robot that passes every 1-3 days can keep hitting weeds at the white-thread to cotyledon stage, which is the most vulnerable window. Since uprooting in dry surface soil is the main killer at that stage, aggressive grouser or spike wheels plus a light dragged rake or finger-tine comb in the inter-row is a credible blind treatment that needs no per-weed vision. This is the same mechanism Tertill claims. Efficacy will fall sharply on wet soil and on weeds past 2 true leaves.
- Rolling cultivators, torsion weeders, finger weeders, brush weeders and drag chains are all tractor tools whose effect depends on forward speed and soil throw. At a robot speed of 0.05-0.2 m/s (ASSUMPTION) the throw is lower. A powered micro-brush or rotary tine driven by a 5-20 W motor would substitute for speed.
- Burial-based kill (≥6 cm of cover) is impossible at this scale. Uprooting and desiccation is the mechanism to design for, so the robot should work in the afternoon or on dry surface soil, and avoid running just before drip irrigation.

### Gaps
- No measured specific draft (N per cm of width) at 1-2 cm depth was found for hoe blades, sweeps or tines. Soil-bin papers exist but full texts were not reached. This is the single most important missing number for the blind-tillage concept.
- No field data quantify cultivation frequency versus efficacy for 1-3-day intervals (Rasmussen and Melander frequency studies were not retrieved).
- Merfield's work on uprooting vs burial vs cutting was not retrieved.
- The Tertill Weed Technology paper's quantitative results (weed density reduction from grousers alone) were not accessible.

---

## Q3. Powered, targeted mechanical tools: string trimmer or rotating blade, micro-rotary or drill, stamping or punching, impact tools, pulling grippers, grit blasting, water jet

### Takeaway
Two small, low-energy targeted mechanisms already exist. One is the **tube stamp** (11 mm stamp, 70 W BLDC motor, up to 1.75 weeds/s, so ≤40 J/weed DERIVED). The other is the **punch/press-into-soil** action of Odd.Bot's gripper on small weeds (2 mm precision, 2 weeds/s per arm). Rotating blades and string trimmers (Tertill, Greenfield, Aigen) are cheap and robust but cut above the growing point. Cutting suppresses broadleaf seedlings more than grasses, and regrowth rises steeply with cutting height. Grit blasting works (0.5-2 g grit per weed at 500-750 kPa) but needs a compressor and kilograms of grit, which a 1-5 kg robot cannot carry.

### Cited Findings
- **Langsenkamp tube stamp** (CIGR 2014): stamp diameter 11 mm, forced through a channel in a tube, powered by a 70 W BLDC motor — [CIGR 2014 paper, HS Osnabrück](https://www.hs-osnabrueck.de/fileadmin/HSOS/Homepages/COALA/Veroeffentlichungen/2014-CIGR_2014_Tube_Stamp_for_mechanical_intra-row_individual_Plant_Weed_Control.pdf)
- Maximum throughput was ~1.75 weeds/s at 3.7 cm/s and 43 weeds per metre, with up to 9 cm/s at lower weed densities (search extract; attributed to the stamping work) — [CIGR 2014](https://www.hs-osnabrueck.de/fileadmin/HSOS/Homepages/COALA/Veroeffentlichungen/2014-CIGR_2014_Tube_Stamp_for_mechanical_intra-row_individual_Plant_Weed_Control.pdf)
- BoniRob (Bosch / Deepfield Robotics): the stamping tool is ~1 cm wide and "drives weeds about 3 cm into the soil" — [IEEE Spectrum](https://spectrum.ieee.org/bosch-deepfield-robotics-weed-control); [Quartz](https://qz.com/553383/this-robot-kills-weeds-and-could-end-the-need-for-herbicides-on-farms)
- The Bosch patent describes a tool that delivers a pulse of pressure, with energy stored in a flywheel, capacitors or pneumatic/hydraulic modules. No stamping force is published in the retrieved material — [US 10548306](https://patentimages.storage.googleapis.com/84/57/60/587733924929d0/US10548306.pdf)
- **Odd.Bot Maverick**: removes up to 2 weeds/s (stated per arm in one source), with up to 2 mm precision. "Smaller weeds are punched or pushed down into the ridge, while larger weeds that can be grasped are pulled out by the gripper." The carrier is ~500 kg and priced from ~€95,000. Claims >240,000 weeds/ha and up to 2 ha/day — [Future Farming 2023](https://futurefarming.com/tech-in-focus/field-robots/odd-bot-aims-to-excel-in-early-stage-weed-removal); [Future Farming product page](https://futurefarming.com/odd-bot-maverick); [AgTecher](https://agtecher.com/product/oddbot); [CORDIS](https://cordis.europa.eu/project/id/190145679)
- **Greenfield Robotics** (BOTONY/WeedBot): small spinning metal blades cut weeds about 0.5 in (~1.3 cm) above ground between rows. Weight reports range 200-400 lb, units are sized for 30-in rows, the maker claims 0.5-1 ac/h per bot and <1 % crop damage, battery runtime is 4-8 h, and robots are rented per acre — [AgTecher](https://agtecher.com/product/greenfield-bot); [Farms.com](https://www.farms.com/news/these-robots-provide-a-high-tech-solution-to-an-age-old-farming-problem-how-to-get-rid-of-weeds-209170.aspx); [Farm Show](https://www.farmshow.com/a_article.php?aid=39843)
- **Aigen Element**: solar-powered (custom 205 W panel on gen 1; 350 W panel on Gen 2), with metal arms carrying small blades that hoe between crop plants. Gen 2 costs $50,000 per unit plus $20,000 per fleet and covers up to 20 acres per robot — [Fresh Consulting](https://www.freshconsulting.com/work/element-gen-2/); [Tech Brew](https://www.techbrew.com/stories/2024/12/06/aigen-robots-farms-superweeds); [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops)
- **Tertill** string trimmer: plants taller than ~1-3 in are spared (capacitive height sensing) and shorter ones are cut. Seedlings <4 in need plant guards. The trimmer line lasts ~1 month — [AgTecher](https://agtecher.com/product/tertill-robot); [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/)
- **Regrowth after cutting**: cutting shoots at 5 cm reduced biomass of all 10 species tested, but 6 annual broadleaf weeds were "much more affected" than 2 perennial forage species and 2 annual grasses (*Alopecurus*, *Bromus*) — [HAL INRAE](https://hal.inrae.fr/hal-02661671)
- Raising cutting height from 5 to 8 cm increased regrowth biomass by 100-400 % for all species. Common hempnettle was the most sensitive, followed by wild oat, wild buckwheat and catchweed bedstraw (attributed by search extract to the Weed Technology 2002 "Regrowth of weed species after cutting" paper) — [Weed Technology 16:873](https://complete.bioone.org/journals/weed-technology/volume-16/issue-4/0890-037X(2002)016[0873:ROWSAC]2.0.CO;2/Regrowth-of-Weed-Species-after-Cutting1/10.1614/0890-037X(2002)016[0873:ROWSAC]2.0.CO;2.full)
- Compensatory regrowth is triggered by removing the apical shoot, which breaks apical dominance. For some species, more apical shoot removed meant less compensatory growth; ivyleaf morningglory was unaffected — [Weed Science: compensatory weed growth](https://www.cambridge.org/core/journals/weed-science/article/abs/characterization-of-compensatory-weed-growth/0557141B5070AF9A8F2EC7565F969DEF)
- Common ragweed cut three times at 20 cm showed only 25-33 % mortality (mowing extension summary) — [UMD Extension](https://www.extension.umd.edu/resource/mowing-casually-thought-integrated-weed-management-tool)
- **Air-propelled abrasive grit** (Forcella): a single exposure of ≤1 s at ~500 kPa typically killed small broadleaf seedlings. *Setaria pumila* was controlled better at 500 and 750 kPa than at 250 kPa, and with the nozzle 30 cm from plants rather than 60 cm — [Forcella 2012, Weed Technology 26:161-164](https://projects.sare.org/media/pdf/9/6/0/960924WT2012_v26_p161-164.pdf)
- Waterhemp seedlings were completely controlled with as little as 0.5 g of spent coffee grounds at 690 kPa. Velvetleaf was more tolerant but "damaged appreciably by 1 to 2 g" — [Weed Technology: spent coffee grounds](https://resolve.cambridge.org/core/journals/weed-technology/article/spent-coffee-grounds-as-airpropelled-abrasive-grit-for-weed-control-in-organic-production/30C89F771A7B7F122E6FB60A910C379D)
- In field corn, 2-3 abrasion events plus between-row cultivation gave >90 % season-long in-row control, with yields equal to hand-weeded plots — [AGRIS: Air-propelled abrasive grit in field corn](https://agris.fao.org/search/en/records/65de1fab4c5aef494fda13f0); [RAFS: abrasion with organic fertilizer grit](https://www.cambridge.org/core/journals/renewable-agriculture-and-food-systems/article/postemergence-weed-control-through-abrasion-with-an-approved-organic-fertilizer/FED4552D50C74076121347A044C12A98)

### Inferences
- DERIVED, stamp energy: 70 W ÷ 1.75 weeds/s = **≤40 J per weed** (electrical, upper bound at full motor power). With 180 kJ that is ≥4,500 weeds per charge. A spring-and-latch or capacitor-solenoid striker charged by a 5-15 W motor between hits fits the 5-50 W budget. The reaction force (pushing an 11 mm rod 3 cm into soil) must be absorbed by the robot's weight, so a 1-5 kg robot probably needs a smaller stamp (5-8 mm) and shallower target (1-2 cm) on cotyledon-stage weeds. This is an inference: no force data were found.
- The tube-stamp geometry crushes the meristem and buries it in one stroke, so unlike a trimmer it does not leave a regrowing stub. It is local (≈1 cm²), so it is the safest mechanical option within 2-5 cm of a pepper stem, apart from hand pulling.
- String trimmers and spinning blades need no precise targeting (Tertill uses only height sensing), and draw roughly 5-30 W (ASSUMPTION; no motor spec found). But they cut above the growing point. Grasses, whose meristem sits at or below the soil surface, and rosette broadleaves will regrow, so repeated cutting every few days is needed. This is acceptable for a robot that patrols every 1-3 days.
- A micro "drill" or rotary mill (a 5-10 mm cutter plunged 1-2 cm on the meristem) is conceptually between stamp and hoe. No demonstrated implementation or energy data were found.
- Grit blasting is out for this rover. DERIVED: 0.5-2 g per weed × 1,000 weeds = 0.5-2 kg of grit per 1,000 weeds, plus a compressor to supply 500-750 kPa air. Water jets likewise need a high-pressure pump and a water supply (no data found).
- Pulling grippers (Odd.Bot) need large manipulators and gripping force. For a 1-5 kg robot, the "punch small weeds into the soil" half of Odd.Bot's behaviour is the transferable idea, not pulling.

### Gaps
- No stamping force, impulse or per-weed energy measurement was found (the Michaels/Haug/Albert IROS 2015 paper was not retrieved).
- No string-trimmer or micro-blade motor power or energy-per-weed data for Tertill, Greenfield or Aigen.
- No demonstrated micro-rotary meristem drill, spring-loaded impact tool or water-jet weeder at small scale was found.

---

## Q4. Laser weeding: dose-response, wavelengths, blue-diode modules, dwell time, aiming, implementations, safety and regulation

### Takeaway
The dose needed depends strongly on growth stage and on how the dose is defined. A broad 445 nm diode array killed small seedlings at only ~0.2-0.5 J/mm² of treated area (Walsh, Muller & Coleman 2025). A focused 2 µm thulium spot on the meristem needed up to 12.7 J/mm² for cotyledon dicots and ~20 J/mm² for cotyledon-2-leaf weeds, and 3-leaf ryegrass needed 19-76 J/mm². A 5-20 W blue diode module (≈ engraver class) can plausibly kill cotyledon-to-2-leaf seedlings in ~0.2-2 s each (DERIVED). In practice it was too slow at 1 W and risked crop damage at 5 W in the one low-cost study. Every such system is IEC 60825-1 Class 4 and needs a light-tight shroud with interlocks.

### Cited Findings
- **Andreasen, Vlassi & Salehan 2024** (*Frontiers in Plant Science*): a 50 W thulium fibre laser (2 µm) aimed at the apical meristem of young weeds. Three plants per species were dosed at seven levels from 0 to 12.7 J/mm², with log-logistic dose-response fits. The listed doses fit a ~2 mm beam (3.14 mm²; 50 W → ~15.9 W/mm²; 0.8 s → 12.7 J/mm²) — this was back-calculated by the earlier researcher, not confirmed in the full text — [Frontiers in Plant Science 2024](https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2024.1375164/full)
- An MDPI paper on thulium fibre laser weeding reports that most cotyledon-stage dicot weeds were controlled at doses below 12.7 J/mm² (search extract) — [AgriEngineering 6:251](https://mdpi-res.com/d_attachment/agriengineering/agriengineering-06-00251/article_deploy/agriengineering-06-00251.pdf)
- A review states that weeds at cotyledon to two-leaf stages "are usually killed when they are exposed to about 20 J mm-2". Another abstract notes that weeds must be irradiated at cotyledon to 2-leaf stage to avoid regrowth — [Frontiers in Agronomy 2024](https://www.frontiersin.org/articles/10.3389/fagro.2024.1342372)
- Ryegrass at the three-leaf stage: all plants controlled at 76.4 J/mm², and 93.3 % at 19.1 J/mm² (2020 study) — [Frontiers in Agronomy 2020](https://www.frontiersin.org/article/10.3389/fagro.2020.601542)
- Wöltjen et al. (2008, LZH): fitted dose-response curves showed that increasingly higher doses are needed from seedling to 2nd-leaf to 3rd growth stage — [LZH](https://lzh.de/node/71131)
- **Walsh, Muller & Coleman 2025** (*Weed Technology* 39:e87): a 97 W, 445 nm laser diode array on rigid ryegrass and wild radish. Small seedlings were controlled at ~0.2-0.5 J/mm² and the energy needed rose with plant size. (Abstract-level extract; no J-per-weed figure was retrieved.) — [Cambridge core reader](https://www.cambridge.org/core/product/4E12FA1EC168FC4E223573F0D38EEF00/core-reader)
- **Rakhmatulin & Andreasen 2020** (*Agronomy* 10:1616): a compact, inexpensive laser weeding device tested on couch grass among tomatoes with 0.3, 1 and 5 W lasers. "The 1 W laser was not sufficient to eliminate all weed plants and required too long exposure time." The 5 W laser "was more efficient but also harmed the crop if the laser beam became split into two". The authors say exposure time must be reduced significantly — [MDPI Agronomy](https://www.mdpi.com/2073-4395/10/10/1616); [DOAJ](https://doaj.org/article/4782e34def6b43928fa459dd279fd7ce)
- Mississippi State / SARE onion-field project: a delta robot carrying a 450 nm, 60 W diode laser gave 90 % weed mortality within 48 h (presentation, not peer-reviewed). A separate poster used a 10 W 450 nm diode end-effector on a delta robot, which performed best within ~150 mm stand-off — [SARE LaserWeeding slides](https://projects.sare.org/media/pdf/L/a/s/LaserWeeding_Muneeb.pdf); [SARE poster](https://projects.sare.org/media/pdf/P/o/s/Poster_v13.pdf)
- 2022 corn-field robot (YOLOX detection, tracked platform, robot-arm blue laser): the authors conclude that a blue light laser is effective at removing weeds (dwell times not in the snippet) — [Frontiers in Plant Science 2022](https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2022.1017803/epub)
- Osadcuks et al. 2020 (*Agronomy Research*): a 445 nm, 12 W laser module moved by a UR10 arm over weed canopies at constant maximum output, so dose was set by scan speed and pattern — [Agronomy Research 18(SI2)](https://agronomy.emu.ee/wp-content/uploads/2020/05/AR2020_Vol18SI2_Osadcuks.pdf)
- Opt Lasers markets 450 nm weeding laser heads with minimum optical power of 30 W, 60 W and 120 W — [Opt Lasers](https://www.optlasersgrav.com/blue-lasers-in-laser-weeding)
- **Carbon Robotics**: LaserWeeder G2 uses 240 W diode lasers. The G2 1800 has 48 × 240 W lasers and 480 LED bed-top lights, and one news report gives 24 lasers for another configuration. Models range from 2.0 m / 1.9 t to 18.3 m / 6.3 t. The G2 600 covers 1.5-3.0 ac/h and needs ≥145 hp — [Carbon Robotics G2 1800](https://carbonrobotics.com/laserweeder-g2-1800); [Carbon Robotics G2 600](https://carbonrobotics.com/laserweeder-g2-600); [Profi](https://www.profi.co.uk/news/laserweeder-for-more-farms-and-crops/); [Mezha](https://mezha.ua/en/news/carbon-robotics-laserweeder-304439/)
- The original LaserWeeder reportedly used 30 CO2 lasers of 150 W each (search extract; attribution not pinned to a single page) — [Western Producer](https://www.producer.com/crops/video-high-powered-laser-put-to-work-killing-weeds/)
- **Safety**: industrial lasers for welding, cutting and marking are Class 4 under IEC 60825-1. Class 4 is a hazard to eye and skin from the direct beam and sometimes from diffuse reflection, and is a potential fire hazard. Enclosing the source can lower product classification, but an enclosure with gaps must be treated as Class 4. Typical Class 4 controls include a laser safety officer, controlled access, key switches and interlocks — [HSE Blog](https://www.hseblog.com/laser-safety/); [Voortman](https://www.voortman.net/en/knowledge-base/laser-classes-for-fiber-lasers); [Amada](https://amadaweldtech.com/wp-content/uploads/2023/04/Designing-Laser-Systems-for-Safe-Operation.pdf)
- Every laser system sold in the EU must also meet the applicable EU machinery legislation — [ComplianceGate](https://www.compliancegate.com/laser-device-regulations-european-union/)

### Inferences
- DERIVED, per-weed optical energy:
  - Walsh et al. dose (0.2-0.5 J/mm²) over a 5 mm spot (19.6 mm², ASSUMPTION for a cotyledon seedling) gives **~4-10 J optical**.
  - The thulium meristem dose (12.7-20 J/mm² on a 2 mm spot, 3.14 mm²) gives **~40-63 J optical**. Ryegrass at 3 leaves (19.1-76.4 J/mm², same spot ASSUMPTION) gives ~60-240 J.
  - The large gap between 0.2-0.5 and 12.7-20 J/mm² is unresolved. It likely reflects dose definition (whole-plant area vs focused spot) and wavelength absorption (445 nm is strongly absorbed by chlorophyll; 2 µm by water). The report should flag this rather than average the two.
- DERIVED, dwell time per weed: a 5 W optical blue module delivers 4-10 J in **0.8-2 s**. A 20 W module takes 0.2-0.5 s. A 40 J thulium-equivalent dose takes 8 s at 5 W. This matches Rakhmatulin's finding that 1 W was too slow.
- DERIVED, electrical energy: with blue-diode wall-plug efficiency of 25-35 % (ASSUMPTION; no source retrieved), 4-10 J optical needs **~11-40 J electrical** per weed, so about 4,500-16,000 weeds per 180 kJ. A "20 W" engraver module draws roughly 60-80 W when firing (ASSUMPTION), which fits the 5-50 W *average* budget only at a low duty cycle. Many hobby "10-40 W" engraver ratings are electrical input, not optical output, so the real optical power is ~1/3-1/4 of the label (ASSUMPTION; unverified).
- Aiming options:
  - Fixed beam with the robot positioning itself: cheapest, but slow.
  - Pan-tilt servo: cheap, but ±1-2 mm accuracy at 10-15 cm stand-off is marginal for a 2 mm meristem spot.
  - Galvo: fast, but adds mass and cost.
  - Short stand-off focusing (≤150 mm, per the SARE poster) inside a skirt or shroud under the robot body is the natural geometry for an under-canopy rover.
- Crop risk near pepper: millimetre aiming allows operation within 2-5 cm of a stem in principle. But Rakhmatulin saw crop damage from a split beam, and reflections off plastic mulch or wet leaves are a concern. The shroud must close to the ground (brush or curtain skirt). A downward beam inside a closed housing and a ground-contact interlock is the minimum design.
- Fire risk: Class 4 sources ignite dry residue, and plastic mulch and drip tape absorb blue and IR. Firing should be restricted to plant pixels identified by the camera, with a hard cap on dwell time.
- The EU Machinery Regulation (EU) 2023/1230 replaces the Machinery Directive and applies from 20 January 2027. This is from general knowledge and was NOT verified in this session. It would apply to a laser rover sold in the EU, in addition to IEC/EN 60825-1.

### Gaps
- Mathiassen et al. 2006, Heisel et al. 2001 (CO2 dose and meristem targeting) and Marx et al. 2012 dose values were not retrieved.
- No ED50 or ED90 per species was retrieved from Andreasen 2024 (pmc.ncbi.nlm.nih.gov and the Frontiers full text were not fetched).
- Rakhmatulin's exact wavelength, dwell times and mass/cost were not retrieved (mdpi.com blocked).
- A search snippet mentioned a 50 W blue laser killing small weeds in ~0.3-0.4 s, but the source paper could not be pinned down.
- WeLASER and LZH system specs, and RPi/Jetson hobby laser-weeding builds with measured efficacy, were not found.
- No measured wall-plug efficiency for hobby blue-diode engraver modules was found.

---

## Q5. Electrical weeding (Zasso, RootWave, CropZone, Small Robot Company "Dick", microshock): energy, voltage, contact, root kill, battery feasibility, safety

### Takeaway
Pulsed "microshock" electrical weeding (Massey/AgResearch, NZ) is the most battery-friendly physical method found. A single ~10 kV pulse of about 5 J killed small *Capsella* seedlings. Grass seedlings (*Lolium multiflorum*) needed ~200 J/plant for 90 % kill, and the authors' field paper reported very low per-hectare energies. By contrast, continuous-contact systems (RootWave, Zasso) and larger weeds need kJ per plant. A miniature capacitor-discharge unit on a 1-5 kg robot is technically plausible. The open issues are electrode contact, crop current paths within 2-5 cm, wet soil and human-safety certification.

### Cited Findings
- Massey thesis: small *Capsella bursa-pastoris* in a fallowed vegetable bed were killed by single 10 kV pulses of ~5 J. For *L. multiflorum*, a 90 % death rate was projected at 200 J per plant — [Massey thesis](https://mro.massey.ac.nz/handle/10179/71403)
- Massey group: "5 J is sufficient energy to bring about death or severe growth limitation in many seedlings up to 15 cm height" — [Massey Research Online](https://mro.massey.ac.nz/items/5936d960-461e-4f22-81b2-d9a2aded6ffa/full)
- Bloomer, Harrington, Ghanizadeh & James 2024, *Sustainability* ("Pots to Plots: microshock weed control is an effective and energy efficient option in the field"): >90 % control of all species. For 5 weeds/m², energy was ~15 kJ/ha for *Lepidium didymum* and *Amaranthus powellii*, and 363 kJ/ha (leaf contact only) to 555 kJ/ha (plant pressed to soil) for in-ground *L. multiflorum*. In-ground outdoor treatments were as effective and efficient as earlier greenhouse in-bag trials — [AgResearch figshare](https://agresearch.figshare.com/articles/journal_contribution/Pots_to_Plots_Microshock_weed_control_is_an_effective_and_energy_efficient_option_in_the_field/27747909); [IDEAS/RePEc](https://ideas.repec.org/a/gam/jsusta/v16y2024i11p4324-d1398513.html); [Landwise](https://landwise.org.nz/?p=2043)
- Discharged energy increased when the electrode pressed the plant to a dry soil surface rather than touching leaves only — [Landwise](https://landwise.org.nz/?p=2043)
- A peer reviewer of the Pots to Plots paper asked about a "safe distance" between weed and crop and whether crops can be damaged when weeds are close — [MDPI review report](https://mdpi.com/2071-1050/16/11/4324/review_report)
- Massey/Landwise also built a hand-held "electric spatula" — [Landwise: the electric spatula](https://www.landwise.org.nz/2023/11/05/the-electric-spatula/)
- Swedish pot study: 5-20 kV and 0.2-13.5 s exposures. 5 kV for 0.2 s gave 100 % mortality of *Tripleurospermum inodorum* seedlings — [SLU](https://pub.epsilon.slu.se/id/document/20445124)
- Greenhouse continuous-contact studies reported 4-111 kJ per plant — [Agronomy 2023 review](https://mdpi.com/2073-4395/13/11/2694)
- A 2025 field study in blueberry found that tissue damage depends on plant density and morphology, water content, and plant and soil electrical conductivity, so pot data do not transfer directly — [Frontiers in Agronomy 2025](https://public-pages-files-2025.frontiersin.org/journals/agronomy/articles/10.3389/fagro.2025.1688961/pdf)
- RootWave (manufacturer claim): effective control from 50 MJ/ha (range recorded 50-98 MJ/ha), "a fifth of the energy of herbicides" per hectare, using patented high-frequency AC — [No-Till Farmer](https://no-tillfarmer.com/articles/11882-uks-rootwave-electro-weeder-succeeds-in-independent-trials); [Future Farming](https://futurefarming.com/crop-solutions/weed-pest-control/electric-weeding-with-high-frequency-electricity-gives-better-weed-control-with-lower-energy-use)
- Small Robot Company "Dick": uses RootWave technology delivering "10-12,000 volt" blasts, positioned by an igus delta robot. Robots are battery powered and ≤300 kg. As of 2021 Dick was a prototype, and its trials were to explore the force needed for different weeds — [Electricity Forum](https://electricityforum.com/news/weed-electricity); [The Engineer](https://www.theengineer.co.uk/small-robot-company-igus-weeding/); [igus](https://toolbox.igus.com/?p=3666); [Future Farming](https://futurefarming.com/small-robot-company-dick-weeding-robot)
- Site-specific electrocution ~15-19 MJ/ha (Coleman 2019) — [Cambridge](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E)

### Inferences
- DERIVED: at 5 J per broadleaf seedling and 60-80 % HV converter efficiency (ASSUMPTION), the battery cost is ~6-8 J per weed, or **~22,000-29,000 broadleaf seedlings per 180 kJ**. At 200 J per grass seedling it is ~250-330 J per weed, or ~550-700 grass seedlings per charge.
- Internal inconsistency to flag: the Bloomer 2024 per-hectare numbers at 5 weeds/m² (50,000/ha) imply ~0.3 J per broadleaf weed (15 kJ/ha) and ~7-11 J per ryegrass weed (363-555 kJ/ha). Those are an order of magnitude below the thesis's 5 J and 200 J figures. Either the per-ha figures count only delivered pulse energy on very small seedlings, or the extract garbled units. The full paper must be checked.
- A 5 J pulse is in the same range as the per-pulse energy cap commonly applied to electric-fence energizers (IEC 60335-2-76). This is from general knowledge, NOT verified this session. It suggests a pulsed design could follow a familiar safety framework. Continuous-contact HV (RootWave/Zasso style) at kV and kJ levels is far less compatible with a consumer-scale robot.
- Hardware: a pulsed design needs a small HV converter and capacitor (flyback or ignition-coil class), a probe electrode lowered onto the weed, and a return path through the soil. Mass of order 100-300 g is plausible (ASSUMPTION; a hand-held spatula exists), which fits a 1-5 kg robot.
- Crop risk: current returns through the soil and roots. Within 2-5 cm of a pepper stem in wet, drip-irrigated soil, crop roots may share the current path. No safe-distance data were found. Plastic mulch insulates (blocking return through soil), and drip tape is a conduction and damage risk.
- Electrical treatment kills roots of seedlings (current travels to the roots), which is an advantage over cutting. Grasses need ~40× more energy than broadleaf seedlings per the thesis figures.

### Gaps
- No Zasso or CropZone voltage, frequency or J/weed data were found. CropZone's (herbicide + electrical) energy figures were not found.
- The safe crop-to-electrode distance is unknown.
- Behaviour on wet or drip-irrigated soil and on plastic mulch was not quantified.
- Regulatory status for autonomous HV weeding in the EU was not found.

---

## Q6. Thermal methods (flame, hot water, steam, foam, infrared, hot air, contact probe, microwave): energy per weed, battery feasibility, fire and mulch risk

### Takeaway
Thermal methods are energy-hungry even when targeted. Site-specific flaming costs ~200-870 J of fuel per weed (DERIVED). Hot water costs ~0.35 kJ per mL heated, so ≥0.7-3.5 kJ per weed (DERIVED). Microwave needs 77-500 J/cm² (≈100-4,000 J per seedling). None fits a 50-150 Wh battery at useful weed counts, and flame and heat endanger plastic mulch, drip tape and pepper stems. A heated contact probe touching only the meristem is the only thermal variant that might fit, but no data were found.

### Cited Findings
- Broadcast flaming needs 1,008-4,334 MJ/ha for young broadleaf weeds, and infrared 2,000-3,887 MJ/ha (100-1,000× tillage). Site-specific treatment reduces thermal energy by ~99 % — [Cambridge (Coleman 2019)](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E); [AHRI](https://www.ahri.uwa.edu.au/?p=16169)
- A review notes that no published research had quantified the minimum mid-infrared energy dose (J/cm²) needed to control seeds or seedlings — [Cambridge thermal review](https://www.cambridge.org/core/product/65ADA897B679D840205CDD3843C13A33/core-reader)
- Hot-water work at Ghent (De Cauwer et al. 2015, *Weed Research*) expressed dose as energy per unit area (kJ/m²) as a function of water temperature and spray volume (numbers not retrieved) — [De Cauwer 2015 PDF (WUR)](https://www.wur.nl/upload_mm/d/e/a/2a47a866-63c7-4482-a2d9-2018679e87b6_Weed%20Research%20-%20Efficacy%20and%20fuel%20use%20hot%20water%20weed%20control%20(De%20Cauwer%2C%202015).pdf)
- A modular multi-nozzle hot-water weeder for field robots, with vision that sets water volume per plant, has been described — [AURA project](https://aura-mu.github.io/portfolio-HWW.html)
- A patent describes detecting a "particularly vulnerable point" per plant and directing treatment there, to cut liquid and energy and allow treatment closer to crop plants — [US patent 12245584](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12245584)
- Microwave: theoretical ~1.7 kJ to heat a ~13 g plant from ~20 °C to a lethal temperature (Brodie et al. 2019, a calculation, not a measurement) — [GJAIRD](https://avantipublishers.com/index.php/gjaird/article/download/716/369/1420)
- Area-wide microwave needed ~10× the energy of chemical treatment. A short, intense burst aimed at the stem was as effective as whole-plant treatment with much less energy — [Brodie et al. 2012, Int. J. Agronomy](https://www.doi.org/10.1155/2012/287608)
- Emerged weeds under a horn antenna needed ~77-500 J/cm², depending on species and antenna height — [CORE](https://core.ac.uk/download/162219343.pdf)
- 560 J/cm² applied to soil reduced weed establishment from the seedbank by 70-80 % — [Univ. Melbourne](https://findanexpert.unimelb.edu.au/scholarlywork/1214011-the-effect-of-microwave-soil-treatment-on-rice-production-under-field-conditions)

### Inferences
- DERIVED, hot water: heating water from 15 to 98 °C takes 4.19 J/(g·K) × 83 K ≈ **348 J per mL**. At 2-10 mL per weed (ASSUMPTION), that is 0.7-3.5 kJ per weed, or only ~50-250 weeds per 180 kJ, plus 2-10 kg of water per 1,000 weeds. Steam (≈2.6 kJ/g including latent heat) is worse per gram.
- DERIVED, microwave: 77-500 J/cm² × 1-4 cm² seedling (ASSUMPTION) = 77-2,000 J RF. At an assumed magnetron or solid-state efficiency of 50-70 %, that is ~110-4,000 J electrical per weed. Hardware is typically kW-class, and RF exposure is a hazard. Not suitable.
- Flame from a butane cartridge would sidestep the battery: ~200-870 J per weed as fuel (DERIVED from Coleman) means a 100 g butane canister (~4.6 MJ at ~46 MJ/kg, ASSUMPTION) could in principle treat thousands of weeds. However, plastic mulch, drip tape, dry residue under canopy, and pepper stems within 2-5 cm make open flame a serious fire and crop-injury risk.
- A resistively heated contact probe pressed on the meristem (a "soldering iron" approach) is the only thermal option that could be both local and battery-feasible. Energy and efficacy are unknown, and it contacts soil and may fail on wet or muddy seedlings. Treat it as an untested idea.

### Gaps
- No measured J/weed for hot water, steam, foam, hot air or contact probes on seedlings was found.
- De Cauwer's numeric dose-response was not retrieved.
- No small-robot thermal implementation was found.

---

## Q7. Chemical micro-dosing (drop-on-demand, micro-spot spraying: Ecorobotix, Utstumo "Asterix", Urdal, Kilter AX-1): µL per weed, hardware, natural contact herbicides, pepper labels, EU regulation

### Takeaway
Drop-on-demand delivered effective control with micrograms of active per weed: 7.6 µg glyphosate or 0.15 µg iodosulfuron per plant in pots, and 5.3 µg glyphosate per droplet in a carrot field, a ~10× cut in herbicide use. Energy per weed is negligible, and a 100-500 mL tank serves 10^4-10^5 weeds (DERIVED). The drawbacks are regulatory: authorisation for use in the crop, operator rules for autonomous application, and organic incompatibility for synthetic actives. Natural contact actives (pelargonic acid) need ~1 g a.i./m² broadcast-equivalent, are weaker on grasses, and work only on young weeds.

### Cited Findings
- Utstumo PhD: indoor pot trials with four weed species showed the drop-on-demand (DoD) system could control weeds with as little as 7.6 µg glyphosate or 0.15 µg iodosulfuron per plant — [Utstumo PhD (NTNU)](https://tomgra.folk.ntnu.no/Trygve%20Utstumo_PhD.pdf)
- Asterix field trial in carrots: weeds were effectively controlled with 5.3 µg glyphosate per droplet, a ten-fold reduction in herbicide use — [Asterix paper (NTNU Open)](https://ntnuopen.ntnu.no/ntnu-xmlui/bitstream/11250/2592250/1/asterix_paper_v1.3.pdf); [NTNU Open record](https://ntnuopen.ntnu.no/ntnu-xmlui/handle/11250/2562224)
- Urdal et al. 2014: glyphosate had no effect in DoD droplets unless mixed with suitable additives — [Urdal ICARCV 2014](https://tomgra.folk.ntnu.no/papers/Urdal_ICARCV_2014.pdf)
- EU project and press claims: ~95 % herbicide reduction (project objective and reporting, not independent measurement) — [CORDIS 829983](https://cordis.europa.eu/project/id/829983/reporting/es); [Irish Examiner](https://www.irishexaminer.com/farming/arid-30970338.html)
- Kilter AX-1 applies a micro-droplet of pelargonic acid to each detected weed with 6 × 6 mm targeting resolution, and claims an average 96.6 % input reduction at ~5 % weed pressure (vendor claim; droplet volume not stated) — [Kilter Systems](https://www.kiltersystems.com/products)
- Ecorobotix promotes ultra-high-precision spraying to make bioherbicides viable (vendor material; no per-weed volume retrieved) — [Ecorobotix](https://ecorobotix.com/?p=19793)
- A machine-vision robotic micro-dosing system achieved sub-centimetre accuracy in indoor tests — [Bohrium summary](https://www.bohrium.com/paper-details/application-accuracy-of-a-machine-vision-controlled-robotic-micro-dosing-system/812022476266536962-1388); see also [NSF PAR: robotic micro-volume herbicide spray](https://par.nsf.gov/biblio/10386376-algorithm-system-development-robotic-micro-volume-herbicide-spray-towards-precision-weed-management)
- Pelargonic acid, broadcast: one product (680 g a.i./L) at a target 16 L/ha in 400 L/ha spray volume, in a logarithmic sprayer trial (attribution per search extract) — [Julius-Kühn-Archiv](https://ojs.openagrar.de/index.php/JKA/article/view/8947/8226)
- Pelargonic acid efficacy is "partial" in greenhouse and field. Grasses, notably *Alopecurus myosuroides* and *Lolium rigidum*, are less sensitive — [Agronomy 2023 (DOAJ)](https://doaj.org/article/8ac365ffe3cf43eea06b5b7884620df3). Acceptable efficacy is achievable on young weed stages — [AGRIS: pelargonic acid on several weed species](https://agris.fao.org/search/en/records/687a8a579d6e0577321ec144)

### Inferences
- DERIVED, pelargonic per spot: 16 L/ha × 680 g/L = 10.9 kg a.i./ha = 1.09 g/m² = 109 µg/cm². The spray is 40 mL/m² (4 µL/cm²) at ~27 g a.i./L. A 6 × 6 mm spot (0.36 cm²) therefore takes ~1.4 µL carrying ~39 µg a.i. A **100 mL tank is ~69,000 spots** (500 mL ≈ 350,000). Even at 5-10× more per spot to ensure full meristem coverage, that is still 7,000-70,000 weeds per 100 mL.
- DERIVED, glyphosate: at 5.3-7.6 µg per weed and a 1 % a.e. working solution (10 µg/µL, ASSUMPTION), each weed gets 0.5-0.8 µL, so 100 mL covers 130,000-190,000 weeds. Energy is a solenoid or piezo pulse of ~mJ (ASSUMPTION), which is negligible.
- Hardware is the lightest of all options: a 5-50 g micro-solenoid or inkjet-style valve, a small pump or pressurised reservoir, and a 100-500 g tank. It works on grasses (systemic actives) as well as broadleaves, and kills roots.
- Crop risk: drift is tiny with droplets aimed from a few cm. Glyphosate (systemic) on a pepper leaf or green stem is catastrophic, so per-weed targeting must exclude crop pixels with a margin. Contact actives (pelargonic, acetic) only scorch what they touch, so they are safer near pepper but weaker.
- Regulation (general knowledge, NOT verified this session): in the EU, plant protection products are authorised per crop and use under Regulation (EC) 1107/2009, and Directive 2009/128/EC governs application. Glyphosate and synthetic actives are not permitted in organic production. An autonomous consumer robot dispensing herbicide would face operator-certification and label questions.

### Gaps
- No list of herbicides labelled for pepper (EU or US) was retrieved. No acetic-acid per-weed dose was found.
- No droplet volume from Kilter or Ecorobotix was found.
- The EU legal status of autonomous spot-application by non-professional users was not researched.

---

## Q8. Other mechanisms: UV-C, high-intensity light, cryogenic, ultrasound, seed-bank depletion, seed predation, mulching, cover-crop seeding, trampling or crushing, pneumatic blowing

### Takeaway
None of these has data supporting it as a primary killing tool on a tiny battery robot. UV-C energy is prohibitive (up to 100,000 MJ/ha; lettuce tolerated 1.7 kJ/m²). The useful "other" levers are indirect: frequent shallow disturbance to deplete the surface seedbank, mulch (plastic or organic) to suppress emergence in the row, and crushing or punching seedlings into soil, which a light robot's wheels or a stamp already do.

### Cited Findings
- UV-based thermal weed control "can require up to 100,000 MJ/ha" for some species — [Cambridge thermal review](https://www.cambridge.org/core/product/65ADA897B679D840205CDD3843C13A33/core-reader)
- Oilseed rape seedlings were treated with 0.2-1.5 kJ/m² UV-C for pathogen suppression, not weed kill; 0.8 kJ/m² seven days before inoculation cut disease severity by 44 % — [J. Plant Diseases & Protection / Springer 2024](https://link.springer.com/article/10.1007/s10343-024-01021-8)
- Lettuce chlorophyll fluorescence (Fv/Fm) was not reduced by UV-C up to 1.70 kJ/m², i.e. no PSII damage (review, per search extract) — [Folia Horticulturae 2025](https://reference-global.com/download/article/10.2478/fhort-2025-0012.pdf)
- Microwave soil treatment at 560 J/cm² reduced seedbank emergence by 70-80 % — [Univ. Melbourne](https://findanexpert.unimelb.edu.au/scholarlywork/1214011-the-effect-of-microwave-soil-treatment-on-rice-production-under-field-conditions)
- Odd.Bot's small-weed action is to punch or push seedlings down into the soil — [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/odd-bot-aims-to-excel-in-early-stage-weed-removal)
- Tertill's grousers cause shallow soil disturbance that may uproot or bury seedlings — [Cambridge core reader](https://www.cambridge.org/core/product/07DF9F4329CB64D749E822168A097D92)

### Inferences
- DERIVED: UV-C at up to 10 MJ/m² (= 1 kJ per cm² of weed) with emitters at a few % efficiency (ASSUMPTION) is roughly tens of kJ electrical per weed, so infeasible. UV-C doses that crops tolerate (≤1.7 kJ/m²) are far below doses reported for weed kill, so there is no evident selective window.
- Crushing or trampling: a 1-5 kg robot's ground pressure is low (DERIVED: 49 N on ~4 wheel patches of a few cm² each is tens of kPa, an ASSUMPTION about contact area), but repeated passes over white-thread and cotyledon seedlings with grousers damage them. This is the Tertill mechanism and it costs nothing beyond driving energy.
- Seedbank depletion: frequent shallow stirring (stale-seedbed style) triggers germination and then kills the flush. A daily-patrolling robot is well suited to this, but no quantitative source was retrieved here.
- Cryogenic, ultrasound and pneumatic blowing: no data were found. Each would need large energy or consumables (LN2, compressed air) and is not credible at this scale.

### Gaps
- No data were found on cryogenic, ultrasound, high-intensity visible light (non-laser), pneumatic blowing, seed predation enhancement, or EarthSense-style cover-crop seeding from small robots. These were not searched for lack of budget.

---

## Q9. Selectivity, targeting precision and crop safety within 2-5 cm of a pepper stem; smallest demonstrated implementations

### Takeaway
Pepper transplants form a fibrous, fairly shallow root system. Shallow cultivation (3 cm) did not hurt yield, but deep cultivation (9 cm) did, by pruning roots and drying soil. So soil-engaging tools must stay ≤2-3 cm near the row. Blind tools (wheels, rake, trimmer) can run across the inter-row without per-weed vision. Inside ~5 cm of the stem, only mm-targeted actions belong: stamp or punch, micro-dose of a contact or systemic active, a shrouded low-power laser, or perhaps electric pulses. The smallest demonstrated systems are Tertill (1.1 kg; trimmer + wheels), Rakhmatulin's compact 5 W laser, Massey's hand-held electric spatula, and the Langsenkamp tube-stamp module (70 W motor).

### Cited Findings
- Transplanted peppers root to about 2 ft but use soil moisture efficiently — [MSU Extension E1815](https://www.canr.msu.edu/resources/peppers_commercial_vegetable_recommendations_e1815). Transplants typically form a fibrous root system without the taproot of direct-seeded plants (≥3 ft) — [Texas A&M AgriLife](https://aggie-horticulture.tamu.edu/vegetable/files/2011/10/pepper-Bell.pdf)
- A Texas HortScience study compared 9 cm (deep) with 3 cm (shallow) cultivation of pepper. Deep cultivation decreased productivity, attributed to root pruning and soil drying (attribution to this ASHS PDF per search extract) — [HortScience 22(3)](https://journals.ashs.org/downloadpdf/view/journals/hortsci/22/3/article-p418.pdf)
- Precision of targeted actuators: Odd.Bot up to 2 mm — [AgTecher](https://agtecher.com/product/oddbot); Kilter 6 × 6 mm — [Kilter](https://www.kiltersystems.com/products); micro-dosing robot sub-centimetre — [Bohrium](https://www.bohrium.com/paper-details/application-accuracy-of-a-machine-vision-controlled-robotic-micro-dosing-system/812022476266536962-1388)
- Laser crop injury occurred from a split beam at 5 W next to tomato — [MDPI Agronomy](https://www.mdpi.com/2073-4395/10/10/1616)
- Inter-row hoe blades should stay within ±10 mm depth and control ≥98-99 % of weeds without crop injury — [Acta Agric. Scand. B](https://informahealthcare.com/doi/full/10.1080/09064710.2017.1367837)
- Greenfield claims <1 % crop damage with blades ~0.5 in above ground between rows — [AgTecher](https://agtecher.com/product/greenfield-bot)
- Tertill relies on height sensing (spares plants taller than ~1-3 in), and seedlings need guards — [AgTecher](https://agtecher.com/product/tertill-robot); [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/)
- Electrical: the crop-weed safe distance was raised as an open question by a reviewer — [MDPI review report](https://mdpi.com/2071-1050/16/11/4324/review_report)

### Inferences
Selectivity and near-stem suitability per mechanism (INFERENCE; precision figures from the sources above):

| Mechanism | Per-weed vision needed? | Usable 2-5 cm from pepper stem? | Smallest demonstrated |
|---|---|---|---|
| Grouser/spike wheels, dragged rake, finger tines (≤2 cm deep) | No (blind inter-row) | Only if depth ≤2-3 cm and the tool stays ≥5 cm from stem; no in-row selectivity | Tertill, 1.1 kg |
| String trimmer / spinning blade | Minimal (height or obstacle sensing) | Risky: stem abrasion. Needs a guard or a crop-pixel mask | Tertill, 1.1 kg; Greenfield ~100-180 kg |
| Stamp / punch (5-11 mm) | Yes, ~2-5 mm accuracy | Yes, very local; best mechanical option | Tube-stamp module (70 W motor) on BoniRob; Odd.Bot punching |
| Micro-rotary drill on meristem | Yes | Probably yes (untested) | None found |
| Gripper pulling | Yes | Risk of disturbing crop roots in fibrous zone | Odd.Bot (500 kg) |
| Grit blasting | Coarse (band) | Corn tolerated it in-row; pepper unknown | Tractor-mounted |
| Laser (blue diode / thulium) | Yes, mm aiming on meristem | Yes if shrouded, beam-split and reflection risk controlled | Rakhmatulin compact 5 W device; SARE 10 W delta robot |
| Electrical pulse | Yes (electrode placement) | Unknown: soil/root current paths | Massey hand-held spatula |
| Thermal (flame, hot water, steam, microwave) | Coarse | No: heat spread, mulch and drip-tape damage | Tractor or hand units |
| Micro-dose (systemic) | Yes, crop exclusion margin critical | Yes with good masking; any crop hit is serious | Asterix, Kilter (large) |
| Micro-dose (pelargonic / acetic) | Yes | Yes, contact only; weaker on grasses | Kilter AX-1 |

### Gaps
- No pepper-specific data on root-zone radius versus distance from the stem at 1-3 cm depth were found, and no data on pepper sensitivity to laser, electrical or abrasive exposure.
- Plastic mulch plus drip tape interactions with each tool were not quantified anywhere.

---

## Q10. Ranking for a 15 x 20 cm, 1-5 kg, 50-150 Wh rover and the 2-3 most promising tool concepts

### Takeaway
The most promising concepts are:
1. **Blind, high-frequency surface disturbance**: aggressive grouser or spike wheels plus a light finger-tine or rake comb, swept every 1-3 days to kill weeds at the white-thread to cotyledon stage, with a small string trimmer or blade for escapes (the Tertill model, scaled for pepper inter-rows).
2. **A targeted micro-stamp or punch** (5-11 mm, ≤40 J/weed) for weeds near the crop and for escapes.
3. **A shrouded 5-20 W 445-450 nm diode laser** for cotyledon-to-2-leaf seedlings, or, if the use is non-organic, chemical micro-dosing (lowest energy and mass, effective on grasses, but blocked by regulation and organic rules).

Pulsed microshock electrical weeding is a strong dark horse on energy (≈5 J/seedling) but has unresolved crop-proximity and safety questions.

### Cited Findings
- Electrical battery cost: ~5 J per small broadleaf seedling, ~200 J per grass seedling — [Massey thesis](https://mro.massey.ac.nz/handle/10179/71403)
- Laser dose: 0.2-0.5 J/mm² for small seedlings with a 445 nm array — [Walsh et al. 2025](https://www.cambridge.org/core/product/4E12FA1EC168FC4E223573F0D38EEF00/core-reader); ~20 J/mm² focused at cotyledon-2-leaf — [Frontiers in Agronomy 2024](https://www.frontiersin.org/articles/10.3389/fagro.2024.1342372)
- Stamp: 70 W motor, ~1.75 weeds/s — [CIGR 2014](https://www.hs-osnabrueck.de/fileadmin/HSOS/Homepages/COALA/Veroeffentlichungen/2014-CIGR_2014_Tube_Stamp_for_mechanical_intra-row_individual_Plant_Weed_Control.pdf)
- Micro-dose: 5.3-7.6 µg glyphosate per weed — [Utstumo PhD](https://tomgra.folk.ntnu.no/Trygve%20Utstumo_PhD.pdf); [Asterix](https://ntnuopen.ntnu.no/ntnu-xmlui/bitstream/11250/2592250/1/asterix_paper_v1.3.pdf)
- White-thread vulnerability and uprooting mortality in dry soil (36 → 91 %) — [AGRIS](https://agris.fao.org/search/en/records/669f6943a4dac527f930bb92)
- Tertill precedent: 1.1 kg, ~21 cm, solar, wheels + trimmer — [Tertill FAQ](https://tertill.com/pages/faqs); [AgTecher](https://agtecher.com/product/tertill-robot)
- Flaming and IR need 1,008-4,334 MJ/ha broadcast — [Cambridge](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E)

### Inferences
Ranking against the rover constraints (all DERIVED or INFERENCE; scores are qualitative):

| Rank | Concept | Battery J/weed (DERIVED) | Hardware add-on (ASSUMPTION) | Efficacy envelope | Main risks |
|---|---|---|---|---|---|
| 1 | Blind shallow disturbance (grouser/spike wheels + finger-tine/rake comb) every 1-3 days, + micro trimmer for escapes | No per-weed cost. Driving energy only (~5-15 W locomotion, ASSUMPTION) | ~50-200 g; cheap | White-thread to cotyledon; best on dry surface soil; weak on grasses past 2 leaves and on wet soil | Draft limit ~3-30 N; no in-row selectivity; trimmer regrowth |
| 2 | Targeted micro-stamp / punch (5-11 mm, spring or flywheel energy store) | ≤40 J (≥4,500 weeds per 180 kJ) | ~150-400 g; servo/BLDC + latch | Small seedlings of all types; buries and crushes the meristem | Reaction force on a light robot; needs ~2-5 mm targeting; no force data |
| 3a | Shrouded 445-450 nm diode laser, 5-20 W optical | ~11-40 J (~4,500-16,000 weeds per charge); 0.2-2 s dwell | ~150-400 g incl. heatsink and optics; pan-tilt or fixed + robot positioning | Cotyledon to 2-leaf; grasses and later stages need several times more energy | Class 4 eye and fire risk; EU machinery and laser compliance; reflections off mulch |
| 3b | Chemical micro-dose (contact or systemic) | ≈0 J; 100 mL ≈ 10^4-10^5 weeds | ~100-600 g incl. tank | Systemic: all stages incl. grasses. Pelargonic: young weeds only, weak on grasses | Crop authorisation, organic ban, misfire on crop |
| 4 | Pulsed microshock electrode | ~6-10 J broadleaf; ~250-330 J grass | ~100-300 g HV module + probe | Broadleaf seedlings excellent; grasses costly; kills roots | Crop current path within 2-5 cm unknown; wet soil; HV safety and certification |
| 5 | Powered string trimmer alone | Low (motor running) | ~50-150 g | Suppression, not kill; grasses regrow | Stem abrasion near crop |
| 6 | Grit blasting | Compressor energy plus 0.5-2 g grit per weed | Compressor + kg of grit | Good on broadleaf seedlings | Payload infeasible |
| 7 | Thermal (flame, hot water, steam, microwave), UV-C | 200 J (flame fuel) to tens of kJ | Heavy | Variable | Energy, mass, fire, mulch and drip-tape damage |

- Combination recommendation (INFERENCE): concept 1 runs continuously in the inter-row strip with no per-weed vision. Vision then flags in-row or near-stem weeds and escapes ≥2 leaves for concept 2 (organic) or 3a/3b. That keeps the energy-intensive targeted tool for a minority of weeds.
- Worked example (DERIVED, illustrative): a 1,000 m² polytunnel with ~60 % inter-row area (ASSUMPTION) and a flush of 20 weeds/m² holds ~12,000 weeds.
  - Laser at ~30 J/weed: ~360 kJ, about one full 100 Wh pack per flush.
  - Electrical at ~8 J: ~100 kJ.
  - Stamp at ≤40 J: ≤480 kJ.
  - Micro-dose: ~10-20 mL of liquid.
  - Blind cover of 600 m² at 54 m²/h (0.1 m/s, 15 cm swath, ASSUMPTIONS) takes ~11 h. At ~15 W total that is ~165 Wh, about 1-3 charges per full pass. So covering the area every 1-3 days needs either ~1 h/day per 50-100 m² of robot time, or several robots.

### Gaps
- Numbers that would most change the ranking:
  - measured draft at 1-2 cm depth for small tines and blades;
  - the force and impulse needed for a stamp to kill cotyledon-stage weeds;
  - blue-diode-module wall-plug efficiency and real optical output;
  - safe electrode distance from crop;
  - field efficacy of grouser-only disturbance (Tertill paper).
- None of these were retrievable in this session; several sources sit on blocked hosts (mdpi.com, cambridge core-prod, bioone.org, pmc, sare.org, grdc.com.au) and could be checked if those hosts are allowed.
