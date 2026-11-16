# Shoebox Weeders Must Out-Visit Weeds, Not Out-Muscle Them

You can build a 15 × 20 cm under-canopy weeding rover from today's parts, and it fits physically in every pepper system examined. It is viable only as a slow, low-force "patroller" that comes back every few days and kills weeds between the white-thread and two-leaf stages. It is not viable as a miniature cultivator. Width is never the constraint. Open-field rows are 70-102 cm, plasticulture row middles roughly 50-90 cm and tunnel paths 60-90 cm. The limit is height: pepper's first fruit and branches sit **10-25 cm above the soil**, so late in the season the body must stay below about 10-12 cm. Force sets the hard ceiling. A 1-5 kg robot can pull only about **2-25 N**, while a full-width 15 cm blade in moist or crusted soil needs roughly **20-150 N**. The tool must therefore be narrow, rotary, stamping, optical, electrical or chemical, and it must be aimed at very small weeds. Energy is adequate: 8-25 W on a ~100 Wh pack runs 4-12 h. Throughput is not. One rover covers about **0.04-0.2 ha/day** if it cultivates its whole inter-row, or about **0.3-0.8 ha/day at 0.3 m/s** if it drives each inter-row once and treats only the weeds it sees. A hectare therefore needs roughly 0.25-2 robots on a 5-7-day cycle. Camera-only row following under canopy has been proven on larger robots (485 m per intervention). Weed perception fits on a Raspberry Pi 5 with a Hailo accelerator or on a Jetson Orin Nano. But nobody has shown reliable camera-only row switching, and RTK GNSS under canopy is off by 0.4 m or more. No robot this small has ever reported measured weed control in a farm field. The closest, the 1.1 kg Tertill, achieved 54-75% control in gardens and has been discontinued. The value case is narrow: weeding close to pepper stems, where hand hoeing costs growers hundreds to two thousand dollars or pounds per hectare; organic plasticulture row middles; and bare-soil tunnels. It is weak in the open inter-row of conventional fields, which tractor hoes already clean cheaply. Most source numbers come from abstracts and search extracts rather than full texts, and many design figures are first-order calculations. They set bounds on the problem; they do not settle it.

| Dimension | Verdict | What binds it |
|---|---|---|
| Fit in pepper rows | Fits in every system | Body ≤10-12 cm tall under 10-25 cm branches late in the season; muddy furrows after irrigation |
| Mobility | Feasible with 10-15 cm wheels and ≥5 cm clearance | Wet clay, clods and bed shoulders; never field-tested at this width |
| Tool force | Low-force tools only | 2-25 N of pull, against 20-150 N for a 15 cm blade in cohesive soil |
| Energy | Adequate on battery | ~100 Wh gives 4-12 h; solar under canopy gives only ~3-9 Wh/day |
| Coverage | Weak | 0.04-0.2 ha/day full-width; ~0.3-0.8 ha/day targeted |
| Navigation | In-row following proven on larger robots; row turns unproven | Turns were done by hand even in a 25 km deployment; GNSS error ≥0.4 m under canopy |
| Weed perception | Feasible on Pi 5 + Hailo or on Orin Nano | No data at 5-25 cm camera height; pepper look-alike weeds; blur and low light |
| Where it pays | Niche | Only near stems, or where herbicides and tractors cannot go |

## No robot this small has weeded a farm field yet

