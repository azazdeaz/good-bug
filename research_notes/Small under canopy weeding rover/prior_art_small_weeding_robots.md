# Prior art: very small autonomous weeding robots and small under-canopy / between-row rovers (status as of November 2026)

Scope note for the report writer. The proposed robot is about 15 cm wide x 20 cm long, drives between crop rows under the canopy, uses RGB camera(s) + IMU + optional GPS, and runs on a Raspberry Pi or small Jetson. These notes combine (a) a salvaged earlier search session and (b) a continuation session in November 2026.

Source caveat: in both sessions WebFetch could not reach most hosts (blocked: cambridge.org, openagrar.de, therobotreport.com, tertill.com, onesoil.ai, agnavigator.com). Nearly every finding below therefore comes from search-engine result summaries of the cited pages, not from full-text reads. Numbers that appear in only one source, or that conflict between sources, are flagged. Unit conversions and arithmetic marked "(calc)" are mine, not the source's.

Self-reference flag: the Hackster project "Good Bug, Raspberry Pi weeding robot" (https://hackster.io/azazdeaz/good-bug-raspberry-pi-weeding-robot-ae58e5, GitHub azazdeaz/good-bug) is this repository's own earlier write-up. The local README links to that Hackster page and the git remote is github.com/azazdeaz/good-bug. Do not count it as independent prior art.

---

## Q1. Which robots are closest in size and concept, and what are their specs, performance, prices, status and lessons?

### Takeaway
Only three precedents are near the proposed size class (under about 10 kg) and have actually weeded outside a lab. The **Tertill** (consumer garden weeder, dry soil) was discontinued by about 2025 and gave 54–75% weed control in a peer-reviewed trial. The **Nissan "Aigamo" duck prototype** (about 1.5 kg) and the **NARO/Iseki "Aigamo Robo"** (6 kg, sold in Korea from 2026) both work in flooded rice paddies. The closest under-canopy concept is UIUC/EarthSense's **TerraSentia / Cover Crop Robot** (about 51 x 33 x 33 cm, under 13.6 kg). It navigates with RGB cameras, but it was deployed for phenotyping and cover-crop seeding, not weeding. Every commercial between-row weeder that ships at scale weighs about 90–400 kg (Greenfield, Aigen, Naïo Oz, Odd.Bot) or far more (FarmDroid, Carbon Robotics).

### Cited Findings

#### A. Sub-10 kg weeding robots (closest in size)

**Franklin Robotics / Tertill Corporation – Tertill (consumer garden weeder, USA)**
- History and status: Kickstarter launch in 2017. The official Tertill site now says the company decided to end the product after getting it "into thousands of gardens", that a limited set of accessories is still sold, and that no support is available any more. — [tertill.com (search summary; direct fetch blocked)](https://tertill.com/)
- Discontinuation date: a startup-profile aggregator says the product line was discontinued "by late 2025" and that the company was renamed Tertill Corporation. The aggregator's text is partly garbled; no dated company announcement was found. — [startupintros.com](https://startupintros.com/orgs/tertill-corporation)
- Retail status: Amazon UK lists it as "currently unavailable", and an eBay listing calls new units "discontinued and hard to find". An older gearbrain listing still says "Discontinued: No" (undated, likely stale). — [Amazon UK](https://www.amazon.co.uk/Tertill-Garden-Weeding-Robot-Automatically/dp/B08NXQPZPG); [eBay](https://www.ebay.com/itm/336157555741); [gearbrain](https://www.gearbrain.com/devices/7FB41792BB6D11EC8E8341B1B269F696.html)
- Price: $349 (Bob Vila review; review date not captured). — [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/)
- Form: a solar-powered, waterproof disc that looks like a small green Roomba and is meant to stay in the garden from spring to fall. — [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/); [House Digest](https://www.housedigest.com/1866284/tertill-garden-weed-robot-tool-worth-buying-reviews)
- Weeding tool: the wheels churn the soil, and a cutting string (trimmer) on the underside cuts short plants. — [Bob Vila / reviews summary](https://www.bobvila.com/articles/tertill-weeding-robot-review/)
- Plant-vs-weed discrimination: per the maker's FAQ, by plant height (short = weed). No camera. Reviewers warn that an overgrown weed may read as a crop plant, so the bed must be fully weeded before deployment. — [tertill.com FAQ (search summary)](https://tertill.com/pages/faqs); [Modern Castle review](https://moderncastle.com/home-goods/tertill-review/)
- Coverage: up to 200 sq ft (about 18.6 m², calc) per robot, e.g. a 10 x 20 ft garden. One tester measured about 2 h of total running per day. — [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/); [Modern Castle](https://moderncastle.com/home-goods/tertill-review/)
- Deployment constraints: the garden must be enclosed by a gap-free barrier about 4 in (10 cm) high. The robot can get stuck on uneven terrain. One reviewer's unit did not hold up to Florida humidity, rain and sun, and another noted a 3.2-star Amazon rating. — [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/); [House Digest](https://www.housedigest.com/1866284/tertill-garden-weed-robot-tool-worth-buying-reviews)
- Peer-reviewed efficacy: Sanchez & Gallandt, *Weed Technology* 35(1), DOI 10.1017/wet.2020.94 (online Aug 2020, cited as 2021).
  - Weed-control efficacy was 54–75% with the string trimmer active and 16–29% without it (wheels only). Wheels-only figures were 16% on condiment mustard and 22% on pearl millet.
  - The best control came from the combined effect of wheel soil disturbance plus the trimmer.
  - It reduced dicot (broadleaf) density but did not significantly reduce monocot (grass) density.
  - The authors called efficacy "high" for home gardens but said Tertill "would require modification to be viable for farm-scale use".
  - Sources: [Cambridge abstract](https://www.cambridge.org/core/journals/weed-technology/article/abs/functionality-and-efficacy-of-franklin-robotics-tertill-robotic-weeder/07DF9F4329CB64D749E822168A097D92); [ResearchGate](https://www.researchgate.net/publication/343834903_Functionality_and_efficacy_of_Franklin_Robotics'_Tertill_robotic_weeder); [2025 systematic review (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC12756371/). The 16–29% and 16%/22% figures may be different comparisons within the paper; the full text was not retrieved.
- A second study exists, "Effects of Tertill Weeding Robot on Weed Abundance and Diversity" (*Agronomy* 2022, 12(8):1754). Its results were not retrieved. — [doi.org/10.3390/agronomy12081754](https://doi.org/10.3390/agronomy12081754)

**Nissan "Aigamo" robot duck (rice paddy prototype, Japan, 2019)**
- About 1.5 kg and about the size of a large robot vacuum. One source gives 60 cm length (single report).
- Two rotating rubber brushes on the underside, in place of duck feet, stir the water and mud.
- Trials began in Yamagata Prefecture. The engineer was Tetsuma Nakamura. Nissan made no production commitment.
- Sources: [Future Farming](https://futurefarming.com/tech-in-focus/nissan-robot-duck-keeps-paddy-fields-clear-of-weeds); [designboom 2019](https://www.designboom.com/technology/nissan-robot-duck-rice-fields-farming-weeds-06-24-2019/); [FarmingUK](https://www.farminguk.com/news/nissan-introduces-robot-duck-for-japanese-farmers_53286.html)

**"Aigamo Robo" (NARO / NEWGREEN / Tokyo Univ. of Agriculture & Technology / Iseki; Korean release by TYM, Sept 2026)**
- How it weeds: it muddies paddy water to block sunlight and suppress weed germination. This is non-selective, so no crop/weed classification is needed.
- Specs: 6 kg; mainly solar; up to 10 h/day; up to 15,000 m² (1.5 ha) covered. Navigates autonomously by detecting paddy ridges; one summary also says GPS-guided.
- Trials: a two-year trial on 36 organic rice farms reported a weed reduction. The excerpt was truncated before the figure.
- Status: TYM announced its domestic (Korean) release in September 2026. Price was not found.
- Sources: [Herald Corp (Korea), Sept 2026](https://biz.heraldcorp.com/article/10863020); [JStories: small robot to save Japan's terraced paddies](https://jstories.media/small-robot-save-japans-terraced-paddies/)

#### B. Under-canopy / between-row rovers under about 50 kg (closest in concept)

**EarthSense TerraSentia and Cover Crop Robot (CCR); UIUC CropFollow / CropFollow++**
- Size: about 20 x 13 x 13 in (about 51 x 33 x 33 cm, calc), under 30 lb (under 13.6 kg). Runtime is about 8 h per one TERRA-MEPP page, but about 4 h per a 2019 model description (conflict). — [TERRA-MEPP](https://terra-mepp.illinois.edu/node/5); [TERRA-MEPP: revamped robot at ARPA-E summit](https://terra-mepp.illinois.edu/node/78)
- Other coverage gives "about 30 lb, just over a foot wide". One article says most of the mass is its 5,500 mAh LiPo battery. — [AgriExpo trends](https://trends.agriexpo.online/project-66965.html); [UST magazine](https://www.ust-media.com/ust-magazine/UST024/76/)
- Price: $5,000 (2017 launch reporting). An aggregator's $15,000 figure looks auto-generated and is unreliable. — [Daily Illini](https://dailyillini.com/?p=180444); [TERRA-MEPP repost of Digital Trends](https://terra-mepp.illinois.edu/node/48); [Robotomated (unreliable)](https://robotomated.com/explore/agricultural/earthsense-terrasentia)
- Fleet and deployment: UIUC/NCSA coverage says EarthSense has over 100 TerraSentia units. A separate article says the cover-crop robot had "planted several hundred acres" (dates about 2022–2023; which page carries which claim is uncertain). A 2024/25 preprint credits the robots' compact, light build for their broad deployment. — [NCSA "A Fruitful Partnership"](https://www.ncsa.illinois.edu/a-fruitful-partnership/); [High Plains Journal "Bots on the ground"](https://hpj.com/?p=32805); [Research Square preprint](https://www.researchsquare.com/article/rs-5187688/latest)
- CropFollow (Sivakumar et al., RSS 2021):
  - Navigation uses a single front-facing RGB camera on TerraSentia, trained in corn.
  - Over 25 km of field testing, it averaged 485 m per intervention versus 286 m for a state-of-the-art LiDAR system.
  - Corn-trained models appeared to transfer to soybean rows, but soybean trials were left for future work.
  - Source: [RSS 2021 paper](https://www.roboticsproceedings.org/rss17/p019.pdf)
- CropFollow++ (UIUC + EarthSense, arXiv 2404.17718, 2024):
  - Low-cost RGB cameras; a neural network predicts row keypoints that feed a model-predictive controller.
  - Deployed on three EarthSense Cover Crop Robots over 25,315 m in total, planting cover crops under corn canopy.
  - Most runs were shorter than 250 m; three autonomous runs exceeded 2,000 m.
  - Interventions fell into seven types, including vision keypoint errors, physical robot failures, corn gaps, weeds/occlusion and planting errors.
  - Source: [arXiv 2404.17718](https://arxiv.org/pdf/2404.17718v1.pdf)
- Weeding status: TerraSentia is described as adaptable to a mechanical weeding arm, and UIUC-linked sources list selective spraying/mechanical weeding as future work. No deployed autonomous under-canopy weeding product was found. A 2024 Illinois (ACES) study states that under-canopy robots "are not yet commercially available for corn and soybeans". — [ACES Illinois news](https://aces.illinois.edu/news/new-illinois-study-explores-adoption-robotic-weeding-fight-superweeds); [UIUC Digital Ag / EarthSense partnership](https://digitalag.illinois.edu/2022/07/14/cda-partnership-with-earthsense/)
- Related: ROW-SLAM (UMN) addresses localising corn stalks mid-season under canopy, where there is "very little space between the camera" and the plants. A US patent covers "coordinated autonomous teams of under-canopy robotic systems". — [arXiv 2109.07134](https://arxiv.org/pdf/2109.07134); [USPTO 12001221](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12001221)

**UCLA SAMBot (Du, Zhang, Tsang, Jawed; ICRA 2022, arXiv 2112.13986): under-canopy weed classification**
- Small robot running a Jetson Nano with a TensorRT-optimised CNN.
- Built the AIWeeds dataset: about 10,000 annotated images of flax plus 14 common weeds, from 20 locations in North Dakota, California and central China.
- MobileNetV2 was chosen for the shortest inference time and lowest memory use.
- Results: about 90% classification accuracy on unseen flax-field scenes (0.2–0.3 m rows), with 47.78 ms inference.
- The weeding actuator and the weed-kill rate were not retrieved.
- Sources: [arXiv 2112.13986](https://arxiv.org/abs/2112.13986); [NSF PAR copy](https://par.nsf.gov/servlets/purl/10354717)

**Fendt Xaver (ex-EU MARS "Mobile Agricultural Robot Swarms"): seeding swarm, not weeding**
- Mass: 40 kg per unit (newer report) versus 50 kg (launch reports), with ground pressure about 200 g/cm².
- Fendt claims: 6–12 units plant about 1 ha/h of maize. A later report gives six units at about 3 ha/h (about 2 ha/h including loading). Fendt also claims about 70% less energy than conventional equipment. If one unit fails, the remaining units' paths are re-optimised automatically. These are manufacturer claims with no independent trial data found.
- The MARS EU/ECHORD project demonstrated corn seeding with two robots.
- Sources: [FWI](https://www.fwi.co.uk/machinery/technology/fendt-proposes-swarm-robots-corn-planting); [Irish Examiner "Fendt ready to sell robot swarms"](https://www.irishexaminer.com/farming/arid-30811455.html); [ECHORD MARS](https://echord.eu/mars); [Lectura](https://lectura.press/en/article/xaver-robot-system-for-planting-and-accurate-documentation/40474); [OEM Off-Highway](https://www.oemoffhighway.com/market-analysis/trends/press-release/20983953/fendt-fendt-develops-robot-system-for-planting-and-accurate-documentation)

**ACFR / Agerris Digital Farmhand (University of Sydney; smallholder-oriented)**
- Low-cost platform that can be hand-towed, remote-controlled or autonomous.
- Carries a smartphone, sensors and computing; two 24 V motors with chain drive; two cameras with ML for crop/weed discrimination. It has used Emlid Reach RS+ RTK GNSS.
- Configurable for 1.5–3 m row spacing.
- Weight and dimensions were not found.
- Sources: [Good Fruit & Vegetables](https://www.goodfruitandvegetables.com.au/story/7433579/robotics-revolution-rolling-out-across-australia/); [Robotic Gizmos](https://www.roboticgizmos.com/digital-farmhand-row-crop-robot/); [Emlid blog](https://blog.emlid.com/agricultural-robot-powered-with-reach/)

**ROMI rover / LettuceThink (Sony CSL Paris + EU ROMI project; market-garden weeding)**
- LettuceThink (EFITA 2017) is a lightweight, modular, open-source platform built on a Raspberry Pi and Arduino-compatible boards linked over Ethernet. The paper reports only early exploratory results.
- The ROMI rover has one weeding arm (CNC-style) for small vegetable farms, with a target price of €5,000 or less.
- Field tests ran on an organic farm near Barcelona and at Pépinières Chatelain near Paris CDG.
- No quantified weed-kill results were found.
- Sources: [EFITA 2017 paper](https://csl.sony.fr/wp-content/uploads/2017/01/ColliauxHanappe_109_EFITA2017.pdf); [EU Horizon Magazine](https://projects.research-and-innovation.ec.europa.eu/en/horizon-magazine/robots-weeding-organic-farms-and-patrolling-greenhouse-pests); [ROMI downloads](https://romi-project.vercel.app/downloads)

#### C. Open-source / hobby builds
- **Nindamani** (AutoRoboCulture, India, 2020): an AI mechanical weed-removal prototype on ROS 2 and a Jetson Nano. It won first place in the autonomous machines category of NVIDIA's "AI at the Edge" challenge. A community fork, Open-Weeding-Delta, targets early-stage crops with mechanical or laser actuators. No field efficacy data. — [NVIDIA Jetson projects](https://developer.nvidia.com/embedded/community/jetson-projects/nindamani); [GitHub](https://github.com/AutoRoboCulture/Nindamani-the-weed-removal-robot); [NVIDIA blog 2020](https://blogs.nvidia.com/blog/robotics-duo-digs-into-the-weeds-with-winning-ai-project-at-hackster-io/); [Open-Weeding-Delta](https://github.com/Agroecology-Lab/Open-Weeding-Delta)
- **WEEDINATOR** (Goat Industries, UK; 2026 iteration): tractor-scale, not small.
  - Runs OpenCV + YOLO26n on a Jetson Nano; weeds with three rotating claws on steel bars.
  - It "still requires some operator involvement".
  - The 2026 log notes that the hydraulics lack dynamic pressure regulation, so running too fast causes side-to-side oscillation.
  - Sources: [Hackaday.io](https://hackaday.io/project/204846-weedinator-2026-agricultural-robot); [Hackster news](https://www.hackster.io/news/the-weedinator-project-brings-autonomous-agriculture-to-the-masses-1038dda40d19)
- **Acorn** (Twisted Fields, open source, USA):
  - 400 lb (181 kg) per one database entry; solar-powered with optional batteries and generator.
  - Built from welded aluminium and bicycle parts. An 800 W solar array is on board; the robot draws about 60 W on level ground per 2019 posts, later about 20 W.
  - Still a prototype in 2021 coverage, and the community has discussed a 50%-scale version.
  - 2025–26 status not found.
  - Sources: [agricultural-robotics.com](https://www.agricultural-robotics.com/robot/acorn); [Twisted Fields energy budget](https://community.twistedfields.com/t/energy-and-power-budgets/63); [Hackaday 2021](https://hackaday.com/2021/03/05/a-new-open-source-farming-robot-takes-shape/); [50% scale thread](https://community.twistedfields.com/t/50-scale-version-of-acorn/325)
- **FarmBot Genesis**: a CNC gantry for raised beds, not a rover. Its weeder tool "smash[es] weeds into the ground". Kits cost about $2,995 (v1.7) and $4,395 (XL v1.7) in about 2019; one retailer now lists the XL at $6,230. — [FarmBot docs](https://genesis.farm.bot/v1.5/FarmBot-Genesis-V1.5/intro/high-level-overview); [weeder BOM](https://genesis.farm.bot/v1.6/bom/plastic-parts/weeder); [Future Farming](https://futurefarming.com/farmbot-genesis-seeding-feeding-and-weeding); [West3D](https://west3d.com/en-au/products/farmbot-genesis-xl-v1-7-complete-smart-cnc-farming-solution)
- **OpenWeedLocator (OWL)**: a Raspberry Pi 4 + camera that detects weeds and drives a relay board for spot spraying. MIT-licensed with open hardware, and backed by a *Scientific Reports* paper on fallow weed detection. — [Raspberry Pi magazine](https://magazine.raspberrypi.com/articles/open-weed-locator); [Sci. Rep. s41598-021-03858-9](https://springerlink.bibliotecabuap.elogim.com/content/pdf/10.1038/s41598-021-03858-9.pdf)
- **Solar-concentrator weeder**: a Raspberry Pi 3B+ drives two DC motors and scans the ground with a camera, then burns weeds with a Fresnel lens. Garden scale, no efficacy data. — [Hackaday 2023](https://hackaday.com/2023/06/30/gardening-robot-uses-sunlight-to-incinerate-weeds); [Tom's Hardware](https://tomshardware.com/news/raspberry-pi-weed-burning-robot)
- **CERES MK2** (Hack Club Stardance): a hobbyist devlog with a Raspberry Pi camera mount for image recognition. — [Stardance](https://stardance.hackclub.com/projects/5595)
- **Good Bug**: Raspberry Pi + camera + UPS (this project; see the self-reference flag above). — [Hackster](https://hackster.io/azazdeaz/good-bug-raspberry-pi-weeding-robot-ae58e5)

#### D. Mid-size commercial between-row / in-row weeders (about 90–400 kg)

**Greenfield Robotics (Kansas, USA): "WeedBot" / BOTS, between-row cutting in 30-in (76 cm) row crops**
- Speed and work rate:
  - Travels at about 3.5 mph (about 1.56 m/s, calc).
  - One robot does 0.5–1 acre/h (about 2,000–4,000 m²/h, calc; company figures). A fleet of ten does about 10 acres/h (company claim).
  - A University of Vermont pilot measured about 7 acres/day (about 2.8 ha) for a single robot. Other figures: about 0.85 acre/h, and 30–100 acres/day for a fleet.
- Weeding method: spinning blades cut between-row weeds to about 0.5 in. A company profile claims under 1% crop damage.
- Crops: cotton, corn, soybeans and milo.
- Mass: about 200 lb (about 91 kg) in older reports, about 350 lb (about 159 kg) in a later interview (conflict).
- Hours and power: solar reported in some articles. Sources conflict on whether it works day and night or daylight only.
- Fleet and season targets (2024): a fleet of 25 small yellow robots. Targets for the season ranged from about 5,000 acres with 20 farmers to more than 20,000 acres (conflicting). No measured season total was found.
- Price and business model: $36,500 per robot (Nathan Dyrud). The company moved from robots-as-a-service to outright purchase, with leasing to come, and opened a public share offering (StartEngine, 2025) "after 6 years in the field".
- Sources: [Farm Equipment](https://farm-equipment.com/articles/21281); [No-Till Farmer podcast](https://www.no-tillfarmer.com/articles/12183-podcast-weeder-robots-and-taking-the-chemicals-out-of-no-till-with-greenfield-robotics); [No-Till Farmer: lower costs](https://no-tillfarmer.com/articles/12385-robot-weeders-provide-lower-costs-and-higher-crop-prices); [Startland 2022](https://startlandnews.com/2022/03/greenfield-robotics-weedbot/); [Precision Farming Dealer](https://www.precisionfarmingdealer.com/articles/6426-greenfield-robots-autonomously-kill-weeds-plant-cover-crops-and-more); [UVM Extension](https://www.uvm.edu/extension/nwcrops/news/recap-first-public-demonstration-autonomous-weeding-robots-vermont); [The New Lede 2024](https://www.thenewlede.org/2024/07/in-bid-to-slash-chemical-use-robots-take-on-farm-fields/); [KCUR 2024](https://www.kcur.org/news/2024-04-08/robots-weed-fields-cut-down-herbicides); [Compass Vermont](https://www.compassvermont.com/p/vermont-farmers-see-robotic-weeders); [Future Farming: share offering](https://www.futurefarming.com/tech-in-focus/field-robots/greenfield-robotics-opens-public-share-offering-after-6-years-in-the-field/); [KingsCrowd 2025](https://kingscrowd.com/greenfield-robotics-on-startengine-2025/); [K-State 2025 profile](https://extension.k-state.edu/news-and-publications/news/stories/2025/01/kansas-profile-greenfield-robotics.html)

**Aigen Element (USA): solar, AI, in-row weeding**
- Price: $50,000 per unit plus a $20,000 per-fleet fee for coordination and software.
- Coverage (inconsistent across sources): "up to 20 acres (8 ha)" per robot; "30 acres per week"; "20–40 acres per season". Up to 14 h/day of operation.
- Gen2 (April 2025): wider and taller, for cotton, soy and sugar beet. In 2025 about 50 units were operational, 30 of them running on three farms.
- Oct 2026: a four-armed version that pulls weeds mechanically was launched.
- Raised $12M to scale the fleet to more than 20,000 acres.
- Weight was not found (a comparison site leaves it blank).
- Sources: [Future Farming: Gen2](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops); [Fast Company 2024](https://www.fastcompany.com/91033135/aigen-most-innovative-companies-2024); [AgFunderNews](https://agfundernews.com/aigen-raises-12m-to-scale-its-fleet-of-solar-powered-autonomous-robots-to-more-than-20000-acres); [Global Agriculture (four-arm)](https://www.global-agriculture.com/ag-tech-research-news/aigen-launches-four-arm-robot-that-pulls-weeds-without-herbicide/); [Precision Farming Dealer](https://www.precisionfarmingdealer.com/articles/6511-aigen-launches-next-gen-element-weeding-robots-with-bowles-farming); [Robotomated comparison](https://robotomated.com/compare/aigen-element-vs-small-robot-tom)

**Naïo Oz (France): the smallest mainstream commercial weeding robot**
- Specs: 150 kg (plus a 30 kg optional front weight kit); 1.30 m long x 0.47 m wide x 0.83 m high; about €40,000 (2023 figure). — [Future Farming spec page](https://futurefarming.com/naio-oz-mechanical-weeding-robot-and-farmhand); [Naïo Oz page](https://www.naio-technologies.com/en/oz-robot/)
- Company: Naïo Technologies entered *redressement judiciaire* (court-supervised receivership) in June 2025. A new company, "Naïo", led by Antoine Monville, took it over for a reported €170,000 and kept about 20 of 49 jobs (CMS press release, 12 Nov 2025). The successor dropped the Orio and Jo robots to focus on Ted and Oz, and added a small tool carrier, "Sanka". — [Réussir](https://www.reussir.fr/machinisme/naio-technologies-place-en-redressement-judiciaire); [Réussir: buyer found](https://www.reussir.fr/fruits-legumes/un-repreneur-pour-les-robots-naio-technologies); [Agra](https://www.agra.fr/en-redressement-judiciaire-naio-technologies-cherche-un-repreneur); [CMS](https://cms.law/en/fra/news-information/cms-francis-lefebvre-conseille-les-repreneurs-de-naio-technologies); [Pleinchamp](https://www.pleinchamp.com/actualite/sanka-un-petit-robot-porte-outils-au-catalogue-de-naio); [top agrar](https://www.topagrar.com/technik/news/mit-naio-technologies-ist-der-nachste-roboterhersteller-pleite-20016041.html)
- A cfnews item dated 29/06/2026 describes a liquidation followed by a takeover by former shareholders. This conflicts with the 2025 takeover reports and is treated as unreliable. — [cfnews](https://www.cfnews.net/L-actualite/Retournement/Operations/Capital-innovation/Naio-Technologies-repris-par-certains-de-ses-actionnaires-547433)

**Odd.Bot Maverick (Netherlands): in-row weed pulling with delta arms**
- Mass and size: about 400 kg; carrier 2.05 x 1.70 x 1.55 m.
- Speed and work rate: up to 600 m/h of travel. Up to three 24 V batteries of 8 h each. Weeding 1 ha takes up to 16 h (about 625 m²/h, calc); the brochure claims up to 2 ha/day.
- Price: €90–120k earlier, "from €95,000" in Nov 2024; standalone "Weader" arm €32,000.
- Claim: replaces 80% of manual weeding, equivalent to up to 10 manual weeders.
- Status: €2M raised in Jan 2025, when the first 16 robots for 2025 were reported sold out. A 2025 update says it is on "dozens of fields" in NL/BE/DE/FR.
- Sources: [Future Farming spec](https://futurefarming.com/odd-bot-maverick); [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/odd-bot-takes-to-the-fields-with-maverick-weeding-robot); [Precision Farming Dealer](https://www.precisionfarmingdealer.com/articles/6324-oddbot-secures-2-million-to-advance-autonomous-chemical-free-weeding-solutions); [Odd.Bot 2025 update](https://3nx74d24v2ongn07.umso.co/news/the-maverick-in-2025)

**Earth Rover CLAWS (UK): concentrated-light weeding and scouting in vegetables**
- Weeding method: pulsed concentrated light, which the company says becomes eye-safe beyond a set distance.
- Power: up to 16 h per charge with a solar top-up. The company says one unit ran the full 2026 summer season without a battery swap.
- Sales: 10 units sold and 15 under construction (CEO James Miller).
- Market and economics: targets vegetable farms of about 100 ha with heavy weed pressure. The company projects 1–3 year payback.
- Funding and demos: grants from Innovate UK and the Forestry Commission (ESA also lists the project). Public demo on 23 July 2025.
- Mass and price were not found.
- Sources: [AgFunderNews](https://agfundernews.com/🎥-earth-rover-bets-on-pulsed-light-to-tackle-weeds-without-chemicals); [Global Agriculture](https://www.global-agriculture.com/ag-tech-research-news/pulsed-light-robot-clears-weeds-from-uk-vegetable-fields-without-herbicides/); [HTA FutureGrow demo](https://hta.org.uk/futuregrow/earth-rover-demonstration); [ESA Business](https://business.esa.int/projects/earth-rover-concentrated-light-autonomous-weeding-and-scouting)

**Small Robot Company "Tom" (UK): scouting/weed-mapping robot (detail in Q4)**
- Scanned about 20 ha/day, collecting about 6 TB per 8 h shift (single secondary source).
- Light enough to fit in a small van. Weight was not found.
- Weed-mapping service: launched on four UK farms, mapping broad-leaved weeds in wheat. Trial pricing was £15/ha; the planned service model was about £140/ha.
- Sources: [AITopics (secondary)](https://aitopics.org/tag/kill%20individual%20weed); [FWI](https://www.fwi.co.uk/arable/crop-management/weed-management/video-worlds-first-robotic-weed-mapping-service-launched); [AA Farmer](https://www.aafarmer.co.uk/news/weed-mapping-service-from-tom.html); [Future Farming: Tom](https://futurefarming.com/small-robot-company-tom-monitoring-robot)

**Rowbot Systems (Minneapolis, founded 2012): between-row fertiliser rover for tall corn (not weeding)**
- GPS-linked; carried 40 gal of UAN and side-dressed four rows at a time.
- A later patent filing criticises it as "heavy, driven by diesel engines", able to "permanently damage crops", and "barely" fitting in corn rows. This is a competitor/inventor critique, not an independent assessment.
- Current status was not found.
- Sources: [Farm Progress](https://www.farmprogress.com/precision-ag/robot-attempts-nitrogen-sidedress-cover-crop-seeding); [USPTO 11789453](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11789453)

**Tensorfield (USA)**
- Showed a spot-spraying robot at FIRA USA 2025 that microdoses each weed with about 0.1 g of vegetable oil after camera detection of weed vs crop.
- Also makes no-contact crop-thinning and in-row weeding machinery.
- Size, price and performance were not found.
- Sources: [Farm Equipment](https://www.farm-equipment.com/articles/24879-technology-in-agriculture-continues-to-grow-at-rapid-pace); [Tracxn](https://tracxn.com/d/companies/tensorfield/__rY_VQp9mEYI91Iwk34ZRmBe9f-gG8tYaqDXJyJzpyJE)

**Ecorobotix Gen 1 (Switzerland)**
- An approximately 130 kg solar robot, predecessor of the tractor-scale AVO/ARA sprayers. ARA is certified for 95% drift reduction in Germany and the Netherlands.
- Ecorobotix closed a $105M Series D in Oct 2025.
- Sources: [AgTecher](https://agtecher.com/product/ecorobotixs-autonomous-weed-killer/); [Precision Farming Dealer 2025 funding](https://www.precisionfarmingdealer.com/articles/7113-autonomous-weeding-robots-lead-2025-precision-ag-funding)

#### E. Large machines (brief contrast only)
- **FarmDroid FD20** (Denmark): solar, RTK-GPS seeding and weeding.
  - Mass 900 kg (2023 factsheet) versus 1,250 kg standard and 1,800 kg max (current factsheet).
  - Working width up to 3.5 m; up to 6 ha/day; 6 kWh battery.
  - Price about €97,000 plus VAT, plus an RTK base station (Ireland, 2023).
  - Sources: [FarmDroid FD20](https://farmdroid.com/products/farmdroid-fd20/); [2023 factsheet](https://farmdroid.com/wp-content/uploads/Factsheet-FD20-Metricimperial-2023.pdf); [Farmers Journal 2023](https://www.farmersjournal.ie/machinery/farm-machinery/ireland-first-autonomous-seed-n-weed-robot-gets-to-work-in-the-southeast-769012)
- **Carbon Robotics LaserWeeder**: G2 (Feb 2025) is "faster, lighter, modular". More than 150 LaserWeeders have shipped to farmers in 14 countries. One market piece cites about $268/acre deployment cost (low-quality source). — [Farm Equipment](https://www.farm-equipment.com/articles/24879-technology-in-agriculture-continues-to-grow-at-rapid-pace); [RobotToday (low quality)](https://robottoday.com/article/weeding-and-crop-management-robots-the-34-b-market-disruption-hiding-in-plain-sight)
- **SwarmFarm** (Australia): about 2 t robots fully loaded. A nextpaper.cc story about "12 kg SwarmFarm micro-robots" and a 1,000-unit deployment is future-dated, unverifiable, and should be treated as fabricated or unreliable. — [WeedSmart](https://www.weedsmart.org.au/content/swarmfarm-targeting-small-weeds-all-year/); [nextpaper.cc (unreliable)](https://nextpaper.cc/en/posts/flash-swarmfarm-2028)

### Inferences
- **Tertill.** It is the only dry-land precedent below about 2 kg with peer-reviewed efficacy. Its success depended on four conditions:
  - a fenced, flat, pre-cleaned garden bed;
  - continuous daily patrolling, so weeds were always tiny;
  - a non-vision height rule;
  - a string trimmer plus wheel scuffing.

  It controlled broadleaves but not grasses, and was judged not farm-viable. A 15 x 20 cm rover inherits the same physics: it can only kill very small weeds and must revisit often.
- **Aigamo Robo.** It is the strongest evidence that a robot under 10 kg can be a real product with multi-farm trials (36 farms). It works because its weeding action is non-selective (turbid water), so it needs no crop/weed perception. That does not transfer to dry-soil row crops.
- **TerraSentia/CCR.** These prove a robot about 33 cm wide and under 14 kg can drive tens of km under corn canopy using RGB-only row following. Even so, interventions were common (most runs under 250 m), and no weeding tool was ever commercialised on it. A 15 cm robot is about half that width, which gives more lateral clearance in 76 cm rows but less ground clearance and less obstacle-climbing ability.
- **Commercial scale.** Commercial between-row weeding has settled at about 90–400 kg (Greenfield, Oz, Odd.Bot) because tool forces, speed, battery and solar area all scale with mass. No commercial weeding robot below about 90 kg serves row-crop farms as of Nov 2026, apart from discontinued consumer Tertill and paddy robots.

### Gaps
- Tertill's exact mass, diameter, battery capacity and sensor type (some secondary descriptions mention capacitive sensing; the FAQ summary says height) could not be confirmed because tertill.com and Cambridge were blocked. The exact discontinuation date is also unconfirmed.
- No retrieved data for: Wyss Institute, ETH Zurich or Wageningen small/swarm weeding robots; Bonn (BoniRob is about 1 t-class, not relevant to size); ACFR RIPPA; QUT AgBot II. Searches for QUT AgBot II returned nothing.
- Aigen Element weight, sensors and compute; Earth Rover CLAWS mass and price; Greenfield 2025/2026 measured acreage. None found.
- EarthSense CCR and TerraSentia compute hardware (Jetson or otherwise) was not captured.
- Fendt Xaver 2025–2026 commercial status and independent field data were not found.
- UIUC/EarthSense under-canopy mechanical weeding: no published field efficacy was found.

---

## Q2. Is there any robot near 15 x 20 cm that has been demonstrated weeding in real soil? Mass and tool?

### Takeaway
No source found documents a robot of about 15 x 20 cm weeding in real field soil with measured results. The closest demonstrated precedent is the Tertill: a Roomba-like disc garden weeder (dimensions/mass not confirmed in retrieved sources) using a string trimmer and wheel scuffing, with 54–75% control in a peer-reviewed garden trial. The only other small robots with field weeding evidence are the paddy robots (about 1.5 kg Nissan prototype, 6 kg Aigamo Robo), which work in water rather than soil. Hobby Raspberry Pi/Jetson weeders exist but publish no efficacy data.

### Cited Findings
- **Tertill.** Disc-shaped and "Roomba-like". String trimmer plus wheels; height-based weed rule.
  - 54–75% efficacy with the trimmer and 16–29% with wheels only.
  - Controlled dicots but not monocots.
  - "Would require modification to be viable for farm-scale use."
  - Sources: [Cambridge abstract](https://www.cambridge.org/core/journals/weed-technology/article/abs/functionality-and-efficacy-of-franklin-robotics-tertill-robotic-weeder/07DF9F4329CB64D749E822168A097D92); [ResearchGate](https://www.researchgate.net/publication/343834903_Functionality_and_efficacy_of_Franklin_Robotics'_Tertill_robotic_weeder); [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/)
- **Nissan Aigamo duck (2019).** About 1.5 kg, two rotating rubber brushes, paddy water and mud, no production commitment. — [Future Farming](https://futurefarming.com/tech-in-focus/nissan-robot-duck-keeps-paddy-fields-clear-of-weeds); [designboom](https://www.designboom.com/technology/nissan-robot-duck-rice-fields-farming-weeds-06-24-2019/)
- **Aigamo Robo.** 6 kg; muddies water to block light; up to 1.5 ha per day-run; two-year, 36-farm trial reported a weed reduction (figure truncated). — [Herald Corp 2026](https://biz.heraldcorp.com/article/10863020)
- **Small rovers with RGB weed perception but no published weed-kill numbers:**
  - SAMBot, Jetson Nano, about 90% classification accuracy. — [arXiv 2112.13986](https://arxiv.org/abs/2112.13986)
  - Nindamani, Jetson Nano. — [GitHub](https://github.com/AutoRoboCulture/Nindamani-the-weed-removal-robot)
  - Raspberry Pi solar-lens weeder. — [Hackaday](https://hackaday.com/2023/06/30/gardening-robot-uses-sunlight-to-incinerate-weeds)
  - LettuceThink, Raspberry Pi. — [EFITA 2017](https://csl.sony.fr/wp-content/uploads/2017/01/ColliauxHanappe_109_EFITA2017.pdf)
  - Good Bug, Raspberry Pi; this project. — [Hackster](https://hackster.io/azazdeaz/good-bug-raspberry-pi-weeding-robot-ae58e5)
- **A 2023 study of a lightweight weeding platform.** At 90 kg, the robot could generate up to about 2,082 N of soil-interaction force. The paper notes that "miniaturized robots under 100 kg" have been proposed for swarm operation. — [Agriculture 13(8):1510](https://doi.org/10.3390/agriculture13081510)
- **An older intra-row corn weeder prototype (about 2011, China).** 89.7% weeding rate and 1.9% seedling damage, but slow: about 23.7 h per acre at 0.07 m/s (about 171 m²/h, calc). Size was not captured. — [ResearchGate](https://www.researchgate.net/publication/251682319_Design_and_testing_of_an_intra-row_mechanical_weeding_machine_for_corn)

### Inferences
- **Tool choice.** A 15 x 20 cm robot (probably 1–3 kg) cannot generate the hundreds to thousands of newtons that hoes and tines need (90 kg gives about 2 kN). It is therefore restricted to low-force tools that only work on cotyledon-stage weeds: string trimmers, small rotating brushes or wheel scuffing, micro-dosing, laser/light, or heat. The Tertill result (good on small broadleaves, poor on grasses) is the most direct evidence of where this tool class tops out.
- **Real novelty.** Under-canopy operation in field row crops with selective vision is genuinely unprecedented at this size; the Tertill operated in open garden beds without a canopy.

### Gaps
- No peer-reviewed field efficacy exists for any camera-guided weeding robot under about 10 kg in soil.
- Tertill's exact dimensions and mass were not confirmed in this session (sources blocked).

---

## Q3. What do reviews of small/lightweight/swarm weeding robots conclude about minimum practical size, compaction, work rate and economics?

### Takeaway
Economic modelling, mainly from Harper Adams (Lowenberg-DeBoer et al.) and Gaus et al. 2017, finds small autonomous machines can be cost-competitive, especially on small or irregular fields, because labour per machine disappears. Reduced compaction is still an untested hypothesis. No review found states an explicit minimum practical robot size. Work rate per small robot is low, so fleet size and continuous operation set throughput; maintenance of many units is a recognised cost risk.

### Cited Findings
- **Gaus et al. (2017, GEWISOLA)** used partial budgeting to cost mechanical weeding of wheat by a swarm of small autonomous robots. Assuming each robot covers 3 ha five times per season, they estimated an operating cost of **€30.20/ha/h** (units as reported by the citing review; the original was not retrieved). — [Lowenberg-DeBoer et al. 2020, Precision Agriculture](https://link.springer.com/article/10.1007/s11119-019-09667-5); [OpenAgrar abstract (fetch blocked)](https://www.openagrar.de/receive/openagrar_mods_00042517)
- **Lowenberg-DeBoer et al. 2020 review** ("Economics of robots and automation in field crop production"): small robots could "almost eliminate" the competitive advantage that large rectangular fields give commodity agriculture. — [Springer](https://link.springer.com/article/10.1007/s11119-019-09667-5)
- **"Economics of autonomous equipment for arable farms" (2021, UK model)**: relatively small autonomous equipment would have economic advantages across a wide range of farm sizes, but especially for small farms. Related Harper Adams work finds swarms can reach minimum production cost on small, irregularly shaped fields. — [ResearchGate](https://www.researchgate.net/publication/351914854_Economics_of_autonomous_equipment_for_arable_farms); [Economics of field size and shape for autonomous crop machines (2023)](https://link.springer.com/article/10.1007/s11119-023-10016-w); [Harper Adams swarm economics project](https://www.harper-adams.ac.uk/research/project/247/economics-of-swarm-robotics-in-uk-agriculture)
- **Lowenberg-DeBoer 2022 (AEPP), "Lessons to be learned in adoption of autonomous equipment for field crops"**:
  - Reduced soil compaction is a hypothesised benefit that "has not yet been tested in the field".
  - Swarms could work when soils are too wet for heavy machines.
  - One robot down for repair doesn't stop the others.
  - Source: [Wiley](https://onlinelibrary.wiley.com/doi/full/10.1002/aepp.13177)
- **Industry commentary**: swarms remove the per-vehicle driver cost, but maintaining many robots could take as much time as running one large machine. — [Precision Farming Dealer "Is your farm ready for the swarm"](https://www.precisionfarmingdealer.com/articles/4405-is-your-farm-ready-for-the-swarm)
- **GRDC GroundCover**: farm example of a lighter footprint and tighter turning circle reducing compaction (anecdotal). — [GRDC](https://groundcover.grdc.com.au/innovation/precision-agriculture-and-machinery/bots-on-the-ground-farmings-new-army)
- **2023 lightweight-platform paper**: miniaturised robots under 100 kg have been proposed for swarms, but traction and soil-force needs push mass up (90 kg gives about 2 kN). — [Agriculture 2023](https://doi.org/10.3390/agriculture13081510)
- **Other relevant reviews located but not summarised (content not retrieved)**:
  - "How much can farmers pay for weeding robots? A Monte Carlo simulation study". — [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC10075499/)
  - "Operational, Economic, and Environmental Assessment of an Agricultural Robot in Seeding and Weeding Operations" (AgriEngineering 2023). — [doi](https://doi.org/10.3390/agriengineering5010020)
  - "Small autonomous machines for sustainable soil management in vegetable crops and orchards". — [ResearchGate](https://www.researchgate.net/publication/374570877_Small_autonomous_machines_for_sustainable_soil_management_in_vegetable_crops_and_orchards)
  - "Swarm Robots in Agriculture". — [arXiv 2103.06732](https://arxiv.org/pdf/2103.06732)
  - "Review of Current Robotic Approaches for Precision Weed Management". — [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC9305686/)
  - Systematic review of intelligent mechanical weed control (2025). — [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC12756371/)
- **Measured or claimed work rates, collected for comparison**:

  | Robot | Rate | Source |
  |---|---|---|
  | Tertill | maintains about 18.6 m² per robot | [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/) |
  | Chinese intra-row prototype | about 171 m²/h (calc) | [ResearchGate](https://www.researchgate.net/publication/251682319_Design_and_testing_of_an_intra-row_mechanical_weeding_machine_for_corn) |
  | Odd.Bot Maverick | about 625 m²/h (calc from 16 h/ha) | [Future Farming](https://futurefarming.com/odd-bot-maverick) |
  | Aigamo Robo | about 1.5 ha in up to 10 h, about 1,500 m²/h (calc) | [Herald Corp](https://biz.heraldcorp.com/article/10863020) |
  | Greenfield, one robot | 0.5–1 acre/h, about 2,000–4,000 m²/h (company); UVM pilot about 2.8 ha/day | [Compass Vermont](https://www.compassvermont.com/p/vermont-farmers-see-robotic-weeders) |
  | FarmDroid | up to 6 ha/day | [FarmDroid](https://farmdroid.com/products/farmdroid-fd20/) |

### Inferences
- **Work-rate arithmetic (calc, assumption-laden).** Assume one inter-row strip of 0.76 m (30-in corn/soy) is treated per pass.
  - At 0.1 m/s the rate is about 274 m²/h, or about 36 robot-hours per ha. At 0.3 m/s it is about 820 m²/h, or about 12 robot-hours per ha.
  - A 15 cm robot whose tool only treats its own swath plus reach would likely need two passes per inter-row (one per crop row), roughly doubling those hours.
  - This puts a single unit in the same range as the slowest research prototypes and Odd.Bot. Field-scale use therefore implies fleets of tens of units per tens of hectares plus near-continuous operation, or a niche in small, high-value plots.
- **Compaction.** The compaction benefit of a robot of a few kg is real in principle, since ground pressure is negligible, but no study has field-validated compaction benefits even for 40–50 kg swarms.
- **Economics.** The economic case in the literature rests on removing operator labour and on small or irregular fields. It does not depend on robot size per se, and maintenance and logistics for many tiny units are the flagged risk.

### Gaps
- The Gaus et al. 2017 full numbers (seasonal €/ha, number of robots, comparison with conventional hoeing) could not be retrieved (OpenAgrar blocked).
- No source found directly compares cost/ha of small-robot weeding with hand weeding and with camera-guided tractor hoes. The Monte Carlo paper and the AgriEngineering 2023 paper probably contain such comparisons but were not retrieved.
- No review found gives an explicit "minimum practical size" for weeding robots.

---

## Q4. Documented failure modes and business/technical lessons from small ag-robot companies that shut down or pivoted (2022–2026)

### Takeaway
The dominant failure mode in 2024–2026 was financial rather than technical:
- Small Robot Company (liquidation 2024), FarmWise (closed April 2025), Naïo (receivership June 2025, sold for about €170k), AgroIntelli (bankruptcy early 2026), Guardian Agriculture (Aug 2025) and the consumer Tertill (discontinued about 2025) all ran out of runway or investor support.
- Technical lessons cluster around low throughput and narrow operating envelopes: terrain, weed size and grass weeds.
- Survivors shifted business models (RaaS to purchase) and narrowed product lines.

### Cited Findings
- **Small Robot Company (UK), liquidation in early 2024**:
  - It raised more than £11M over about 6.5 years.
  - Its lead investor pulled out, and an investment under a signed term sheet failed to arrive before the runway ran out. A crowdfunding round neared its initial £1.5M target in the first week but was not enough.
  - The CEO blamed the UK agtech investment climate and said the technology "delivered value at a profit" with customers waiting.
  - Kroll was reported as appointed to sell assets.
  - Product history: "Dick" (the weed zapper) never made it out of early trials; "Harry" was planting; "Wilma" was the AI. A Tom robot at the James Hutton Institute kept operating after the collapse.
  - One source's figure of "€19M revenue in 2023" is implausible and should be ignored.
  - Sources: [FarmingUK](https://www.farminguk.com/news/great-sadness-small-robot-company-put-into-liquidation_64099.html); [FWI](https://www.fwi.co.uk/business/small-robot-company-in-liquidation-with-farming-investors-at-risk); [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/small-robot-company-today-proceeds-into-liquidation); [The Robot Report](https://www.therobotreport.com/agtech-startup-small-robot-company-shutting-down/); [iGrow News](https://igrownews.com/small-robot-company-announces-liquidation/); [Scottish Farmer: Hutton robot operational](https://www.thescottishfarmer.co.uk/news/24105378.hutton-robot-operational-post-start-up-firms-collapse/); [Scottish Farmer: farewell Tom, Dick, Harry](https://www.thescottishfarmer.co.uk/arable/24105366.arable-matters---farewell-tom-dick-harry/); [Service Dealer](https://servicedealer.co.uk/latest-news/small-robot-company-enters-liquidaton)
- **FarmWise (USA, Vulcan/Titan weeders)**: closed operations on 1 April 2025, citing a venture-funding slump and farmers' financial pressures. It sought a buyer for its AI weeding technology, and one source says Taylor Farms acquired it days later. — [Fertilizer Daily](https://www.fertilizerdaily.com/20250403-farmwise-to-close-operations-exploring-sale-of-its-ai-weeding-technology/); [Farm Progress](https://www.farmprogress.com/technology/farmwise-closure-leaves-farmers-in-limbo-as-ag-tech-faces-investment-hurdles); [The Bot Scout](https://www.thebotscout.com/companies/farmwise/)
- **Naïo Technologies (France)**: receivership in June 2025, then a court-approved transfer to a new entity for about €170k with about 20 of 49 jobs kept (Nov 2025). The new owner cut Orio and Jo to focus on Oz and Ted. An earlier period of financial turmoil was also reported (AgFunderNews; date not captured). — [Réussir](https://www.reussir.fr/fruits-legumes/un-repreneur-pour-les-robots-naio-technologies); [CMS](https://cms.law/en/fra/news-information/cms-francis-lefebvre-conseille-les-repreneurs-de-naio-technologies); [Pleinchamp](https://www.pleinchamp.com/actualite/sanka-un-petit-robot-porte-outils-au-catalogue-de-naio); [AgFunderNews](https://agfundernews.com/ag-robotics-startup-naio-bounces-back-after-a-year-of-financial-turmoil)
- **AgroIntelli (Denmark, Robotti)**: a bankruptcy order in early 2026 after it could not meet financial obligations. Staff kept working and orders were produced while the receiver assessed takeover bids. — [AgInsights](https://www.aginsights.blog/agrointelli-files-for-bankruptcy-as-pioneer-of-robotti-autonomous-field-robot-seeks-new-investor/); [TractorEvolution](https://tractorevolution.com/tractor-news/danish-agricultural-robot-maker-agrointelli-files-for-bankruptcy-as-search-for-investor-begins/); [PreciseMag](https://precisemag.net/danish-robotics-firm-files-for-bankruptcy/)
- **Guardian Agriculture (spray drones)**: ceased operations in late Aug 2025 after earlier layoffs. — [The Robot Report](https://www.therobotreport.com/drone-startup-guardian-agriculture-shuts-down/)
- **Tertill (consumer)**: the product ended with no support, after "thousands of gardens". Reviewer-reported field problems: getting stuck on uneven ground, the need for a barrier, the need for a pre-cleaned bed, durability in humid climates, a 3.2-star Amazon rating, and no grass control in the peer-reviewed trial. — [tertill.com (search summary)](https://tertill.com/); [Bob Vila](https://www.bobvila.com/articles/tertill-weeding-robot-review/); [House Digest](https://www.housedigest.com/1866284/tertill-garden-weed-robot-tool-worth-buying-reviews); [Cambridge](https://www.cambridge.org/core/journals/weed-technology/article/abs/functionality-and-efficacy-of-franklin-robotics-tertill-robotic-weeder/07DF9F4329CB64D749E822168A097D92)
- **Business-model shifts among survivors**:
  - Greenfield moved from robots-as-a-service to outright sale at $36,500 per robot and opened public crowd-equity (2025).
  - Aigen sells at $50k per unit plus a $20k fleet fee.
  - Odd.Bot sold out a 16-unit batch.
  - Sources: [Compass Vermont](https://www.compassvermont.com/p/vermont-farmers-see-robotic-weeders); [Future Farming: Greenfield](https://www.futurefarming.com/tech-in-focus/field-robots/greenfield-robotics-opens-public-share-offering-after-6-years-in-the-field/); [Future Farming: Aigen](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops); [Precision Farming Dealer: Odd.Bot](https://www.precisionfarmingdealer.com/articles/6324-oddbot-secures-2-million-to-advance-autonomous-chemical-free-weeding-solutions)
- **Technical failure modes in under-canopy autonomy (CropFollow++)**: vision keypoint errors, physical robot failures, corn gaps, weeds/occlusion and planting errors. Most runs were shorter than 250 m between interventions. — [arXiv 2404.17718](https://arxiv.org/pdf/2404.17718v1.pdf)
- **Rowbot**: criticised as too heavy, diesel-powered and barely fitting corn rows, with risk of crop damage (patent critique). — [USPTO](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11789453)
- **WEEDINATOR**: hydraulic oscillation at speed; still needs operator involvement. — [Hackaday.io](https://hackaday.io/project/204846-weedinator-2026-agricultural-robot)
- **Commentary sources located but not retrievable (fetch blocked)**: "Agtech Boom and Bust, 2017–2026: Six Stories" (OneSoil) and "Agriculture Robotics is difficult AF" (Rhishi Pethe). — [OneSoil](https://onesoil.ai/en/resources/agtech-boom-and-bust-six-stories); [Substack](https://sftw.substack.com/p/agriculture-robotics-is-difficult)

### Inferences
- **Small Robot Company.** Its collapse shows that a "small robots, per-plant" thesis can be technically credible yet die on capital intensity and a slow route to revenue. Its weeding robot (Dick) never left early trials, and only the scouting robot reached commercial service.
- **Financial pattern.** Companies kept alive by acquirers or crowd-equity (Naïo, FarmWise to Taylor Farms, Greenfield) survived by narrowing products and selling hardware outright. Pure service models and multi-product portfolios did not survive the 2024–2025 funding slump.
- **Lessons for a tiny rover**, from Tertill and CropFollow++:
  - plan for frequent interventions;
  - define a narrow operating envelope (bed prep, weed stage, crop stage);
  - expect grasses to escape low-force tools;
  - design for humidity, mud and sealing early.

### Gaps
- Earth Rover, Tensorfield and Kilter: no shutdown or pivot news found. The Kilter/AX-1 swarm appears in a 2026 simulation paper, but the company's status was not found.
- No primary post-mortem (founder interview or detailed lessons) was retrieved for Small Robot Company or Tertill; The Robot Report and OneSoil pages were blocked.

---

## Q5. Has swarm operation been demonstrated in the field with measured results?

### Takeaway
Fleets of small-to-mid robots run commercially, but no peer-reviewed measured field results for a coordinated swarm of small ground weeding robots were found:
- Greenfield runs about 25 weeding robots (about 90–160 kg each).
- Aigen ran about 50 units in 2025, 30 of them on three farms.
- EarthSense ran three under-canopy Cover Crop Robots (25 km of logged autonomy).

Published swarm performance numbers are manufacturer claims (Fendt Xaver, seeding), simulations (AX-1 swarm, 2026) or aerial swarms (SAGA).

### Cited Findings
- **Greenfield Robotics**: a fleet of 25 in the Midwest (2024). Claimed rates: about 10 acres/h per 10 robots and 30–100 acres/day. The independent UVM pilot measured about 7 acres/day for one robot. No audited season total was found. — [The New Lede](https://www.thenewlede.org/2024/07/in-bid-to-slash-chemical-use-robots-take-on-farm-fields/); [Compass Vermont](https://www.compassvermont.com/p/vermont-farmers-see-robotic-weeders); [UVM Extension](https://www.uvm.edu/extension/nwcrops/news/recap-first-public-demonstration-autonomous-weeding-robots-vermont)
- **Aigen**: 50 operational units in 2025, 30 running on three farms. Weed-control efficacy was not published in retrieved sources. — [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/aigen-launches-solar-powered-element-gen2-robot-for-weeding-crops)
- **EarthSense CCR**: three robots, 25,315 m total under corn canopy, planting cover crops (a fleet of under-canopy robots with measured autonomy, but seeding, not weeding). — [arXiv 2404.17718](https://arxiv.org/pdf/2404.17718v1.pdf)
- **Fendt Xaver/MARS**: claims of 6–12 units at about 1 ha/h and six units at about 3 ha/h (2 ha/h with loading), with automatic re-planning if a unit fails. These are manufacturer claims for maize seeding, with no independent data found. — [FWI](https://www.fwi.co.uk/machinery/technology/fendt-proposes-swarm-robots-corn-planting); [Irish Examiner](https://www.irishexaminer.com/farming/arid-30811455.html)
- **"Agricultural weed control by a swarm of robots: field decomposition and workload distribution"** (Artificial Life and Robotics, 2026): an AX-1 robot swarm studied in simulation only. More robots shorten field completion time. — [Springer](https://link.springer.com/article/10.1007/s10015-026-01157-y)
- **SAGA project**: a swarm of UAVs field-tested for weed monitoring and weeding tasks (aerial, not ground). — [arXiv "Past, Present, and Future of Swarm Robotics"](https://arxiv.org/pdf/2101.00671)
- **Small Robot Company**: operated individual robots, not a swarm, before liquidation. — [Future Farming](https://futurefarming.com/tech-in-focus/field-robots/small-robot-company-today-proceeds-into-liquidation)

### Inferences
- **What a tiny-rover swarm would add.** A swarm of 15 x 20 cm weeding rovers would be novel at two levels: the robot size, and measured swarm weeding results. The nearest real-world evidence for "many cheap units sharing a field" is Greenfield and Aigen. Both use far larger robots, and both treat coordination as fleet management (independent row assignments) rather than tight swarm behaviour.

### Gaps
- No field-measured efficacy or throughput from any small-robot weeding swarm was found.
- Aigen and Greenfield have not published efficacy (percent weed control) in retrieved sources.

---

## Q6. Which precedents work within the RGB + IMU + optional GPS and Raspberry Pi / small Jetson constraints?

### Takeaway
RGB-only perception on small embedded compute is well precedented for navigation and weed classification (CropFollow, CropFollow++, SAMBot on Jetson Nano, Nindamani on Jetson Nano, OWL on Raspberry Pi 4, LettuceThink on Raspberry Pi). The small robots that actually weeded in the field avoided vision entirely: Tertill used a height rule, Aigamo Robo is non-selective, and FarmDroid uses RTK seed maps. Combining RGB under-canopy navigation with selective weeding on a Pi/Jetson-class computer in a rover this small is therefore an unproven combination.

### Cited Findings
- **CropFollow**: a single front RGB camera on TerraSentia gave 485 m per intervention, beating LiDAR (286 m), over 25 km. — [RSS 2021](https://www.roboticsproceedings.org/rss17/p019.pdf)
- **CropFollow++**: low-cost RGB cameras with keypoint-based row perception and MPC; 25.3 km on three robots. — [arXiv 2404.17718](https://arxiv.org/pdf/2404.17718v1.pdf)
- **SAMBot**: Jetson Nano running a TensorRT MobileNetV2; about 90% weed/crop classification; 47.78 ms inference. — [arXiv 2112.13986](https://arxiv.org/abs/2112.13986)
- **Nindamani**: Jetson Nano with ROS 2. — [NVIDIA](https://developer.nvidia.com/embedded/community/jetson-projects/nindamani)
- **WEEDINATOR**: Jetson Nano running YOLO26n and OpenCV (large platform). — [Hackaday.io](https://hackaday.io/project/204846-weedinator-2026-agricultural-robot)
- **OpenWeedLocator**: Raspberry Pi 4 + camera for real-time weed detection and spot spraying. — [Raspberry Pi magazine](https://magazine.raspberrypi.com/articles/open-weed-locator)
- **LettuceThink**: Raspberry Pi + Arduino. — [EFITA 2017](https://csl.sony.fr/wp-content/uploads/2017/01/ColliauxHanappe_109_EFITA2017.pdf)
- **Digital Farmhand**: smartphone-based compute plus two cameras and RTK GNSS (Emlid Reach RS+). — [Good Fruit & Veg](https://www.goodfruitandvegetables.com.au/story/7433579/robotics-revolution-rolling-out-across-australia/); [Emlid](https://blog.emlid.com/agricultural-robot-powered-with-reach/)
- **Tertill**: no camera; height-based weed rule. — [tertill.com FAQ (search summary)](https://tertill.com/pages/faqs)
- **Aigamo Robo**: ridge detection, with GPS mentioned; non-selective action. — [Herald Corp](https://biz.heraldcorp.com/article/10863020)
- **FarmDroid**: RTK GPS weeding between rows and plants (no camera needed for in-row work). — [FarmDroid](https://farmdroid.com/products/farmdroid-fd20/)
- **Under-canopy GPS**: ROW-SLAM notes the extreme close range between camera and plants under corn canopy. CropFollow chose camera-based relative navigation over absolute positioning. — [arXiv 2109.07134](https://arxiv.org/pdf/2109.07134); [RSS 2021](https://www.roboticsproceedings.org/rss17/p019.pdf). (That GNSS is unreliable under a closed canopy is general knowledge in the field, but no retrieved source in this session states it explicitly; treat it as an inference.)

### Inferences
- **Navigation.** CropFollow/CropFollow++ is the most directly reusable precedent for the proposed rover's navigation: RGB keypoint row-following plus a predictive controller on a small platform. Even so, its intervention rate (most runs under 250 m) implies frequent human help unless recovery behaviours are added.
- **Detection.** On-board weed classification at Jetson Nano scale (about 50 ms per frame with MobileNetV2) is feasible. Raspberry Pi 4 has been used for detection (OWL), but in open, above-crop views rather than under canopy.

### Gaps
- The compute hardware of TerraSentia/CCR (as used in CropFollow++) was not captured.
- No precedent was found that combines RGB-only under-canopy navigation with selective weeding on a Raspberry Pi.