The concept has no direct precedent. The nearest dry-land relative is the Tertill garden robot ([AgTecher](https://agtecher.com/product/tertill-robot)):

- a solar disc of about 1.1 kg and 21 × 21 × 12 cm;
- it scuffs the soil with cambered grouser wheels and cuts short plants with a spinning nylon string;
- it has no camera, and treats anything short enough to pass under it as a weed.

Because of that rule, beds had to be hand-cleaned first and enclosed by a gap-free barrier about 10 cm high ([Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/)). In the only peer-reviewed trial ([Weed Technology](https://www.cambridge.org/core/journals/weed-technology/article/abs/functionality-and-efficacy-of-franklin-robotics-tertill-robotic-weeder/07DF9F4329CB64D749E822168A097D92)):

- it gave **54-75% weed control with the trimmer running and 16-29% with the wheels alone**;
- it reduced broadleaf weeds but not grasses;
- the authors judged that it "would require modification to be viable for farm-scale use".

The company has since ended the product and its support ([Tertill](https://tertill.com/)).

The only sub-10 kg weeders in multi-farm use work in flooded rice. The 6 kg solar Aigamo Robo was trialled on 36 organic farms and released in Korea in September 2026 ([Herald Corp](https://biz.heraldcorp.com/article/10863020)). It stirs up mud so the cloudy water shades out weeds. That action is non-selective, so it needs no crop/weed perception, and it does not carry over to dry soil.

The navigation precedent is stronger. EarthSense's TerraSentia is 33 cm wide, 52 cm long and under 13.6 kg ([UST Magazine](https://ust-media.com/ust-magazine/UST024/76); [Farm Progress](https://www.farmprogress.com/technology/autonomous-compact-robots-hold-potential-for-growers)). With LiDAR-based navigation it logged 72 km under corn and sorghum, but finished only **89.76% of autonomous row segments** and "does not provide a fully intervention-free navigation" ([Higuti thesis](https://www.teses.usp.br/teses/disponiveis/18/18149/tde-31052021-194235/publico/2021_higuti_thesis_perception_lidar.pdf)). The platform was used for phenotyping and cover-crop seeding; no weeding tool was ever commercialised on it. A 2024 Illinois study noted that under-canopy weeding robots "are not yet commercially available for corn and soybeans" ([ACES Illinois](https://aces.illinois.edu/news/new-illinois-study-explores-adoption-robotic-weeding-fight-superweeds)). The rover studied here is about half TerraSentia's width and a tenth to a third of its mass.

Commercial weeding robots have instead settled at 90-400 kg:

| Robot | Mass | Price |
|---|---|---|
| Naïo Oz | 150 kg | about €40,000 ([Future Farming](https://futurefarming.com/naio-oz-mechanical-weeding-robot-and-farmhand)) |
| Greenfield | roughly 90-180 kg (reports conflict; [AgTecher](https://agtecher.com/product/greenfield-bot)) | $36,500 per robot ([Compass Vermont](https://www.compassvermont.com/p/vermont-farmers-see-robotic-weeders)) |
| Aigen Element | not published | $50,000 per unit plus a $20,000 per-fleet fee ([Future Farming](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops)) |
| Odd.Bot Maverick | about 400 kg | from about €95,000 ([Future Farming](https://futurefarming.com/odd-bot-maverick)) |

Mass buys soil force. One 90 kg platform can apply about **2,082 N** to the soil, and its authors note that light robots struggle to generate cultivating forces at all ([Agriculture 2023](https://doi.org/10.3390/agriculture13081510)).

Yet in 2024-2026 these companies mostly failed for financial reasons, not mechanical ones:

- Small Robot Company was liquidated in 2024 ([FarmingUK](https://www.farminguk.com/news/great-sadness-small-robot-company-put-into-liquidation_64099.html)).
- FarmWise closed in April 2025 ([Fertilizer Daily](https://www.fertilizerdaily.com/20250403-farmwise-to-close-operations-exploring-sale-of-its-ai-weeding-technology/)).
- Naïo entered receivership and was sold for about €170,000 ([Réussir](https://www.reussir.fr/fruits-legumes/un-repreneur-pour-les-robots-naio-technologies)).
- AgroIntelli filed for bankruptcy in early 2026 ([AgInsights](https://www.aginsights.blog/agrointelli-files-for-bankruptcy-as-pioneer-of-robotti-autonomous-field-robot-seeks-new-investor/)).

Economic modelling does favour small autonomous machines, especially on small or irregular fields, because each machine no longer needs an operator ([Lowenberg-DeBoer et al. 2020](https://link.springer.com/article/10.1007/s11119-019-09667-5)). But the supposed reduction in soil compaction "has not yet been tested in the field" ([Lowenberg-DeBoer 2022](https://onlinelibrary.wiley.com/doi/full/10.1002/aepp.13177)). Maintaining many units could also use up the labour saved ([Precision Farming Dealer](https://www.precisionfarmingdealer.com/articles/4405-is-your-farm-ready-for-the-swarm)). A shoebox rover must therefore earn its place on low unit cost and on doing a job that big robots physically cannot do.

## Pepper rows leave room, but the value sits next to the stem

### Every pepper system has a corridor; low branches cap the height

Row width is never the problem:

- Spanish pimentón: lines 0.73-0.76 m apart, plants 20-30 cm apart in the row ([UEx](https://www.unex.es/conoce-la-uex/estructura-academica/centros/eia/archivos/iag/2012/2012_10%20Avances%20en%20la%20mecanizacion%20del%20cultivo%20de%20pimiento%20para%20pimenton%20en%20la%20comarca%20de%20La%20Vera%20-Caceres.pdf); [MAPA](https://www.mapa.gob.es/app/MaterialVegetal/docs/cultivo%20de%20pimiento%20para%20piment%C3%B3n.pdf)).
- New Mexico chile: rows 76-102 cm apart ([NMSU H-230](https://pubs.nmsu.edu/_h/H230.pdf)).
- US fresh bell pepper: twin rows 30-46 cm apart on plastic-mulched beds set 152-198 cm centre to centre ([Missouri Extension](https://extension.missouri.edu/g6372); [Penn State](https://extension.psu.edu/pepper-production)).
- Hungarian soil-grown tunnel pepper: layouts such as 70 × 35 cm, or twin rows spaced 70 + 50 cm ([Szeged EFOP](https://eta.bibl.u-szeged.hu/5357/2/EFOP343_AP6MGK_olvas%C3%B3lecke_Led%C3%B3n%C3%A9_3_2021_03_15.pdf)).

Height is the problem. In machine-harvested chile the first fruit should set **10-25 cm above the ground** ([NMSU CTF13](https://pubs.nmsu.edu/research/horticulture/CTF13/index.html)). Bell and cherry types have wide fork angles and low, heavy fruit ([Walker, IPC 2018](https://conference.ifas.ufl.edu/PEPPER2018/presentations/Open_Session1/0955_Walker%20IPC%202018.pdf)). A 25 cm-tall rover would push through brittle branches late in the season; only a body of about 10-12 cm can pass under the drooping outer canopy.

Suppose a mature canopy is 40-60 cm wide (no source in the notes measures this). Then 75 cm rows leave roughly 15-35 cm of open soil by mid-season. A 15 cm body with a few centimetres of steering error needs 20-25 cm, so 75 cm rows are marginal. Rows of 90-100 cm and plasticulture row middles stay comfortably open.

| Pepper system | Corridor for the rover | Weeding left to do | Fit |
|---|---|---|---|
| Open field, 70-80 cm rows, bare soil (Hungary, Spain) | Wide early; ~15-35 cm and under 10-25 cm branches after mid-season (inferred) | In-row and near-stem weeds; tractor hoes already cover the inter-row | Good until canopies meet, then low-profile bodies only |
| New Mexico chile, 76-102 cm beds, furrow irrigation | Furrows open all season, but wet for days after each irrigation | Hand hoeing around thinned plants | Good geometry; value depends on a near-stem tool |
| US plasticulture, twin rows on mulched beds | Bare row middle ~50-90 cm all season (inferred) | Row middles and planting holes; the bed itself is plastic | Best geometry; the job is the row middles |
| Soil-grown tunnels | 60-90 cm paths; trained plants with bare lower stems | Planting holes and paths if mulched; everything if bare soil | Good geometry; no GNSS; strict hygiene |
| Substrate or hydroponic | n/a | No weeds | Not a market |

The ground surface matters as much as row width. Phytophthora guidance calls for dome-shaped beds at least 23 cm high ([UMass Extension](https://www.umass.edu/agriculture-food-environment/vegetable/fact-sheets/phytophthora-blight)), and plastic-mulch layers build beds 13-23 cm high ([WSU](https://smallfruits.wsu.edu/2026/01/05/mechanically-laying-biodegradable-plastic-mulch/)). A rover driving in the furrow therefore sits below the zone it must weed. Furrow-irrigated fields add muddy days when the rover cannot drive and steps of several centimetres between furrow and bed. Drip tape lying on bare soil is a cut-and-snag hazard for any blade.

### The critical period largely ends as the canopy closes

Pepper must be kept weed-free from roughly 1-2 weeks to 5-9 weeks after transplanting. The sources vary in the details:

- Florida guidance ends the critical period at early fruit set, 5-7 weeks after transplanting, or about 9 weeks where purple nutsedge is present ([UF/IFAS](https://ipm.ifas.ufl.edu/pdfs/Chapter6-Weed_Mgmt.pdf)).
- California work found that peppers needed at least 8 weed-free weeks for full yield ([California Agriculture](https://dx.doi.org/10.3733/ca.v045n01p11)).
- A Brazilian study at a 5% yield-loss threshold set the period at 11-89 days after transplanting; weeds left all season cost up to 85% of yield ([Planta Daninha](https://doaj.org/article/2679da1463d34004a90557382385cc5e)).
- A European review gives 25-45 days after planting ([EWRS](https://www.ewrs.org/en/documents/WGs-Weed-Management-Systems-in-Vegetables-Pannacci-et-al-weeds-and-weed-management-in-peppers.pdf)).

Direct-seeded chile needs longer, and late flushes still do damage. Spurred anoda emerging after thinning cut chile yield by **31-49%** ([Weed Technology 2023](https://www.cambridge.org/core/product/976115DABDDFAC0957092D6C02380C2F/core-reader)). Purple nutsedge left all season cost Florida bell pepper 30-70% of marketable yield ([Proc. Fla. State Hort. Soc.](https://journals.flvc.org/fshs/article/view/86512)), and nutsedge pierces plastic mulch ([EWRS](https://www.ewrs.org/en/documents/WGs-Weed-Management-Systems-in-Vegetables-Pannacci-et-al-weeds-and-weed-management-in-peppers.pdf)).

The awkward implication is that most of the critical period falls before the canopy closes, while tractor cultivators and hand crews can still get in. The work that genuinely needs an under-canopy robot is the tail: late flushes, nutsedge suppression and stopping weeds from setting seed. For pepper, "works under the canopy" is the secondary selling point. "Works close to the stem without herbicide" is the main one.

The crop also limits the tool and the robot:

- **Depth.** Cultivating 9 cm deep cut bell pepper productivity compared with 3 cm, by pruning roots and drying the soil ([HortScience](https://journals.ashs.org/downloadpdf/view/journals/hortsci/22/3/article-p418.pdf)). Any soil-engaging tool must stay within about 2-3 cm of the surface.
- **Disease.** The robot itself can spread disease. Tobamoviruses such as ToBRFV travel on tools, equipment and trellis ropes ([EPPO](https://gd.eppo.int/taxon/TOBRFV/datasheet)) and survive on greenhouse surfaces for up to six months ([Biobest](https://academy.biobestgroup.com/blogs/news/tobrfv-update)). *Phytophthora capsici* moves in soil and water and survives more than ten years ([UVM Extension](https://www.uvm.edu/vtvegandberry/factsheets/phytophthora.html)). A low rover brushing wet pepper leaves acts like a worker's gloved hand. It must avoid touching foliage and be washable between fields and tunnels.

### Inter-row weeding is cheap; near-stem weeding is what growers pay for

Weed control in pepper splits into two jobs. The inter-row job is cheap: tractor hoes and herbicides already handle it until the crop is laid by. The in-row job is expensive and done by hand:

- New Mexico chile needs hand hoeing for weeds no other method controls ([NMSU CR548](https://pubs.nmsu.edu/_circulars/CR548/index.html)), and that hoeing is "often prohibitively expensive" ([Weed Science](https://www.cambridge.org/core/product/5D10A731699A1159652287ED784797D0)).
- False seedbeds alone cut hoeing costs by **$262-440 per acre, about $650-1,090/ha** ([HortTechnology 2021](https://agris.fao.org/search/en/records/65df6bf27c7033e84beda4ac)).
- Hand weeding of UK field vegetables costs **£1,000-2,000/ha**, sometimes twice a season ([EIP Wales](https://businesswales.gov.wales/farmingconnect/sites/farmingconnect/files/documents/EIP%20Wales_Robotic%20weeder%20final%20report.pdf)).

Plasticulture adds a second niche: the bare row middles. Glufosinate may be applied there at most twice a season. In an organic bell pepper trial, every row-middle treatment tested cut total yield by about 30% compared with weed-free cultivation ([Specialty Crop Grower](https://specialtycropgrower.com/specialty-crop-grower-magazine-weeds-plasticulture-row-middles)).

The inference is blunt. A robot that only weeds the open inter-row competes with a tractor pass worth tens of euros per hectare. A robot that weeds within a few centimetres of stems competes with hundreds to two thousand per hectare of hand labour. No source measures how hand-weeding hours split between the inter-row and the in-row zone, so this is a judgment. It is still the single most important design driver.

Market size tempers the paprika case specifically. Hungarian spice paprika shrank from 9,000 ha in 1990 to **1,062 ha** in 2023 ([Telex](https://telex.hu/gazdasag/2024/12/02/egyre-kevesebbet-terem-a-szegedi-fuszerpaprika)), and Kalocsa is down to about 310 ha ([Világgazdaság](https://www.vg.hu/vilaggazdasag-magyar-gazdasag/2025/10/kalocsai-fuszerpaprika-ermeles)). Green chillies and peppers cover about 2.1 Mha worldwide ([IndexBox](https://www.indexbox.io/blog/chili-and-pepper-world-market-overview-2024-5/)), but most of that is in countries with cheap labour or plasticulture.

The same geometry logic sorts other row crops:

- **Persistent corridor.** Wide-row crops whose lower stems clear as they grow: maize and sunflower on 70-80 cm rows ([Bednar](https://www.bednar.com/en/blog/which-weeder-to-choose-for-your-farm/)), wide-row soybean, staked vegetables and vineyards. Maize and sorghum are where under-canopy robots have actually logged distance ([arXiv 2404.17718](https://arxiv.org/abs/2404.17718)).
- **Pre-closure window only.** Crops that carpet the ground: sugar beet on 45-50 cm rows ([Bednar](https://www.bednar.com/en/blog/which-weeder-to-choose-for-your-farm/)), brassicas, lettuce, carrots, and potatoes once the haulm spreads. In that window, camera-guided tractor hoes already work ([Agriculture 8(4):54](https://www.mdpi.com/2077-0472/8/4/54/html)).
- **Pepper sits in between.** Its rows are wide enough, but its low branches force a low profile.

Commodity crops offer the best geometry but the least value per hectare of weeding, so the economics point back to vegetables. This classification is a judgment from general agronomy; most per-crop dimensions behind it were not verified against sources.

## Physics lets the rover drive, but not hoe

### A 1-5 kg robot pulls only 2-25 newtons

Classical Bekker-Wong terramechanics is the wrong model here. It loses accuracy for wheels below about 50 cm in diameter and loads below about 45 N ([ISTVS](https://istvs.org/publication-news/2012/7/3/a-pressure-sinkage-model-for-small-diameter-wheels-on-compactive-deformable-terrain)). Applied naively, it predicts sub-millimetre sinkage that tilled soil will not deliver. Practical numbers have to come from first-order bounds instead.

Assume net traction of 0.2-0.5 times the robot's weight on workable soil (an assumption):

| Robot mass | Net pull at 0.2 / 0.35 / 0.5 × weight (calculated) |
|---|---|
| 1 kg | 2.0 / 3.4 / 4.9 N |
| 3 kg | 5.9 / 10.3 / 14.7 N |
| 5 kg | 9.8 / 17 / 24.5 N |

Tool draft rises steeply with depth. A hoe share needed 63% more force going from 30 to 50 mm deep, and a further 71% going to 70 mm ([Acta Agric. Scand.](https://informahealthcare.com/doi/full/10.1080/09064710.2017.1367837)). A textbook soil-cutting formula (recalled, not checked against a source this session) puts a 15 cm-wide blade at 2 cm depth at:

- 2-5 N in loose soil;
- 20-40 N in moderately cohesive soil;
- 74-150 N in moist or crusted soil.

A full-width blade therefore works only on loose, freshly tilled soil, which is exactly when weeds matter least.

What stays within reach:

- a single narrow tine or blade, 1-2 cm wide and 1-2 cm deep (about 0.2-20 N depending on soil);
- rotary and string tools, which push back as torque rather than as drag;
- stamping or punching, capped at roughly the robot's own weight (10-50 N per stroke) unless energy is stored in a spring or flywheel.

Any sideways tool force of more than a few newtons will also steer a 15 cm-track robot off course.

### Ten-centimetre wheels and a floor-mounted battery keep it moving

Every small robot with real field mileage uses wheels. Nothing found supports legs or whegs on farm soil at this scale.

Size the wheels to the obstacles. By rule of thumb, a wheel should be at least 2-3 times the height of the largest obstacle. Clods of 3-5 cm and drip tape of 1.5-2 cm therefore call for 10-15 cm wheels and at least 5 cm of ground clearance. Inside a 10-25 cm height envelope, that leaves only 5-15 cm for body, battery and electronics.

Choose the drive layout for loose soil:

- Grousers that bite the soil ahead of the wheel deliver more than 80% of the benefit of a fully grousered wheel ([ISTVS](https://www.istvs.org/publication-news/grousers-improve-drawbar-pull-by-reducing-resistance-and-generating-thrust-at-the-front-of-a-wheel)).
- Skid-steer turns on loose soil cost extra power through bulldozing and sinkage, which hard-ground models leave out ([Fiset et al.](https://easychair.org/publications/preprint/MsZP)).
- So two large drive wheels with a caster, or slightly cambered wheels like Tertill's, should beat a short-wheelbase skid-steer on both scrubbing and odometry.

Ground pressure is only about 2-6 kPa, so compaction is not a concern. The real failure modes are:

- wheels digging themselves in under slip;
- getting stuck on clods and bed crowns;
- wet clay packing the grousers;
- tangling in crop residue, mulch edges or drip tape.

Stability is adequate if the battery sits in the floor:

- With an 11-13 cm track and the centre of gravity 6 cm up, the static tip angle is 43-47°.
- With the centre of gravity at 10-12 cm, it falls to 25-33°.
- One wheel dropping 5 cm off a furrow edge rolls the body about 21°.

Driving centred in a flat furrow bottom is therefore safer than riding the bed shoulders. None of this has been field-tested at 15 cm width. The most important missing number is how often a robot this size gets stuck per kilometre on wet, cloddy vegetable soil.

### Batteries suffice; solar under canopy and heat in tunnels do not

The electrical budget is modest:

- **Raspberry Pi 5:** about 3 W idle and 8.8 W with all cores loaded ([raspberry.tips](https://raspberry.tips/en/faq/raspberry-pi-power-consumption-update-2026-all-models-compared)).
- **Jetson Orin Nano:** 7 W and 15 W modes, plus a 25 W mode added with JetPack 6.2 ([NVIDIA](https://developer.nvidia.com/blog/nvidia-jetpack-6-2-brings-super-mode-to-nvidia-jetson-orin-nano-and-jetson-orin-nx-modules)); a third-party benchmark measured 8-12 W during typical inference ([edgeaistack](https://edgeaistack-site.pages.dev/blog/jetson-orin-nano-power-consumption/)).
- **Driving** a 3 kg robot at 0.3 m/s: about 2-4 W with no tool, about 10 W against 10 N of tool draft (calculated).
- **Totals:** a lean Pi 5 build draws roughly 8-12 W; an Orin Nano build with an intermittently run rotary tool draws 17-40 W.

A 100 Wh pack is about six 21700 cells at roughly 247 Wh/kg ([Batemo](https://www.batemo.com/?p=113656)), about 0.4 kg of cells. It runs about 4 h at 25 W or 8 h at 12 W, enough for a working day. On a robot under 3 kg, though, a 150 Wh pack would already be a third of the mass.

Solar is the trap:

- A 15 × 20 cm panel gives about 6 W peak, or roughly 30 Wh on a clear summer day in the open.
- Under a closing canopy that lets 10-30% of light reach the soil, it gives about 3-9 Wh/day (calculated). No measured transmission value for pepper was found, and pepper canopies are uneven ([Kim et al. 2016](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5016622/)).
- That matches Tertill's 1-2 h/day of short bursts in an open garden ([AgTecher](https://agtecher.com/product/tertill-robot)), not multi-hour field work.
- Large solar weeders work because they carry 205-350 W panels ([PR Newswire](https://prnewswire.com/news-releases/aigen-unveils-the-worlds-first-ai-driven-solar-powered-agricultural-robotics-service-301868463.html); [Fresh Consulting](https://www.freshconsulting.com/work/element-gen-2/)).

A small rover therefore needs charging docks at the headland or swappable packs. A hectare holds about 13 km of inter-row, so the trip to a row-end dock takes only minutes.

Heat and sealing are underrated engineering risks:

- Polytunnels can reach 42 °C before they are vented ([Allotment Garden](https://www.allotment-garden.org/garden-diary/5235/polytunnel-temperatures/)).
- Every Raspberry Pi board throttles at 80 °C chip temperature ([Raspberry Pi](https://www.raspberrypi.com/news/heating-and-cooling-raspberry-pi-5/)).
- The Compute Module 5 is rated from −20 to +85 °C ([CM5 brief](https://pip-assets.raspberrypi.com/categories/944-raspberry-pi-compute-module-5/documents/RP-008181-DS-5-cm5-product-brief.pdf)).

A 5-15 W computer in a sealed IP65-IP67 box of a few litres, sitting in the sun, will need one or more of these (engineering judgment):

- a conductive heat path to a chassis plate;
- inference run only part of the time;
- night or early-morning operation.

Lithium-ion packs should be charged in the shade.

### Coverage, not energy, sets the fleet size

A 15 cm swath at 0.1-0.5 m/s sweeps only 54-270 m²/h. A hectare at 0.75 m row spacing holds 13.3 km of inter-row, so one pass down every inter-row at 0.3 m/s takes 12.3 h of pure driving (calculated). The result depends on the operating mode. In full-width mode the rover cultivates its whole inter-row, which takes several passes. In targeted mode it drives each inter-row once and treats only what its cameras see. All figures in the table are calculated, assuming 10 h of work a day and that 75% of time is spent working rather than turning or docking.

| Mode | Area rate | ha per day | Robots per ha, 5-7-day revisit |
|---|---|---|---|
| Full-width cultivation (3-7 passes per inter-row) | 36-202 m²/h | 0.04-0.2 | ~1.2-1.9 at 0.3 m/s; 3.5-5.6 at 0.1 m/s |
| Targeted treatment (1 pass per inter-row) | 110-1,350 m²/h | 0.1-1.35 | ~0.25-0.6 at 0.3 m/s |

A critical period of 3-9 weeks with a 5-10-day revisit means 4-12 passes a season. Energy per hectare is small: about 150 Wh per targeted pass and 740 Wh per full-width cultivation at 12 W. A 100 Wh robot therefore treats about 0.6 ha per charge in targeted mode.

Large machines are 10-100 times more productive per unit:

- FarmDroid claims up to 6.5 ha/day ([Future Farming](https://futurefarming.com/farmdroid-fd20-automatic-seeding-and-weeding)).
- A Greenfield robot was measured at about 2.8 ha/day in a Vermont pilot ([UVM Extension](https://www.uvm.edu/extension/nwcrops/news/recap-first-public-demonstration-autonomous-weeding-robots-vermont)).
- Odd.Bot takes up to 16 h per hectare ([Future Farming](https://futurefarming.com/odd-bot-maverick)).

The micro-rover makes sense only if all three of these hold:

- Units cost hundreds rather than thousands of dollars. A rough, unsourced parts estimate in the notes is $300-900 per unit at low volume, against TerraSentia's $4,999 launch price ([CSL Illinois](https://csl.illinois.edu/news/teachable-ultra-compact-autonomous-phenotyping-robot-introduced-investors-market)).
- It runs long days with automatic charging.
- It treats weeds near plants rather than cultivating whole inter-rows.

## Cameras can steer and see weeds; switching rows is unsolved

### GNSS is a geofence, not a guide

Under a canopy, satellite positioning degrades badly:

- A small robot in sorghum rows saw **RTK errors above 0.4 m even with a high-end Trimble antenna**, plus drift ([Higuti et al. JFR preprint](https://terra-mepp.illinois.edu/sites/terra-mepp.illinois.edu/files/2021-06/JFR_under_canopy_lidar_preprint.pdf)).
- RTK under tree canopy reported "fixed" solutions 88.2% of the time, yet a horizontal RMSE of 2.03 m. These are wrong fixes that look valid ([RTK under tree canopy](https://www.researchgate.net/publication/391566844_Evaluation_of_the_Repeatability_and_Accuracy_of_RTK_GNSS_under_Tree_Canopy)).

A 15 cm rover will do worse, because its antenna sits at 10-25 cm, under the foliage and next to wet soil.

GNSS cannot tell which row the robot is in (that needs error below half the row spacing), and it cannot steer within the row. It can:

- fence the field;
- tell which headland the robot is at;
- re-anchor the robot's position in open headlands and early in the season;
- guide it back to a dock.

For that role, a dual-frequency Quectel LC29HEA at about $58 with antenna is enough ([rtklibexplorer](https://rtklibexplorer.wordpress.com/2024/04/28/dual-frequency-rtk-for-less-than-60-with-the-quectel-lc29hea/)). A u-blox ZED-F9P board, listed at $259.95 by SparkFun ([SparkFun](https://www.sparkfun.com/sparkfun-gps-rtk-sma-breakout-zed-f9p-qwiic.html)), pays off only if open-sky RTK at the headland matters.

Inside polytunnels with ETFE film, even plain (non-RTK) GNSS reaches decimetre level ([ResearchGate](https://www.researchgate.net/publication/401524977_A_Study_of_Standalone_GNSS_Positioning_Precision_in_Covered_Agricultural_Environments_ETFE_Polytunnels_Versus_Glass_Greenhouses)). Metal hoops still cause signal reflections. AprilTags (printed fiducial markers) on hoops or pillars are the proven greenhouse alternative ([COMPAG](https://www.sciencedirect.com/science/article/abs/pii/S016816992200607X)).

### Keypoint row-following transfers best to a low camera

The strongest camera-only evidence is the University of Illinois CropFollow line:

- **CropFollow** ran ResNet-18 on 320 × 240 images from a single front RGB camera, predicting the robot's heading and offset in the row. It achieved **485 m per human intervention, against 286 m for a state-of-the-art LiDAR system**, over more than 25 km ([RSS 2021](https://www.roboticsproceedings.org/rss17/p019.pdf); [ApproxCaliper](https://proceedings.mlsys.org/paper_files/paper/2023/file/89efa87dc8f0a5d18e4ae0a479658f60-Paper-mlsys2023.pdf)).
- **CropFollow++** instead predicts three keypoints (the vanishing point and where the left and right rows meet the image) and uses IMU roll and a confidence signal for occlusion. It cut late-season collisions from 33 to 13 over about 1.9 km ([RSS 2024](https://roboticsconference.org/2024/program/papers/23/)). It was deployed for 25 km on cover-crop robots, though most runs ended within 250 m ([arXiv 2404.17718](https://arxiv.org/abs/2404.17718)).
- **LeCropFollow** (2026) runs at 20 Hz on a Jetson Orin Nano and cut failures in planting gaps by a factor of 2.4 ([arXiv 2606.31941](https://arxiv.org/html/2606.31941v1)).
- **AdaCropFollow** adapts on board to new fields without labels, but was demonstrated on a much bigger AGX Orin ([Uni Bonn](https://www.ipb.uni-bonn.de/wp-content/papercite-data/pdf/sivakumar2026iros.pdf)).

Other approaches fit this rover less well:

- Bonn's two-camera visual servoing reaches 3.82 cm accuracy across five crops, but assumes a view from above the canopy ([BonnBot-I](https://arxiv.org/abs/2109.11936)).
- Segmentation-based row following still needs human help across gaps longer than 1 m ([de Silva et al.](https://arxiv.org/pdf/2309.11989)).
- Classical Hough-transform row detection runs at 30 FPS on a Pi 3B, and a learned ONNX row model at about 10 FPS on a Pi 5 ([PMC review](https://pmc.ncbi.nlm.nih.gov/articles/PMC11558879/)). That is enough at 0.1-0.3 m/s.

CropFollow++'s setup matches this rover best: a forward camera under the canopy, looking down the row. But every model was trained on corn, sorghum or soy from a higher camera, and nothing has been published on under-canopy row following in pepper or with a camera 10-20 cm off the ground.

There is a real tension here. The navigation camera wants height, while branch clearance pushes the whole body below about 12 cm.

Mulched raised beds make the problem easier. The bed walls and mulch edges form two high-contrast lines meeting at a vanishing point. Furrow lines have been detected 98% of the time with 3.8° angle error ([ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S1881836616301094)). Accuracy on mulch can drop sharply at certain times of day ([COMPAG](https://www.sciencedirect.com/science/article/abs/pii/S016816992100507X)).

### Row ends and headland turns are the weakest link

Even the University of Illinois 25 km commercial deployment did lane turns **by hand** ([arXiv 2404.17718](https://arxiv.org/html/2404.17718v1)). The other evidence is thin:

- The learned turning policy "Learning to Turn" exists only in simulation ([arXiv 2408.03059](https://arxiv.org/pdf/2408.03059)).
- A vision-based, field-scale row-switching system reached 92.5% coverage with 2.5% overlap in simulation. Its errors were dominated by rotation during switching and by headland moves that relied on wheel odometry ([arXiv 2309.11989](https://arxiv.org/pdf/2309.11989)).
- A row-exit method averaged 24 cm error and used a depth camera, which an RGB-only rover lacks ([arXiv 2306.05869](https://arxiv.org/abs/2306.05869)).
- Skid-steer odometry error spikes in turns and on loose ground ([Fictionlab](https://fictionlab.pl/blog/differential-drive-or-skid-steering-trade-offs-you-feel-only-in-the-field/)).

General visual SLAM is no rescue. On the 7.3 km Rosario soybean dataset, ORB-SLAM3 "incorrectly closes 7 loops and the estimated trajectory gets irreversibly corrupted" ([Rosario v2](https://arxiv.org/pdf/2508.21635)).

The pragmatic answer is engineered aids:

- a coded AprilTag or coloured stake at each row mouth, a technique already used for row transitions ([ResearchGate](https://www.researchgate.net/publication/357001386_Minimum-time_row_transition_control_of_a_vision-guided_agricultural_robot));
- pivot turns controlled by IMU yaw (the body spins in a 25 cm circle);
- a known row spacing;
- re-entry accepted only when the row-keypoint confidence recovers;
- a topological map that records only row number and distance along the row, instead of full metric SLAM.

Visual-inertial odometry with loop closure switched off can bridge short gaps. Monocular ORB-SLAM3 runs at 18-20 FPS on a Pi 5 but leaves little CPU for anything else ([ResearchGate](https://www.researchgate.net/publication/400559478_An_In-depth_Evaluation_of_ORB-SLAM3_on_the_Raspberry_Pi_5_Performance_Stability_and_Design_Guidelines_for_Embedded_SLAM)). A GPU version reaches 32 FPS with camera plus IMU on an Orin Nano ([arXiv 2608.17874](https://arxiv.org/html/2608.17874)).

The first field experiment should test turn reliability, not in-row following.

### Detect the pepper and treat everything else green

Weed perception becomes tractable once the problem is inverted: find the crop, not the weeds.

On bare soil between rows, simple green-on-brown colour thresholding works on a Pi with no accelerator. OpenWeedLocator runs it at 416 × 320 on a Raspberry Pi 4 ([Scientific Reports](https://springerlink.bibliotecabuap.elogim.com/content/pdf/10.1038/s41598-021-03858-9.pdf)). Its reported performance:

- about **79% precision and 52% recall** across fallow fields, rising to 92% and 74% at good sites ([CAWS 2022](https://caws.org.nz/wp-content/uploads/2023/02/0218.pdf));
- **95.7% recall at 5 km/h with a global-shutter camera**, against 26-31% for rolling-shutter Pi cameras at 30 km/h ([arXiv 2305.10311](https://arxiv.org/pdf/2305.10311)).

Shadows are the main documented failure ([Precision Agriculture](https://link.springer.com/article/10.1007/s11119-017-9511-z)). Under a canopy, low pepper leaves and fallen green leaves would also read as weeds.

The workable design pairs a colour mask for "all green" with a learned detector for the one class that must never be hit: the pepper plant and its stem, protected by a keep-out radius. This mirrors two existing approaches:

- Tertill's height rule;
- Bonn's zero-shot "Weeds are Weird" method, which assumes the most common plant is the crop ([Chong et al. 2025](https://www.ipb.uni-bonn.de/wp-content/papercite-data/pdf/chong2025iros.pdf)).

Transplanted pepper helps: each plant has a single upright stem on a known line at a known spacing.

The hard remainder is weeds touching the stem:

- *Solanum nigrum* (black nightshade) is a relative of pepper that is hard to identify ([UC IPM](https://ipm.ucanr.edu/weeds-identification-gallery/black-nightshade/)).
- The only pepper weed dataset notes that some weeds "look almost identical" to the crop ([AmeriGEOSS](https://data.amerigeoss.org/dataset/weeddetection_hotpeppers)).
- Locating the stem matters more than a high detection score; one study found that "merely achieving high mAP does not guarantee effective stem localization" ([arXiv 2502.06255](https://arxiv.org/pdf/2502.06255)).

Data is a bigger gap than models. Two useful datasets exist:

- a pixel-labelled Trinidad hot-pepper set ([AmeriGEOSS](https://data.amerigeoss.org/dataset/weeddetection_labelled_hotpeppers));
- CropAndWeed, with 74 species and about 112,000 labelled plants including stem positions ([WACV 2023](https://openaccess.thecvf.com/content/WACV2023/papers/Steininger_The_CropAndWeed_Dataset_A_Multi-Modal_Learning_Approach_for_Efficient_Crop_WACV_2023_paper.pdf)).

No public dataset was captured from 5-25 cm under a canopy.

Small amounts of labelled data go a long way:

- In a 2026 cross-crop study, fine-tuning on as few as 25 labelled images from the target field beat unsupervised domain adaptation ([arXiv 2608.21254](https://arxiv.org/pdf/2608.21254)).
- Zero-shot Grounding DINO scored only 12.6 mAP on PhenoBench crop-versus-weed, against 38.2 after adapting on 24 images ([arXiv 2504.07252](https://arxiv.org/html/2504.07252v1)).

Large foundation models are therefore offline auto-labelling tools, not on-board classifiers. The data plan follows:

1. Film from the rover's own camera.
2. Auto-label with a generic "plant" prompt plus row-geometry rules.
3. Hand-correct tens to a few hundred frames, spread across growth stages.

### A Pi 5 needs an accelerator; an Orin Nano runs the whole stack

| Hardware | Reported YOLO speed | Realistic role |
|---|---|---|
| Pi 5 CPU | YOLO26n at 67 ms per image (NCNN, inference only) to 128 ms ([Arm](https://learn.arm.com/learning-paths/embedded-and-microcontrollers/yolo26-arm-runtime/compare-performance/); [Ultralytics](https://docs.ultralytics.com/guides/raspberry-pi)) | Colour mask plus one small detector at low resolution; navigation or detection, not both plus visual odometry |
| Pi 5 + Hailo-8L | YOLOv8s at 80.7 FPS inference only, ~30 FPS in a full 1080p camera pipeline ([Seeed](https://wiki.seeedstudio.com/benchmark_on_rpi5_and_cm4_running_yolov8s_with_rpi_ai_kit/)); 28.6 FPS on PCIe Gen 2 vs 55.7 FPS on Gen 3 ([Hailo community](https://community.hailo.ai/t/issue-with-yolo-fps-on-raspberry-pi-5/11706)) | Detector plus segmentation at camera frame rate |
| Jetson Orin Nano (Super) | YOLOv8n at 3.5-4.4 ms, model only ([benchmark](https://github.com/hokwangchoi/jetson-orin-nano-benchmarks)); ~35 FPS in one user's full pipeline ([Reddit](https://redlib.belloworld.it/r/JetsonNano/comments/1e187cc/yolov8_performances_on_jetson_orin_nano_4gb)) | Row-keypoint network, weed detector and visual odometry at the same time |

Every tier is fast enough at 0.1-0.3 m/s. What decides the choice is accuracy in the field and total delay from capture to action.

Low-precision (8-bit integer) quantisation must be checked on field data. In one Orin Nano YOLO11 test, accuracy (mAP50-95) collapsed from 0.61 at 16-bit to 0.32 with naive 8-bit conversion ([CamThink](https://camthink-ai.github.io/wiki-documents/en/docs/NeoEdge%20NG4500%20Series/Application%20Guide/Object%20Detection)).

For a rover that both navigates and weeds, the Orin Nano is the safer default. A Pi 5 with a Hailo board is the frugal, lower-power alternative.

At low weed densities, the computer may dominate the energy bill. At 5-15 W, 0.1 m/s and a 15 cm swath, computing costs 3.3-10 MJ/ha (calculated). That is the same order as the 14.4 MJ/ha that Coleman and colleagues allotted to imaging in every site-specific method ([AHRI](https://www.ahri.uwa.edu.au/?p=16169)).

### A low camera gets resolution for free; blur, light and height errors cost

Resolution is not the problem at this height. A Camera Module 3 Wide (102° × 67° field of view; [Raspberry Pi](https://raspberrypi.com/products/camera-module-3)) 10 cm above the ground sees about 25 × 13 cm at roughly 0.4 mm per pixel, even downscaled to 640 px. Below about 10 cm it sees less than the chassis footprint (calculated).

Motion blur, exposure and height errors are the problems:

- **Blur.** An 8 ms exposure, plausible under a canopy without lights, smears a weed across about 10 pixels at 0.5 m/s. The rover must either crawl at 0.1-0.2 m/s or use strobed LEDs for exposures of 1 ms or less inside a shroud. The shroud also removes the shadows that break colour thresholds.
- **Time in view.** At 0.5 m/s a weed stays in view for only 0.26 s, about one frame on a Pi 5 CPU.
- **Height errors.** Converting a single camera's image to ground positions is sensitive at this height. With the camera 10 cm up, a 1 cm change in its height, from a clod or from pitching, shifts a target 10 cm off-centre by about 1 cm. A leaf tip 2 cm above the soil and 8 cm off-centre can land 2 cm off. That is acceptable for a hoe or a stamp, but not for a laser aimed at a growing point.

(All figures in this list are calculated.)

Tracking each weed between frames matters more than raw detection accuracy:

- A laser weeder rose from 47% to 72% weed elimination simply by adding DeepSORT tracking ([Mwitta et al. 2024](https://www.frontiersin.org/articles/10.3389/fagro.2024.1388452/full)).
- Fusing wheel-encoder odometry cut the positioning error of an in-row hoe from 9.88 to 6.06 mm ([Bohrium](https://bohrium.dp.tech/paper/arxiv/600d1eeeb42d2a8a0a834feb)).
- The low-cost Pi 5 laser robot LiteWeed copes by stopping and aligning over each weed, which limits it to about 0.01-0.03 ha/day ([Global Agriculture](https://www.global-agriculture.com/ag-tech-research-news/low-cost-laser-weeding-robot-built-for-smallholder-farms-passes-field-tests-in-canada-and-tanzania/)).

## Only low-force tools on tiny weeds will work

### Energy rules out heat; force rules out hoes

Coleman and colleagues' energy comparison frames the choice ([Coleman et al. 2019](https://www.cambridge.org/core/product/CE8E572179F660512A09F5490094189E)). Treating weeds one at a time, hoeing, electrocution, lasers and herbicides all need roughly 15-20 MJ/ha (lasers up to about 250 MJ/ha). Broadcast flaming needs 1,008-4,334 MJ/ha. At 5 weeds/m², with half of a 100 Wh pack going to the tool, the rover can spend tens to a couple of hundred joules per weed (calculated).

That rules out:

- hot water: about 0.35 kJ per mL heated, so 0.7-3.5 kJ per weed (calculated);
- microwave: 77-500 J per cm² of seedling ([CORE](https://core.ac.uk/download/162219343.pdf));
- UV: up to 100,000 MJ/ha ([thermal review](https://www.cambridge.org/core/product/65ADA897B679D840205CDD3843C13A33/core-reader));
- broadcast flame;
- grit blasting. It works (0.5 g of coffee grounds at 690 kPa killed waterhemp seedlings, [Weed Technology](https://resolve.cambridge.org/core/journals/weed-technology/article/spent-coffee-grounds-as-airpropelled-abrasive-grit-for-weed-control-in-organic-production/30C89F771A7B7F122E6FB60A910C379D)), but it would need kilograms of grit and a compressor.

Add the 2-25 N force ceiling, and the options shrink to five families of tool.

The biology favours treating small weeds often:

- **Uprooting kills young seedlings.** Seedlings are most vulnerable from the white-thread stage (sprouted, not yet green) to their first true leaves, and uprooting is the main way they die. Drier soil raised the death rate of uprooted seedlings from **36% to 91%** ([AGRIS](https://agris.fao.org/search/en/records/669f6943a4dac527f930bb92)).
- **Burial is out of reach.** Burying needs about 6 cm of soil ([OAPEN](https://library.oapen.org/bitstream/20.500.12657/61502/1/9781801462464_web.pdf)), impossible at this scale.
- **Cutting suppresses but does not kill.** Raising the cutting height from 5 to 8 cm increased regrowth by 100-400% ([Weed Technology 2002](https://complete.bioone.org/journals/weed-technology/volume-16/issue-4/0890-037X(2002)016[0873:ROWSAC]2.0.CO;2/Regrowth-of-Weed-Species-after-Cutting1/10.1614/0890-037X(2002)016[0873:ROWSAC]2.0.CO;2.full)), and annual broadleaves suffer far more than grasses ([HAL INRAE](https://hal.inrae.fr/hal-02661671)).

A rover that returns every 1-3 days catches weeds in that vulnerable window. One that returns weekly faces plants at two leaves or more, and needs several times the energy or force per weed.

| Rank | Concept | Battery energy per weed (calculated) | Works on | Main risks |
|---|---|---|---|---|
| 1 | Untargeted shallow soil disturbance: grouser or spike wheels plus a light finger-tine or rake comb every 1-3 days, with a small string trimmer for escapes | Driving energy only | White-thread to cotyledon (seed-leaf) weeds on dry surface soil | Cannot tell crop from weed in the row; weak on grasses and on wet soil; trimmed weeds regrow |
| 2 | Targeted micro-stamp or punch, 5-11 mm, energy stored in a spring or flywheel | ≤40 J | Small seedlings of all types; crushes and buries the growing point | The light body must absorb the stroke; needs 2-5 mm aim; no force data |
| 3a | Shrouded 445-450 nm blue diode laser, 5-20 W optical | ~11-40 J electrical; 0.2-2 s per weed | Cotyledon to 2-leaf | Class 4 eye and fire hazard; reflections off mulch; dose uncertain |
| 3b | Chemical micro-dose (contact or systemic herbicide) | ~0 J; 100 mL treats about 10⁴-10⁵ weeds | Systemic: all stages, including grasses. Pelargonic acid: young broadleaves | Crop authorisation, banned in organic, disastrous if it hits pepper |
| 4 | Pulsed "microshock" electrode | ~6-10 J per broadleaf, ~250-330 J per grass seedling | Broadleaf seedlings; kills roots | Safe distance from the crop unknown; wet soil; high-voltage certification |
| 5 | String trimmer alone | Low | Suppression only | Abrades crop stems; grasses regrow |
| — | Thermal, UV, microwave, grit | 0.2 kJ to tens of kJ, or kilograms of consumables | — | Energy, mass, fire, mulch damage |

### Untargeted shallow disturbance is the workhorse; targeted tools handle the stem zone

**Untargeted disturbance** extends what Tertill did. Its grousers caused "considerable shallow soil disturbance, possibly resulting in the uprooting or burial of weed seedlings" ([Weed Technology](https://www.cambridge.org/core/product/07DF9F4329CB64D749E822168A097D92)). Yet the wheels alone achieved only 16-29% control. On this rover, untargeted disturbance has to be:

- aggressive: spiked wheels plus a dragged comb no deeper than 2 cm;
- frequent: every 1-3 days;
- timed for dry surface soil: afternoons, never just before irrigation.

Even then, it cannot touch weeds inside the crop row.

**The micro-stamp** is the best targeted option that needs no consumables. The Langsenkamp tube stamp used an 11 mm plunger driven by a 70 W brushless motor and handled up to 1.75 weeds per second ([CIGR 2014](https://www.hs-osnabrueck.de/fileadmin/HSOS/Homepages/COALA/Veroeffentlichungen/2014-CIGR_2014_Tube_Stamp_for_mechanical_intra-row_individual_Plant_Weed_Control.pdf)), which works out to at most 40 J per weed. BoniRob's 1 cm stamp drove weeds about 3 cm into the soil ([IEEE Spectrum](https://spectrum.ieee.org/bosch-deepfield-robotics-weed-control)), and Odd.Bot punches small weeds into the ground rather than pulling them ([Future Farming](https://futurefarming.com/tech-in-focus/field-robots/odd-bot-aims-to-excel-in-early-stage-weed-removal)).

On a 1-5 kg robot the stamp would need to be smaller: about 5-8 mm, 1-2 cm deep, with a spring latch recharged between strikes by a 5-15 W motor. It crushes and buries the growing point in a footprint of about a square centimetre and leaves no stub to regrow, making it the safest mechanical action within 2-5 cm of a pepper stem. No published force-per-weed figure exists, so the first bench test should measure it.

**Lasers** are energy-feasible, but their dose is poorly pinned down:

- A 97 W, 445 nm diode array controlled small seedlings at only 0.2-0.5 J/mm² ([Walsh et al. 2025](https://www.cambridge.org/core/product/4E12FA1EC168FC4E223573F0D38EEF00/core-reader)).
- A focused 2 µm thulium laser on the growing point needed up to 12.7 J/mm² ([Andreasen et al. 2024](https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2024.1375164/full)).
- A review cites about 20 J/mm² for cotyledon-to-two-leaf weeds ([Frontiers in Agronomy 2024](https://www.frontiersin.org/articles/10.3389/fagro.2024.1342372)).

The 25- to 100-fold gap probably reflects different dose definitions and how strongly each wavelength is absorbed. It is not resolved, and a design should not average across it.

Practical results show the window is narrow. In a compact device, 1 W "was not sufficient" and needed "too long exposure time", while 5 W "harmed the crop if the laser beam became split" ([Rakhmatulin & Andreasen](https://www.mdpi.com/2073-4395/10/10/1616)). A 60 W, 450 nm delta-robot prototype reported 90% weed mortality, but that comes from a presentation, not a peer-reviewed paper ([SARE](https://projects.sare.org/media/pdf/L/a/s/LaserWeeding_Muneeb.pdf)). Every such source is Class 4 under IEC 60825-1 ([HSE Blog](https://www.hseblog.com/laser-safety/)). That means a light-tight shroud closing to the ground, interlocks, firing only on camera-confirmed plant pixels, and a hard cap on dwell time over plastic mulch and drip tape.

**Chemical micro-dosing** is by far the lightest and most energy-efficient option:

- In pots, as little as 7.6 µg of glyphosate or 0.15 µg of iodosulfuron per plant controlled weeds ([Utstumo PhD](https://tomgra.folk.ntnu.no/Trygve%20Utstumo_PhD.pdf)).
- In a carrot field, 5.3 µg of glyphosate per droplet controlled weeds with a ten-fold cut in herbicide use ([Asterix](https://ntnuopen.ntnu.no/ntnu-xmlui/bitstream/11250/2592250/1/asterix_paper_v1.3.pdf)).
- Kilter places pelargonic acid droplets on 6 × 6 mm targets ([Kilter](https://www.kiltersystems.com/products)), but pelargonic acid is weaker on grasses ([Agronomy 2023](https://doaj.org/article/8ac365ffe3cf43eea06b5b7884620df3)).

Its limits are regulatory and agronomic rather than physical. It needs crop-specific authorisation, it is excluded from organic production, and one droplet of a systemic herbicide on a pepper leaf is catastrophic.

**Microshock electrical weeding** is the dark horse:

- Single 10 kV pulses of about 5 J killed small shepherd's-purse seedlings, while ryegrass needed about 200 J per plant for 90% kill ([Massey thesis](https://mro.massey.ac.nz/handle/10179/71403)).
- A field paper reported more than 90% control of every species tested ([AgResearch](https://agresearch.figshare.com/articles/journal_contribution/Pots_to_Plots_Microshock_weed_control_is_an_effective_and_energy_efficient_option_in_the_field/27747909)).

That paper's per-hectare figures imply about 0.3 J per broadleaf weed, an order of magnitude below the thesis's 5 J. Someone needs to read the full text to resolve this. A reviewer has already raised the safe distance between weed and crop as an open question ([MDPI review report](https://mdpi.com/2071-1050/16/11/4324/review_report)), and current returning through moist soil near pepper roots is an unmeasured risk.

**Perennials** are a separate case. Nutsedge regrows from tubers whatever tool hits the shoot, so a rover can only suppress it by repeatedly stripping its leaves, not kill it.

**Combining tools.** The most promising design pairs two tools:

- untargeted disturbance running constantly across the open strip, with no per-weed vision;
- a vision-guided stamp (organic) or a laser or micro-dose (conventional), kept for the minority of weeds near stems and for escapes.

An illustrative calculation shows the scale. A 1,000 m² tunnel with 60% of its area between rows, hit by a flush of 20 weeds/m², holds about 12,000 weeds. Treating all of them would take:

- about one full 100 Wh pack by laser at ~30 J per weed;
- up to about 480 kJ by stamp, roughly 1.3 packs at 40 J per weed;
- only 10-20 mL of liquid by micro-dose.

Covering the 600 m² of inter-row once with the untargeted tool takes about 11 hours at 0.1 m/s.

## Conclusion

The master variable is how often the rover revisits, not how strong it is. Frequent passes keep every weed at the cotyledon stage. That is what makes 2-25 N, 10-40 J per weed and a cheap camera good enough. But frequency multiplies the coverage burden, and the rover's speed is boxed in from both sides. Coverage pushes it toward 0.3-0.5 m/s. Motion blur, a fraction of a second of time in view, and centimetre-level aiming pull it back toward 0.1-0.2 m/s or stop-and-treat. Viability is therefore a three-way trade between revisit interval, speed and fleet size, and it holds only if units are cheap enough to run one or two per hectare. Pepper sharpens a second point. Most of its critical weeding period falls before the canopy closes, so "under canopy" is not the robot's main advantage there; precise, herbicide-free weeding next to the stem is. The easiest first market is not open-field paprika but organic plasticulture row middles and bare-soil tunnels:

- Mulch edges and tunnel hoops give strong navigation cues.
- Docks and fiducials are easy to place.
- GNSS is unnecessary.
- The alternatives, capped herbicide passes or hand labour, are expensive.

A tunnel is the farm's version of Tertill's fenced garden bed. Open-field paprika should come second, once a stamp or other near-stem tool has shown measured kill rates.

Four field experiments would change this verdict more than any further desk research:

1. **Row-end turns** using coded fiducials and IMU-controlled pivots: what fraction succeed?
2. **Mobility:** how often does a 15 cm wheeled body get stuck on wet, cloddy, freshly irrigated soil?
3. **Kill rate** of a micro-stamp and of grouser-and-comb disturbance when revisited every 1-3 days.
4. **Clearance** under real paprika canopies, measured fortnightly through the season.

A few prototypes over one season in a single tunnel and a single field could answer all four. Because no robot under 10 kg has ever published weed-control data from a dry farm field, even a modest, honest result would be the first of its kind.
