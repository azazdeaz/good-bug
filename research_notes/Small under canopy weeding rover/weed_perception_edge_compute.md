# RGB-only weed and crop perception on small embedded compute for a ground-level, under-canopy weeding rover (primary crop: pepper/chili, Capsicum annuum), state of the art to November 2026

Method note for the report writer: none of the full-text pages could be fetched in this session. WebFetch failed with DNS/allowlist errors for docs.ultralytics.com, wiki.seeedstudio.com, arxiv.org, frontiersin.org, pmc.ncbi.nlm.nih.gov, global-agriculture.com and data.amerigeoss.org. Every figure below therefore comes from search-engine extracts of the cited pages, including those salvaged from an earlier researcher's session. Treat single-source numbers as "reported", and check them against the full text before using them as headline figures. Where an extract did not say clearly which page a number came from, I say so.

## 1. Simplifying the problem: green-on-brown in the inter-row, green-on-green near the row, colour indices, and protecting the crop by its stem

### Takeaway
In the bare-soil inter-row, ExG/HSV colour thresholding (as in OpenWeedLocator, OWL) runs in real time on a Raspberry Pi 4/5 without any accelerator. Reported field figures are about 79% precision and 52% recall across fallow fields, rising to 92% precision and 74% recall at individual sites, and up to 95.7% recall with a global-shutter camera at 5 km/h. Shadows are the main documented failure mode. Colour alone cannot separate crop from weed near the row: every green pixel, including low pepper leaves, fallen green leaves and moss, becomes a "weed". The practical simplification is therefore a hybrid: colour thresholding for "all green" plus a learned detector for "crop / crop stem", with a keep-out radius around each crop stem.

### Cited Findings
**OpenWeedLocator (OWL), the reference Raspberry Pi green-on-brown system**
- OWL hardware in the original paper was a Raspberry Pi 4 (8 GB) with a Raspberry Pi HQ Camera (Sony IMX477). Images are resized to 416 × 320 px "to ensure high processing throughput on the Raspberry Pi platform". — [Coleman et al., Scientific Reports 2022 (s41598-021-03858-9)](https://springerlink.bibliotecabuap.elogim.com/content/pdf/10.1038/s41598-021-03858-9.pdf)
- The algorithms tested were ExG (excess green; Woebbecke et al. 1995), an HSV threshold, and a combined ExG + HSV method. The team settled on ExG + HSV, with the HSV step meant to reduce false detections in bright or overexposed areas. — [OWL README mirror](https://git.tdem.in/geezacoleman/OpenWeedLocator); [Sprayers101](https://sprayers101.com/?p=14555)
- OWL runs on a Raspberry Pi 3B+, 4 or 5; Pi 5 + picamera2 support was added in April 2024. Green-on-brown works on the Pi. Green-on-green was "still in development" and needs extra hardware or a model. — [OWL GitHub](https://github.com/geezacoleman/OpenWeedLocator); [SourcePulse summary](https://www.sourcepulse.org/projects/2338961); [openweedlocator-tools](https://github.imc.re/geezacoleman/openweedlocator-tools)
- Fallow-field trial across seven fields: overall about 79% precision and about 52% recall, with individual sites up to 92% precision and 74% recall. — [CAWS 2022 paper](https://caws.org.nz/wp-content/uploads/2023/02/0218.pdf); [GRDC GroundCover summary](https://groundcover.grdc.com.au/innovation/precision-agriculture-and-machinery/weed-detection-technologies-taking-flight)
- Speed and camera study (2023; tillage radish = broadleaf, forage oats = grass; ExG+HSV):
  - Recall fell with speed (5 → 30 km/h) for every camera.
  - The global-shutter Arducam AR0234 reached 95.7% recall at 5 km/h and 85.7% at 30 km/h.
  - The Pi HQ and Pi V2 cameras fell to 31.1% and 26.0% recall at 30 km/h.
  - Precision stayed near-perfect at all speeds.
  - A second extract says the "current Raspberry Pi HQ Camera + latest software" gave 74.8% recall at 5 km/h and 50.5% at 30 km/h. The two HQ figures at 30 km/h (31.1% vs 50.5%) probably refer to old vs updated software; I could not confirm this.
  — [arXiv 2305.10311](https://arxiv.org/pdf/2305.10311); [deeppaper summary](https://arxiv.deeppaper.ai/papers/2305.10311v1?locale=en); [DeepAI](https://deepai.org/publication/investigating-image-based-fallow-weed-detection-performance-on-raphanus-sativus-and-avena-sativa-at-speeds-up-to-30-km-h-1)
- Throughput: a secondary trade article says non-ML colour methods run in real time at "around 15–20 frames analysed per second" on credit-card computers such as the Raspberry Pi and Jetson Nano. This is not the paper's own figure. — [Sprayers101](https://sprayers101.com/?p=14555)

**Colour-index accuracy and failure modes**
- Shadows are often classified as vegetation by ExG-based segmentation. ExG "often provides poor segmentation results especially when the image scene contains an extreme illumination difference caused by dark shadows." — [Springer, Precision Agriculture s11119-017-9511-z](https://link.springer.com/article/10.1007/s11119-017-9511-z); [search summary of ExG shadow literature](https://research.wur.nl/en/publications/shadow-resistant-segmentation-based-on-illumination-invariant-ima/)
- With an HDR camera plus ExG + Otsu, ground-shadow removal improved precision by 20%, specificity by 4.4% and modified accuracy by 13.5% on average, at 0.46 s processing per image. Shadows "change dramatically throughout the day". — [HDR shadow-removal paper (Bohrium listing)](https://www.bohrium.com/paper-details/improved-vegetation-segmentation-with-ground-shadow-removal-using-an-hdr-camera/811054645446180864-1389)
- A global threshold on an illumination-invariant transform outperformed ExG. — [WUR, shadow-resistant segmentation](https://research.wur.nl/en/publications/shadow-resistant-segmentation-based-on-illumination-invariant-ima/)
- On sugar-beet canopy images, ExG−ExR with a manually set threshold reached 94.69% accuracy and ExG + Otsu reached 87.52%. — [Univ. of Tehran journal paper](https://journals.ut.ac.ir/article_87641_dd04186b1426cc39291f08aba93025c8.pdf)
- In paddy fields, Otsu/ExG produced "a lot of image noise" under varying illumination. — [IJABE](https://ijabe.org/index.php/ijabe/article/download/1932/pdf)

**Crop-stem detection as a way to protect the crop**
- Lottes et al. built a fully convolutional network that jointly estimates plant stem positions and pixel-wise crop/weed extent. It "outputs the stem location for weeds, which allows for mechanical treatments, and the covered area of the weed for selective spraying." The 2019 J. Field Robotics version uses image sequences along the row and "substantially improves the stem detection accuracy" and generalisation to unseen fields. — [arXiv 1806.03413](https://arxiv.org/pdf/1806.03413); [Lottes 2019 JFR PDF](https://www.ipb.uni-bonn.de/pdfs/lottes2019jfr.pdf)
- A grassland multi-task network (keypoint stem detection + weed boxes) reached OKS 56.3 for joint stem detection at 12.2 FPS on an NVIDIA Jetson Xavier NX. — [MDPI Agronomy 13(9):2365](https://www.mdpi.com/2073-4395/13/9/2365/html)
- The Weed Stem Detection (WSD) dataset and method (AAAI 2025; YOLOv7 backbone + MSE stem-regression head) found that "merely achieving high mAP does not guarantee effective stem localization". — [arXiv 2502.06255](https://arxiv.org/pdf/2502.06255); [AAAI](https://ojs.aaai.org/index.php/AAAI/article/view/35040)
- A non-camera precedent: the very small Tertill garden robot (a comparable footprint class) uses only a height rule. Capacitive front sensors detect tall plants and the robot turns away; a bottom sensor detects short plants and triggers a string trimmer. "Plants that can pass under the robot are assumed to be weeds." One study gives the threshold as < 2.5 cm, and young crop plants need metal guards. — [Tertill support: sensing plants](https://support.tertill.com/article/20-sensing-plants-and-weeds); [Tertill FAQ](https://tertill.com/pages/faqs); [MDPI Agronomy 12(8):1754](https://mdpi.com/2073-4395/12/8/1754); [Tertill: protecting small plants](https://support.tertill.com/article/15-protecting-small-plants)
- Bonn's "Weeds are Weird" zero-shot method (IROS 2025) also uses a population prior rather than labels: the crop is "the most commonly occurring plant" and weeds are "weird-looking" plants, using SAM + BioCLIP. — [Chong et al. 2025 PDF](https://www.ipb.uni-bonn.de/wp-content/papercite-data/pdf/chong2025iros.pdf); [project page](https://magistri.github.io/publication/2025-10-31-chong2025iros)

### Inferences
- (Inference) In the inter-row of transplanted peppers, the rule "every green blob is a weed" is valid only while (a) no pepper leaves hang into the camera footprint and (b) there is no green non-weed material on the ground: fallen green pepper leaves, moss/algae on wet soil, or green plastic. Under a closing canopy, (a) is violated first. So OWL-style colour detection needs an explicit "crop mask" or a geometric keep-out band along the row.
- (Inference) A low camera under the chassis with a skirt and its own LED lighting removes most sun shadows and dappled light. These are the dominant ExG failure in the literature above. Black PE mulch and black drip tape are not green, so ExG/HSV should reject them. Green or white mulch, specular wet plastic, and green algae on drip lines are plausible false-positive sources. I found no measurements for these specific materials (see Gaps).
- (Inference) Pepper is transplanted, has a single upright stem, and sits on a known row line at known spacing. Detecting one class, "pepper stem / pepper plant", is easier than multi-species weed recognition. A per-plant keep-out radius plus "all other green = weed" would mirror the Tertill (height) and Weeds-are-Weird (most-common plant) priors. This is likely sufficient for inter-row and near-row treatment. Discriminating weeds in direct contact with the crop stem remains hard (see section 7 on Solanum nigrum).
- (Inference) For a 15 × 20 cm rover with the camera 5–25 cm above soil, the camera is close enough that OWL's 416 × 320 px processing resolution still gives sub-millimetre to ~1 mm per pixel (see section 4). Missed tiny weeds would come mostly from blur and exposure, not from resolution.

### Gaps
- No ExG-only vs HSV-only vs ExG+HSV precision/recall breakdown was retrievable from the OWL papers (the full text could not be fetched).
- No measured precision/recall was found for colour indices under a crop canopy, with crop residue, moss, fallen leaves, plastic mulch or drip tape. These failure modes are inferred only.
- No study was found that uses crop-stem detection alone as the crop-protection mechanism for pepper.
- The OWL frame rate on Pi 4/5 is not confirmed from a primary source (only a secondary figure of 15–20 FPS).

## 2. Deep-learning detection and segmentation on edge hardware: accuracy and FPS/latency

### Takeaway
Rough order of magnitude for nano-class YOLO at 640 px:

| Hardware | Reported speed |
|---|---|
| Raspberry Pi 5 CPU (NCNN) | ~65–130 ms/frame (≈ 8–15 FPS, inference only); light models 3–5 FPS end-to-end in an embedded-weed paper |
| Pi 5 + Hailo-8L (AI Kit / AI HAT+ 13 TOPS) | ≈ 30–80 FPS for YOLOv8s end-to-end, depending on the pipeline and PCIe Gen 2 vs Gen 3 |
| Pi 5 + Hailo-8 (26 TOPS) | ~80–190 FPS for YOLOv8n/s (vendor figures are upper bounds measured on PCIe x4) |
| Jetson Nano (legacy) | ~25–31 FPS for YOLOv5/segmentation in ag papers |
| Jetson Orin Nano (Super) TensorRT | ~3.5–5 ms per image for YOLOv8n/YOLO11 (FP16/INT8 engine-only, ≈ 200–290 FPS), though end-to-end pipelines report far lower |

Any of these is fast enough for a rover at 0.1–0.5 m/s, except the Pi CPU at the upper speed. The deciding factors are field accuracy (domain shift) and end-to-end latency, not peak FPS.

### Cited Findings
**Raspberry Pi 4/5 CPU**
- YOLO11n on Pi 5: about 94 ms with NCNN and about 85 ms with OpenVINO. This is a secondhand quote of the Ultralytics Pi guide on the Ultralytics forum and could not be verified. — [Ultralytics community forum](https://community.ultralytics.com/t/raspberry-pi-5-crash-when-running-yolov11n-ncnn/1525)
- The current Ultralytics Raspberry Pi guide benchmarks YOLO26. It lists YOLO26n at 128.42 ms/image on Pi 5 (Ultralytics 8.4.14); the export format was not given in the extract. — [Ultralytics Raspberry Pi guide](https://docs.ultralytics.com/guides/raspberry-pi)
- Arm's runtime guide: YOLO26n on Pi 5 at 67.20 ms/image with NCNN, excluding pre- and post-processing. — [Arm Learning Path, YOLO26 on Raspberry Pi 5](https://learn.arm.com/learning-paths/embedded-and-microcontrollers/yolo26-arm-runtime/compare-performance/)
- An older mirrored Ultralytics table lists YOLOv8n NCNN at 211.26 ms/image on Pi 5. This is inconsistent with the newer numbers; differences in version, timing scope or setup are likely. — [HF mirror of Ultralytics Pi guide](https://huggingface.co/akswelh/Test_model1/blob/main/docs/en/guides/raspberry-pi.md)
- Raspberry Pi 4B (4 GB), YOLOv8n TFLite INT8 at 640 × 640 with 4 threads: about 93 ms P50, about 10.7 FPS. The extract's table headers were ambiguous, so this has low confidence. — [wikidocs hardware benchmark](https://wikidocs.net/336241)
- An embedded weed-detection paper (Albanbay et al.) reports that light models reached 3–5 FPS on Raspberry Pi 5. Its most balanced model reached mAP@0.5 = 94.6% and 207 FPS on a workstation. — [Glasgow eprints 394099](https://eprints.gla.ac.uk/394099/2/394099.pdf)
- LiteWeed: YOLOX-nano on Raspberry Pi 5, mAP 0.32, about 118 ms per detection. — [srsweb preprint](https://radiopharmaconnect.srsweb.org/doi/pdf/10.22541/au.173297211.11959193/v1); [Global Agriculture news](https://www.global-agriculture.com/ag-tech-research-news/low-cost-laser-weeding-robot-built-for-smallholder-farms-passes-field-tests-in-canada-and-tanzania/)

**Raspberry Pi 5 + Hailo (AI Kit / AI HAT+ / AI HAT+ 2)**
- The AI HAT+ comes in two versions: Hailo-8L (13 TOPS INT8) and Hailo-8 (26 TOPS INT8). — [Raspberry Pi AI HAT+ docs](https://raspberrypi.com/documentation/accessories/ai-hat-plus.html)
- An AI HAT+ 2 with a Hailo-10H (40 TOPS, 8 GB on-board RAM) also exists. — [raspberry.tips](https://raspberry.tips/?p=9674)
- YOLOv8s on Pi 5 + Hailo-8L (AI Kit), INT8, 640 × 640, batch 1: 80.7 FPS. A full 1080p camera pipeline gave about 30 FPS, with the gap attributed to camera preprocessing. — [Seeed wiki benchmark](https://wiki.seeedstudio.com/benchmark_on_rpi5_and_cm4_running_yolov8s_with_rpi_ai_kit/); [Hailo community](https://community.hailo.ai/t/raspberry-pi-5-with-hailo-8l-expected-performance/746)
- Hailo Model Zoo figures (YOLOv8s: 491 FPS on Hailo-8, 110 FPS on Hailo-8L) were measured with four PCIe lanes on a desktop CPU. Hailo staff say a Pi 5 (one lane) will not reach them, so treat them as upper bounds. — [Hailo community thread](https://community.hailo.ai/t/the-performance-on-the-raspberry-pi-5-with-the-hailo-8-chip-seems-not-good-as-he-official-results/17473); [Hailo community YOLOv8s results](https://community.hailo.ai/t/yolov8s-performance-results/18146)
- The Pi 5 PCIe link is Gen 2 x1 by default. Forcing Gen 3 roughly doubled YOLOv8s on a Hailo-8L (CM5 8 GB): 28.59 FPS on Gen 2 vs 55.73 FPS on Gen 3. — [Hailo community, Issue with YOLO FPS on Raspberry Pi 5](https://community.hailo.ai/t/issue-with-yolo-fps-on-raspberry-pi-5/11706)
- Hailo's published YOLOv8n (640 × 640) figures are 140 FPS on Hailo-8L and 270 FPS on Hailo-8 (pure inference, batch 1). — [SpecPicks review](https://specpicks.com/reviews/hailo-8-vs-hailo-8l-vs-coral-tpu-pi-5-2026)
- Other reported Pi 5 numbers for the AI HAT+ 26 TOPS:
  - 136 FPS for YOLOv8n (batch 8) and ~80 FPS for YOLOv8s. — [raspberry.tips](https://raspberry.tips/en/raspberrypi-tutorials/raspberry-pi-ai-hat-plus-ai-acceleration)
  - ~190 FPS for YOLOv8n. — [SpecPicks](https://specpicks.com/reviews/raspberry-pi-ai-hat-plus-26-tops-what-it-runs-2026)
  - Sixfab measured the Hailo-8 AI HAT+ at 121.9 FPS on DeepLabV3 (semantic segmentation) and 19.6 FPS on YOLOv8l. — [Sixfab DEEPX vs Hailo](https://sixfab.com/blog/raspberry-pi-5-ai-hat-benchmark-deepx-vs-hailo/)

**Coral Edge TPU**
- The Ultralytics Coral guide (Pi + USB Coral, inference only) reports that the Pi 5 is 30.2% faster than the Pi 4B, and that the Edge TPU "high-frequency" mode is 28.4% faster than standard but can throttle thermally. — [Ultralytics Coral Edge TPU on Raspberry Pi guide](https://docs.ultralytics.com/ko/guides/coral-edge-tpu-on-raspberry-pi)
- No weed-specific Coral FPS was found. In a peach-detection study (Pi 4 + Coral, SSD), Edge-TPU conversion had a "slight impact on detection accuracy but a great advantage in inference time". — [Bohrium listing](https://www.bohrium.com/paper-details/real-time-image-detection-for-edge-devices-a-peach-fruit-detection-application/817350201214763009-21695)

**Jetson Nano (original, Maxwell)**
- A Jetson Nano spray-system segmentation network (UBI, Remote Sensing 2022) ran at 25 FPS at 513 × 513 and 5.9 FPS at 1296 × 966 with TensorRT. TensorRT gave a 14.8× speed-up but mIoU fell by 14.7%. — [MDPI Remote Sensing 14(17):4217](https://mdpi.com/2072-4292/14/17/4217)
- YOLOv5 on Jetson Nano: 29 FPS (carrot) and 31 FPS (onion) weed datasets. — [Computer Optics 49(1), 2025](https://www.computeroptics.ru/KO/PDF/KO49-1/490113.pdf)
- A Seeed table lists YOLOv8n TensorRT FP16 at 120 FPS, but the extract does not say which Jetson board, so it cannot be attributed to the Nano. — [Seeed YOLOv8-TRT-Jetson wiki](https://wiki.seeedstudio.com/ja/YOLOv8-TRT-Jetson)
- SAMBot, a ~US$500 compact robot, runs weed classification on a Jetson Nano (flax). — [arXiv 2112.02162](https://arxiv.org/pdf/2112.02162.pdf)

**Jetson Orin Nano / Orin Nano Super / Orin NX**
- YOLOv8n on Orin Nano 8 GB in MAXN_SUPER mode, 640 × 640, batch 1: TensorRT FP16 4.43 ms (≈ 225 FPS) and INT8 3.49 ms (≈ 286 FPS). — [hokwangchoi/jetson-orin-nano-benchmarks](https://github.com/hokwangchoi/jetson-orin-nano-benchmarks)
- Other YOLOv8n FP16 reports on Orin Nano are much lower: ~90 FPS raw with trtexec ([Hackster](https://www.hackster.io/qwe018931/pushing-limits-yolov8-vs-v26-on-jetson-orin-nano-b89267)) and ~35 FPS in an end-to-end user pipeline ([Reddit r/JetsonNano](https://redlib.belloworld.it/r/JetsonNano/comments/1e187cc/yolov8_performances_on_jetson_orin_nano_4gb)). The spread reflects engine-only vs end-to-end timing.
- YOLO11 on Orin Nano (640, model size not stated; third-party wiki): FP16 4.91 ms vs INT8 3.91 ms. mAP50-95 dropped from 0.6096 (FP16) to 0.3180 (INT8), so naive INT8 calibration can be very costly. — [CamThink NeoEdge NG4500 wiki](https://camthink-ai.github.io/wiki-documents/en/docs/NeoEdge%20NG4500%20Series/Application%20Guide/Object%20Detection)
- Ultralytics says Orin Nano Super YOLO11 inference times are comparable to, and occasionally better than, the Orin NX 16 GB without super mode. — [Ultralytics blog](https://www.ultralytics.com/blog/ultralytics-yolo11-on-nvidia-jetson-orin-nano-super-fast-and-efficient)
- Ultralytics DeepStream guide on Orin NX 16 GB, 640 × 640:
  - YOLOv8s: FP32 15.63 ms, FP16 7.94 ms, INT8 5.53 ms.
  - YOLO11: 8.64 ms (FP32), 5.27 ms (FP16), 4.54 ms (INT8).
  — [Ultralytics DeepStream on Jetson guide](https://docs.ultralytics.com/guides/deepstream-nvidia-jetson)
- YOLO26n TensorRT on Orin Nano (Ultralytics JetPack 6 Docker; anecdotal): about 24 ms inference at imgsz 960 with half precision, plus large overhead elsewhere in the pipeline. — [Ultralytics forum](https://community.ultralytics.com/t/jetson-orin-nano-extreme-time-overhead/1787)
- The YOLO26 paper benchmarks on Jetson Nano and Orin; per-model numbers could not be retrieved. — [arXiv 2509.25164](https://arxiv.org/html/2509.25164v5)
- Crop/weed models on Orin-class boards:
  - YOLO11n on a Jetson Orin Nano, used on a Farm-ng Amiga robot with field trials in soybean and corn (spraying). — [arXiv 2507.05432](https://arxiv.org/html/2507.05432v1)
  - A YOLOv8 instance-segmentation laser-weeding model (BiFPN + dynamic snake convolution) ran at 24.8 FPS on Jetson Orin Nano, with a 6.8 MB model. — [Preprints 202405.0018](https://www.preprints.org/manuscript/202405.0018)

**Lightweight semantic segmentation on Jetson**
- An attention-aided lightweight crop/weed network (0.11 M parameters, 0.24 GFLOPs; tested on BoniRob, rice seeding and WeedMap) ran at 18.14 ms / 55.1 FPS on a Jetson Xavier NX. — [Frontiers Plant Sci 2023, 10.3389/fpls.2023.1320448](https://doi.org/10.3389/fpls.2023.1320448)
- A UAV weed-mapping lightweight segmentation network reached 80.9% accuracy at 4.5 FPS on a Jetson TX2. — [MDPI Applied Sciences 10(20):7132](https://mdpi.com/2076-3417/10/20/7132)
- YolactEdge exceeds 30 FPS on AGX Xavier (generic, not crop-specific). — [Omdena instance segmentation blog](https://www.omdena.com/blog/instance-segmentation)
- Older references: DeepWeeds ResNet-50 classification took 53.4 ms per image (pre-processing + inference) on a Jetson TX2 with TensorRT. The TensorFlow run took about 180 ms and missed the 100 ms real-time target. — [DeepWeeds arXiv 1810.05726](https://arxiv.org/pdf/1810.05726)

**Transformer detectors and on-device cameras**
- A 2025 preprint compares YOLOv8/v9/v10 and RT-DETR for real-time weed detection; its numbers were not retrieved. — [arXiv 2501.17387](https://arxiv.org/pdf/2501.17387)
- Field spot spraying with YOLOv4 on a Jetson AGX Orin (not a low-cost board): 90.6% effective spray rate in the field vs 93.33% indoors, with 95.5% accuracy and 89.47% recall in the field. The drop was attributed to lighting, shadows and wind. — [J. Agric. Food Res., Dec 2024 (DOAJ)](https://doaj.org/article/4ef11f36bbdd4cd9b4129b456c7e0f22)
- YOLOv4-tiny reached about 52 FPS on a Jetson AGX Xavier (Sensors 2024). — [DOAJ](https://doaj.org/article/25722c5df28347798456c7442e2f816c)
- OAK-D-Lite: Myriad X VPU (~4 TOPS), 13 MP IMX214 colour camera plus two OV7251 stereo cameras. Roboflow reports ~15 ms per inference (~66 FPS) with an M1 MacBook host and warns that the host strongly affects FPS. No weed-specific OAK benchmark was found. Farm-ING uses the OAK-D S2 PoE for on-device weed control. — [Roboflow Luxonis OAK docs](https://docs.roboflow.com/deploy/sdks/luxonis-oak); [Zbotic OAK-D Lite review](https://zbotic.in/oak-d-lite-ai-camera-depth-ai-on-edge-device-review/); [Luxonis forum, Farm-ING](https://discuss.luxonis.com/d/6849-farm-ing-luxonis-powering-high-precision-ai-for-sustainable-agriculture)
- A microcontroller tier exists: hardware-aware YOLO compression for weed detection on an STM32U5 (2025). — [arXiv 2511.07990 summary](https://fugumt.com/fugumt/paper_check/2511.07990v1_enmode)

### Inferences
- (Inference) Compute cost scales roughly with pixel count. Dropping from 640 to 320 px input would cut a Pi 5 CPU nano-YOLO from ~65–130 ms to roughly 20–35 ms (≈ 30–50 FPS of raw inference). Because the camera is 5–25 cm from the soil, 320 px still resolves ~0.4–1.9 mm/px (section 4). That makes CPU-only Pi 5 detection feasible for slow rovers. It needs validation: small cotyledon-stage weeds may vanish at 320 px if the camera is high.
- (Inference) Hardware fit:
  - **Pi 4 CPU:** colour indices only.
  - **Pi 5 CPU:** colour + one nano detector at low resolution.
  - **Pi 5 + Hailo-8L / Hailo-8:** detector + segmentation concurrently at camera rate. Hailo-8 runs DeepLabV3 at ~120 FPS.
  - **Orin Nano Super:** detector + segmentation + tracking, and potentially small foundation models (NanoOWL/NanoSAM, section 3) for on-line auto-labelling.
  - **Jetson Nano (original):** works at ~25–30 FPS for YOLOv5-class models in published ag papers, but it is a legacy platform (see Gaps).
- (Inference) INT8 should be validated on in-field data. The CamThink FP16→INT8 mAP collapse and the UBI TensorRT mIoU drop (−14.7%) show that quantisation can cost more accuracy than the speed gain is worth. FP16 on Orin, or Hailo's INT8 compiler with proper calibration, is the safer default.

### Gaps
- Official Ultralytics Pi 4/Pi 5/Orin Nano Super benchmark tables (per-format ms and mAP) and the Seeed Hailo table could not be fetched. The numbers above are secondhand.
- No measured FPS was found for BiSeNet, Fast-SCNN, DeepLabV3-MobileNet or SegFormer-B0 on Pi 5 CPU, Hailo or Orin Nano specifically for crop/weed data. The only crop/weed lightweight-segmentation figure is on a Xavier NX. Hailo DeepLabV3 at 121.9 FPS is generic.
- No RT-DETR edge latency on these boards was retrieved.
- No YOLO11 or YOLO26 measurement on the Hailo AI HAT+ was found.
- No primary source on JetPack support status for the original Jetson Nano (JetPack 4.x, end-of-life) was retrieved.

## 3. Datasets, pretrained models, how many images a new field needs, synthetic data, and foundation models

### Takeaway
There is one pepper-specific crop/weed dataset with pixel labels: Trinidad and Tobago hot pepper, with weed/crop/background classes. There is also a small Roboflow chili set. The large general resources are CropAndWeed (74 species, ~112k instances, stem positions), Weed25, DeepWeeds, PhenoBench (UAV), CWFID and the WSD stem dataset. None was captured at 5–25 cm under a canopy, so own-field data from the rover's own camera is needed. Recent evidence suggests ~25 labelled target images already beat unsupervised adaptation, and a few hundred is a common practical target. Zero-shot foundation models (Grounding DINO, OWL-ViT) are poor at crop-vs-weed: PhenoBench mAP 12.6 zero-shot vs 38.2 after 24-shot adaptation. They are best used as offline auto-labellers with SAM/SAM2, with distilled versions (NanoOWL/NanoSAM) available on Orin.

### Cited Findings
**Pepper/chili-specific data**
- "Unlabelled Weed Detection Images for Hot Peppers" (Trinidad and Tobago): Capsicum annuum on several smallholder farms, with varying weed cover and species. Some weeds "look almost identical" to the crop. CC BY; author Jade Chattergoon, v1.0. — [AmeriGEOSS](https://data.amerigeoss.org/dataset/weeddetection_hotpeppers); [CSISS mirror](https://cloud.csiss.gmu.edu/uddi/dataset/weeddetection_hotpeppers)
- "Labelled Weed Detection Images for Hot Peppers" has pixel-level ground truth with three classes (weed, crop, background). Labels were made by trained local annotators in the MATLAB Image Labeller. Files are on Google Drive; whether the link is still live is unconfirmed. — [AmeriGEOSS labelled](https://data.amerigeoss.org/dataset/weeddetection_labelled_hotpeppers); [CSISS labelled mirror](https://cloud.csiss.gmu.edu/uddi/sk/dataset/weeddetection_labelled_hotpeppers)
- Roboflow "Chili_Weeds_Detection" has two classes ("eweeds", "plant") under CC BY 4.0; image count was not verified. — [Roboflow Universe](https://universe.roboflow.com/bharat-kbspc/chili_weeds_detection)
- Pepper-adjacent models that are not weed models:
  - Light-YOLO-Pepper (YOLOv8n-based, 1.82 M parameters; 168.4 FPS on GPU, 28.9 FPS on CPU) detects missing seedlings in trays. — [AGRIS record](https://agris.fao.org/search/en/providers/122413/records/6970addbbd789e57df2cb79f)
  - A YOLOv8 Chaotian pepper study targets fruit. — [PMC11397926](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC11397926/)
- No YOLO weed-detection study set in chili fields was found. — [search result set](https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2022.1053329/epub)

**General crop/weed datasets**
- CropAndWeed (WACV 2023): 74 crop and weed species, ~112k instances in >8k high-resolution images, with bounding boxes, semantic masks and stem positions. Free for academic use (github.com/cropandweed/cropandweed-dataset). The number of classes depends on the dataset variant. — [WACV paper](https://openaccess.thecvf.com/content/WACV2023/papers/Steininger_The_CropAndWeed_Dataset_A_Multi-Modal_Learning_Approach_for_Efficient_Crop_WACV_2023_paper.pdf); [supplemental](https://openaccess.thecvf.com/content/WACV2023/supplemental/Steininger_The_CropAndWeed_Dataset_WACV_2023_supplemental.pdf)
- Weed25: 14,035 images of 25 weed species; YOLOv3, YOLOv5 and Faster R-CNN baselines. — [PMC9748680](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9748680/)
- DeepWeeds: 17,509 labelled images of 8 weed species. ResNet-50 reached 95.7% classification accuracy. Camera ~1 m above ground for ~10 km/h operation, with a 100 ms per image budget. — [arXiv 1810.05726](https://arxiv.org/pdf/1810.05726)
- PhenoBench (T-PAMI 2024): UAV sugar beet with pixel-wise crop/weed instance labels and crop leaf instances. Tasks are semantic/panoptic/leaf-instance/hierarchical segmentation, scored on a hidden test set that includes an unseen field. A third-party repackaging counts ~30,424 plant and ~106,767 leaf instances (secondhand). — [arXiv 2306.04557](https://arxiv.org/html/2306.04557v2); [Voxel51 docs](https://docs.voxel51.com/dataset_zoo/datasets_hf/phenobench_raw.html)
- CWFID: 60 top-down RGB+NIR images from an organic carrot field, with vegetation masks and crop/weed labels. — [ECCV 2014 workshop paper](https://projet.liris.cnrs.fr/imagine/pub/proceedings/ECCV-2014/workshops/w23/paper26.pdf)
- Weed Stem Detection (WSD, AAAI 2025): 7,161 field images and 11,151 weed instances, with crop and weed boxes plus weed stem coordinates. The arXiv version says it would be released on acceptance; current availability is unconfirmed. — [arXiv 2502.06255](https://arxiv.org/pdf/2502.06255)
- Viewpoints of other datasets are all much higher than the rover's 5–25 cm:
  - VegeBot lettuce: RGB camera ~1.4 m above ground, nadir. — [arXiv 2306.13518](https://arxiv.org/pdf/2306.13518)
  - Latvian crop/weed seedling set: 1,118 images of 14 species, vertical camera at fixed height. — [PMC7305380](https://pmc.ncbi.nlm.nih.gov/articles/PMC7305380)
  - MMIDDWF wheat: RGB-D camera 70 cm above canopy. — [Frontiers 2022](https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2022.936748/full)

**How many images a new field needs; domain shift**
- In a cross-crop study (cotton and soybean UAV imagery, 2026), few-shot fine-tuning with as few as 25 labelled target images beat unsupervised domain-adaptive detection. Label budgets of K ∈ {5, 10, 25} were tested, with 1.5k fine-tuning iterations at learning rate 0.005. — [arXiv 2608.21254](https://arxiv.org/pdf/2608.21254)
- A Rumex study (2026) found models "generalize poorly to the target domain, even after fine-tuning on the source data", driven by viewpoint, sensor and acquisition differences. ViTs generalised markedly better than ResNets, mainly because of large-scale self-supervised pretraining. — [arXiv 2604.25316](https://arxiv.org/html/2604.25316)
- A vendor claim (not peer-reviewed): 200–500 well-labelled images often reach 85–93% mAP for a single weed species when fine-tuning a COCO-pretrained model, and 300 precise labels beat 1,000 noisy ones. — [Datature blog](https://datature.io/blog/deploying-vision-models-on-agricultural-robots-edge-ai-for-the-field-2026)

**Synthetic data**
- Stable Diffusion augmentation (Modak & Stein, ECCV-W 2024) improved mAP of compact YOLO (nano-class) weed detectors as the share of synthetic images grew. — [arXiv 2411.18513](https://arxiv.org/html/2411.18513v2)
- ControlNet-conditioned Stable Diffusion (10 weed classes) gave +1.2 to +1.4% mAP@50:95 when mixed with real images. — [SPIE 13035-25](https://spie.org/defense-commercial-sensing/presentation/Weed-Image-Augmentation-by-ControlNet-Added-Stable-Diffusion/13035-25)

**Foundation models for auto-labelling or zero-shot**
- Few-shot Grounding DINO (CVPR-W 2025): on PhenoBench, zero-shot mAP was 12.6 and 24-shot adaptation reached 38.2. Even one image beat zero-shot. The method replaces the BERT text encoder with trainable embeddings. It reports up to ~24% higher mAP than fully fine-tuned YOLO in few-shot settings. — [arXiv 2504.07252](https://arxiv.org/html/2504.07252v1)
- For "multi-class fine-grained distinctions such as crop-weed differentiation, the zero-shot approach proves inadequate". — [arXiv 2504.07252](https://arxiv.org/pdf/2504.07252)
- Wild blueberry auto-annotation (Grounding DINO or YOLO-World + SAM2): Grounding DINO beat YOLO-World but over-detects. Its Swin-T reached mIoU 0.694 ± 0.175 on fescue grass. — [PMC12694221](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC12694221/)
- FlexiGroBots: GroundingDINO + SAM-HQ combined with task-trained models gave F1 0.96 on a rapeseed weed set. The zero-shot part was not isolated. — [AI4Europe catalogue](https://www.ai4europe.eu/research/ai-catalog/flexigrobots-weed-detection)
- Grounded SAM "relies on text prompting, but text prompts are ill-defined in the crop-weed segmentation task". Bonn's label-free alternative (SAM + BioCLIP) was evaluated on PhenoBench, SB20 and CropAndWeed sugar beet/maize subsets. Code: PRBonn/WeedsAreWeird. — [Chong et al. IROS 2025](https://www.ipb.uni-bonn.de/wp-content/papercite-data/pdf/chong2025iros.pdf); [ecosyste.ms](https://awesome.ecosyste.ms/projects/github.com%2Fprbonn%2Fweedsareweird)
- Pl@ntNet's DINOv2 backbone combined with SAM gives zero-shot plant segmentation (2025). — [arXiv 2510.12579](https://arxiv.org/html/2510.12579v1)
- A YOLO-trained model has been used to bootstrap a Grounded-SAM weed annotation pipeline. — [MDPI Agronomy 14(9):2052](https://www.mdpi.com/2073-4395/14/9/2052)

**Distilled foundation models on Jetson**
- NanoOWL (TensorRT OWL-ViT) on AGX Orin: ViT-B/32 at 95 FPS (28 mAP) and ViT-B/16 at 25 FPS (31.7 mAP). The Orin Nano FPS was listed as "TBD". — [NanoOWL repo summary](https://awesome.ecosyste.ms/projects/github.com%2Fnvidia-ai-iot%2Fnanoowl); [Jetson AI Lab NanoOWL tutorial](https://www.jetson-ai-lab.com/tutorials/nanoowl)
- An AGX Orin user measured 40–60 ms per inference instead of the advertised 95 FPS. — [NVIDIA forum](https://forums.developer.nvidia.com/t/nanoowl-inference-takes-more-time-than-nanoowl-official-github-shows/293051)
- NanoSAM (distilled from the MobileSAM encoder): a third-party listing reports 27 ms for the image encoder and 4.2 ms for the full pipeline on Jetson Orin Nano. This is unverified, and a user could not reproduce it on AGX Orin. — [SourcePulse NanoSAM](https://www.sourcepulse.org/projects/1953865); [NVIDIA forum](https://forums.developer.nvidia.com/t/i-am-not-getting-the-performance-i-expected-with-nanosam/307338)

### Inferences
- (Inference) Recommended data plan:
  - Pretrain or initialise on CropAndWeed, Weed25 and the Trinidad hot-pepper set, plus a strong self-supervised backbone (DINOv2-class), given the Rumex ViT result.
  - Capture video from the rover's own camera, at its own height and with its own lighting, in the target pepper field.
  - Auto-label with Grounding DINO/OWL + SAM2 offline. Use a generic "plant" prompt plus row-geometry rules to assign crop vs weed; text prompts cannot separate them.
  - Human-correct ~25–100 frames for a first model, then grow to a few hundred, spread over growth stages and lighting.
  - Synthetic data is a small (+1–1.5 mAP) add-on, not a substitute.
- (Inference) For an inter-row green-on-brown rover, a "plant vs soil" (binary) plus "pepper vs other" model needs far fewer labels than species classification. Pepper is the only class that must be learned reliably.

### Gaps
- No public dataset captured at 5–25 cm height, under a crop canopy, was found for any crop.
- Image count, resolution and capture geometry of the Trinidad hot-pepper set are unknown (the page could not be fetched).
- No DINOv3-based weed results were found, and no YOLO-World/YOLOE weed benchmarks or Orin Nano numbers.
- Weed-AI (the Australian weed image repository) content could not be retrieved in this session.

## 4. Viewpoint and optics at 5–25 cm: FOV, GSD, perspective, depth of field, motion blur, lighting and camera choice

### Takeaway
At 5–25 cm a wide lens is mandatory to see more than the chassis footprint. For example, the Pi Camera Module 3 Wide (102° × 67°) covers ≈ 25 × 13 cm at 10 cm height, with ~0.4 mm/px even after downscaling to 640 px. So resolution is not the limit; motion blur, exposure under the canopy, lens distortion and very small along-track dwell time are. A global-shutter sensor is clearly better: in OWL tests it gave 95.7% vs much lower recall for rolling-shutter Pi cameras at speed. A downward camera under a shrouded chassis with LED flash would address the shadow, dappled-light and exposure problems together (inference).

### Cited Findings
- Raspberry Pi Camera Module 3 Wide (Sony IMX708): 120° diagonal angle of view (102° horizontal × 67° vertical), 2.75 mm focal length, 4608 × 2592 px with 1.4 µm pixels, HDR, phase-detection autofocus, rolling shutter. In production until at least January 2030. — [Raspberry Pi Camera Module 3 product page](https://raspberrypi.com/products/camera-module-3); [PiShop CM3 Wide listing](https://www.pishop.us/product/raspberry-pi-camera-module-3-wide/)
- Raspberry Pi Global Shutter Camera (IMX296): 1.6 MP (1440 × 1080), C/CS mount, so the FOV depends on the chosen lens. No official datasheet was retrieved. — [search summary of distributor listings](https://raspberrypi.com/products/camera-module-3)
- Third-party wide IMX708 modules with M12 lenses (~120° horizontal) exist from Arducam and Waveshare, with differing distortion. — [Arducam](https://www.arducam.com/blog/?p=94815); [Waveshare IMX708-120](https://www.waveshare.com/product/imx708-120-camera-module.htm)
- In OWL's speed study, the global-shutter Arducam AR0234 kept 85.7% recall at 30 km/h. The rolling-shutter Pi HQ (IMX477) and V2 fell to 31.1% and 26.0%. — [arXiv 2305.10311 summary](https://arxiv.deeppaper.ai/papers/2305.10311v1?locale=en)
- Blur rule of thumb: blur becomes a problem when the distance moved during exposure exceeds the ground sampling distance (GSD). — [CSU Gulbali, UAV flight planning](https://www.csu.edu.au/research/gulbali/remote-weed-detection/detecting-weeds/uav-mission-planning/planning-the-flight-path)
- DeepWeeds/AutoWeed placed the camera ~1 m above ground for ~10 km/h. It needed a fast shutter to avoid motion blur and allowed about 100 ms per image. — [arXiv 1810.05726](https://arxiv.org/pdf/1810.05726)
- In a ground-robot study, a forward-tilted camera gains coverage, but recognition falls with distance because of perspective. — [core.ac.uk 153372564](https://core.ac.uk/download/153372564.pdf)
- A paper on blur-based weed recognition defines a maximum camera speed to keep recognition rates high. — [Bohrium listing](https://www.bohrium.com/paper-details/weed-recognition-using-image-blur-information/811624734826430466-1388)
- An HDR camera plus shadow removal improved ExG precision by 20%, a sign that dynamic range and lighting matter. — [Bohrium listing](https://www.bohrium.com/paper-details/improved-vegetation-segmentation-with-ground-shadow-removal-using-an-hdr-camera/811054645446180864-1389)
- OAK-D-Lite pairs a 13 MP IMX214 colour camera with OV7251 global-shutter stereo cameras and an on-board Myriad X. Stereo depth is available, but it is not RGB-only. — [Zbotic review](https://zbotic.in/oak-d-lite-ai-camera-depth-ai-on-edge-device-review/)

### Inferences
These calculations are mine, using the pinhole approximation and the sourced CM3 Wide 102° H × 67° V field of view.

**Footprint at height h:** about 2.47·h across × 1.32·h along track.

| Height h | Footprint (across × along) |
|---|---|
| 5 cm | 12.3 × 6.6 cm |
| 10 cm | 24.7 × 13.2 cm |
| 15 cm | 37.0 × 19.9 cm |
| 25 cm | 61.7 × 33.1 cm |

A 5 cm camera sees less than the 15 × 20 cm chassis. About 10 cm height is the minimum to cover a typical 20–30 cm inter-row strip with one camera.

**GSD:**

| Height h | Full resolution (4608 px) | Downscaled to 640 px | Downscaled to 320 px |
|---|---|---|---|
| 5 cm | — | ≈ 0.19 mm/px | — |
| 10 cm | ≈ 0.05 mm/px | ≈ 0.39 mm/px | ≈ 0.77 mm/px |
| 25 cm | — | ≈ 0.96 mm/px | — |

A 5 mm cotyledon-stage weed still spans ≥ 5 px at 320 px input from 25 cm.

**Motion blur** (blur = v × t_exp, in pixels at inference resolution):

| Speed | Exposure | Blur on ground | Pixels at 640 px input, h = 10 cm |
|---|---|---|---|
| 0.5 m/s | 1 ms | 0.5 mm | ≈ 1.3 px (acceptable) |
| 0.5 m/s | 8 ms (plausible under a canopy without lights) | 4 mm | ≈ 10 px (destructive) |
| 0.1 m/s | 8 ms | 0.8 mm | ≈ 2 px |

Conclusion: either drive at ≤ 0.1–0.2 m/s, or add strobed LED lighting to allow ≤ 1 ms exposures, or both. A global shutter also avoids rolling-shutter skew, whose size depends on sensor readout time (not sourced here).

**Dwell time:** with the long sensor axis across track at h = 10 cm, the along-track footprint is 13.2 cm.

| Speed | Time in view | Frames at 5 FPS (Pi 5 CPU) | Frames at 30 FPS (Hailo/Orin) |
|---|---|---|---|
| 0.5 m/s | 0.26 s | ≈ 1.3 | ≈ 8 |
| 0.1 m/s | 1.3 s | ≈ 6.6 | ≈ 40 |

Multi-frame confirmation and tracking therefore needs either an accelerator or a slow rover. Rotating the camera so the 102° axis runs along track doubles the dwell time but narrows lateral coverage.

**Depth of field and focus:** at 5–25 cm, plant height (weeds 0–5 cm; pepper leaves higher) is a large fraction of the working distance. Fixed-focus lenses need a small aperture or focus set near the soil plane. The CM3's PDAF helps, but adds focus-hunting latency. No measured DoF figures were found.

**Lens distortion:** wide/fisheye lenses need calibration (intrinsics + distortion) before ground-plane projection (section 5).

**Mounting:**
- A downward-looking camera inside a light-tight shroud with LED illumination gives repeatable colour, which makes ExG/HSV thresholds stable. It also removes canopy shadows and allows short exposures. It costs a smaller footprint and the need for a skirt that clears 0–5 cm weeds and drip tape.
- A forward-looking camera sees farther (more latency budget, earlier crop recognition), but suffers perspective loss and variable sunlight.
- A combined design (forward camera for crop row and stem, downward shrouded camera for targeting) mirrors the multi-camera Flourish approach (section 5).

### Gaps
- No study was found that measures weed detection accuracy at 5–25 cm camera height, or under-canopy light levels in pepper.
- No official IMX296 Global Shutter Camera datasheet or CM3 minimum-focus distance was retrieved.
- No comparison was found between LED-shrouded and natural-light performance for colour indices in row crops.

## 5. Localising the weed for the tool: required accuracy, monocular ground-plane projection, latency budget, and camera-to-tool tracking

### Takeaway
Required accuracy depends on the tool:
- Laser or micro-dose needs ~1–3 mm. Reported lab/greenhouse targeting is ±1 mm and ±3.4 mm; a laser robot reports 1.54 mm mean error.
- Stereo close-range systems report ~2–3 mm per axis.
- Mechanical intra-row hoes work at ~6–10 mm. Encoder fusion cut hoe error from 9.88 to 6.06 mm.

A monocular camera at a known height can project detections to the ground plane. At 5–25 cm height, however, small changes in height or tilt cause large relative errors. Closed-loop visual servoing, or stop-and-align as in LiteWeed, is the low-cost way to reach mm accuracy without stereo. Without tracking, single-pass laser elimination was 47%; with DeepSORT tracking it rose to 72%.

### Cited Findings
- LZH laser weeding achieved ±1 mm targeting under lab conditions and ±3.4 mm in the greenhouse (laser on a rail carriage). The meristem position was inferred from leaf positions using a stereo camera. — [Photonics.com](https://www.photonics.com/a50365/Controlling_Weeds_with_Lasers); [Laser Focus World](https://laserfocusworld.com/industrial-laser-solutions/article/14218955/laser-weed-control)
- A laser weeding robot reported mean positional errors of 1.22 mm (x), 0.67 mm (y) and 1.54 mm resultant, verified by a second camera. The extract attributes this to a laser-robot mechanism paper; attribution is uncertain. — [arXiv 2512.10319](https://arxiv.org/pdf/2512.10319)
- A close-range stereo weed-location system reported mean absolute errors of 2.2 / 2.6 / 3.2 mm (X/Y/Z), about 4.7 mm 3D. It needs stereo. The extract's attribution to MCS-YOLOv8 + adaptive filtering is probable but not certain. — [MDPI Agriculture 16(13):1437](https://www.mdpi.com/2077-0472/16/13/1437)
- Intra-row hoe positioning: fusing rotary-encoder odometry with vision cut mean error from 9.88 mm (vision only) to 6.06 mm, compensating for visual delay. — [Bohrium/arXiv listing, "Optimization Method of Intra-Row Weeding Hoe Positioning Data Based on Odometry Information Fusion"](https://bohrium.dp.tech/paper/arxiv/600d1eeeb42d2a8a0a834feb)
- Mwitta et al. 2024 (cotton): an Ackermann robot with a 2-DOF Cartesian manipulator, stereo vision and a diode laser on pan-tilt.
  - Without tracking: 47% weed elimination in a single pass, 9.5 s cycle per weed.
  - With DeepSORT tracking: 72.35% at 8 s cycle.
  — [Frontiers in Agronomy 10.3389/fagro.2024.1388452](https://www.frontiersin.org/articles/10.3389/fagro.2024.1388452/full)
- Flourish (Bonn) uses a non-overlapping multi-camera system: weeds are classified in the first camera and tracked in later cameras. This combines detection, 3D tracking and predictive actuation to decouple detection latency from actuation. — [Wu et al. 2020 JFR](https://www.ipb.uni-bonn.de/pdfs/wu2020jfr.pdf)
- Worked latency example from a sprayer patent: with a ~1 m field of view at 5.5 m/s, total latency must be < 180 ms. — [US 11,071,991](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11071991)
- A UAV study estimated weed positions from a monocular image stream by fusing detections in an unscented Kalman filter (no mm figures retrieved). — [PMC7013443](https://pmc.ncbi.nlm.nih.gov/articles/PMC7013443)
- An industry blog claims laser systems must fire within 0.5 cm of a crop plant, and that a targeting error "destroys the crop", which argues for pixel-accurate segmentation over boxes. This is a vendor source. — [iMerit blog](https://imerit.ai/resources/blog/beyond-bounding-boxes-why-pixel-perfect-semantic-segmentation-is-critical-for-laser-based-weeding-robots/)
- Joint stem detection outputs the weed stem location for mechanical treatment and the weed area for spraying. — [Lottes 2019 JFR](https://www.ipb.uni-bonn.de/pdfs/lottes2019jfr.pdf)
- LiteWeed (Pi 5) uses a stop-and-align approach, which limits coverage to ~0.01–0.03 ha/day. — [Global Agriculture](https://www.global-agriculture.com/ag-tech-research-news/low-cost-laser-weeding-robot-built-for-smallholder-farms-passes-field-tests-in-canada-and-tanzania/)

### Inferences
These calculations are mine.
- **Monocular ground-plane projection** with a nadir camera at height h: a feature at height z above the soil, at true ground offset r from nadir, projects with lateral error ≈ r·z/(h−z).
  - Example: h = 10 cm, a weed leaf tip at z = 2 cm, r = 8 cm off-centre → ≈ 2 cm error.
  - A camera-height error Δh (soil clods, ridges, rover pitch) causes a proportional scale error Δh/h. At h = 10 cm, Δh = 1 cm, r = 10 cm → 1 cm error. At h = 25 cm, the same Δh gives 0.4 cm.
  - Low cameras are therefore more sensitive to terrain. For mm-level work, target the stem or meristem (z ≈ 0) rather than the leaf centroid, keep the target near nadir or tool-aligned, and close the loop (visual servoing with a camera that sees the tool or laser spot). Otherwise stereo or active depth is required, which is outside this RGB-only scope.
- **Latency budget:** allowed latency = (camera-to-tool distance) / speed.
  - Tool 10 cm behind the view centre: 500 ms at 0.2 m/s, 200 ms at 0.5 m/s.
  - Timing jitter σ_t produces position error v·σ_t: 50 ms jitter at 0.2 m/s = 10 mm, which is unacceptable for laser and acceptable for a hoe.
  - Hardware-timestamped frames plus wheel-encoder (or optical-flow VO) dead-reckoning from capture time to actuation are needed, consistent with the 9.88 → 6.06 mm encoder-fusion result.
  - A downward camera can do visual odometry on soil texture cheaply (inference, not benchmarked here).
- **Tool tiers:**
  - Hoeing or stamping in the inter-row (cm tolerance) is compatible with monocular projection, encoders and Pi-class compute.
  - Spot micro-spray (~1 cm) is feasible with good tracking.
  - Laser/meristem (≤ 2–3 mm) realistically needs stop-and-align or closed-loop servoing at low speed, as LiteWeed and Mwitta show.

### Gaps
- No monocular RGB weeding system at 5–25 cm height reporting mm-level projection error was found.
- No pepper-specific crop-safety distance requirements were found.
- The source attribution for the 1.54 mm laser error and the stereo 4.7 mm figure is from search extracts and should be verified.
- A search summary mentioned an RGB-D Jetson Orin onion system with 0.83 mm mean radial error, but its source page could not be pinned down, so it is excluded.

## 6. Species and growth-stage discrimination, and meristem targeting on edge devices

### Takeaway
Multi-species weed classification is feasible offline: DeepWeeds reached 95.7% across 8 species, and CropAndWeed covers 74 species. On edge devices, crop/weed + stem keypoint models run at ~12 FPS on Xavier-NX-class hardware. Meristem/stem localisation is a distinct task from detection: "higher mAP does not guarantee stem localisation". For laser or stamp tools, growth stage matters because the kill dose rises steeply after the first true leaves. For a small pepper rover, binary crop-vs-weed plus stem keypoints matters more than species ID.

### Cited Findings
- WSD (AAAI 2025): joint crop/weed detection with a weed-stem regression head (YOLOv7). It improved weeding accuracy by 6.7% (arXiv; 5.05% in the AAAI abstract) and cut energy cost by 32.3% vs prior weed recognition systems. — [arXiv 2502.06255](https://arxiv.org/pdf/2502.06255); [mlanthology AAAI 2025](https://mlanthology.org/aaai/2025/liu2025aaai-efficient-a)
- Single-stage joint stem detection reached OKS 56.3 at 12.2 FPS on Jetson Xavier NX (grassland). — [MDPI Agronomy 13(9):2365](https://www.mdpi.com/2073-4395/13/9/2365/html)
- Lottes et al.: the crop/weed stem FCN exploits crop-row arrangement and image sequences. — [arXiv 1806.03413](https://arxiv.org/pdf/1806.03413)
- CropAndWeed provides stem positions for 74 species, so stem-keypoint pretraining data exists. — [WACV 2023](https://openaccess.thecvf.com/content/WACV2023/html/Steininger_The_CropAndWeed_Dataset_A_Multi-Modal_Learning_Approach_for_Efficient_Crop_WACV_2023_paper.html)
- Laser dose depends on growth stage:
  - A minimum of ~35 J killed seedlings in lab work, adaptable to species and stage. — [Photonics.com](https://www.photonics.com/a50365/Controlling_Weeds_with_Lasers)
  - A diode-laser study found growth-stage-1 seedlings controlled at 0.2–0.5 J/mm², with energy needs rising substantially as plants grow. — [Photonics.com a50644](https://photonics.com/Articles/Controlling_weeds_with_lasers/a50644)
  - Earlier laser attempts had limited success, attributed to targeting inaccuracy and advanced growth stages. — [Laser Focus World](https://laserfocusworld.com/industrial-laser-solutions/article/14218955/laser-weed-control)
- Hierarchical leaf → plant panoptic labels (PhenoBench) support growth-stage proxies such as leaf count. — [arXiv 2306.04557](https://arxiv.org/html/2306.04557v2)
- DeepWeeds: 8-species classification at 95.7%, with 53.4 ms per image on Jetson TX2 (TensorRT). — [arXiv 1810.05726](https://arxiv.org/pdf/1810.05726)

### Inferences
- (Inference) For the rover, the most valuable on-device outputs are:
  1. pepper vs not-pepper masks,
  2. weed stem/meristem keypoints,
  3. a weed size or leaf-count proxy for growth stage, which sets tool dwell or energy and flags weeds too large for the tool.
  
  Species ID matters mainly for perennials such as Cyperus, which regrows from tubers and needs a different action. Species ID can be offloaded to post-hoc mapping.
- (Inference) At 5–25 cm height and 0.05–1 mm/px, the meristem of cotyledon-to-4-leaf weeds is resolvable. A low camera actually suits keypoint/meristem targeting better than the 1 m+ viewpoints of most datasets. The new viewpoint, however, means stem/keypoint labels must be created in-house.

### Gaps
- No edge-device (Pi/Hailo/Orin Nano) benchmark of stem/meristem keypoint models was found.
- No pepper-specific growth-stage classifier was found.
- The laser dose figures come from older (≈ 2012) press material and diode studies with unspecified spot sizes.

## 7. Typical pepper weeds and look-alike perception problems

### Takeaway
The weeds that matter most in pepper are Amaranthus spp., Portulaca oleracea, Cyperus rotundus/esculentus, Solanum nigrum, Datura stramonium and grasses (Echinochloa/jungle rice, Cenchrus). Control is critical in roughly the first 24–45 days after transplanting. Solanum nigrum (same family, a "hard to identify" species complex) is the key look-alike, and the Trinidad hot-pepper dataset authors note that some weeds look "almost identical" to the crop. Cyperus is a perception and action problem because it pierces plastic mulch and is grass-like.

### Cited Findings
- European review (EWRS, Pannacci et al.):
  - The critical period for weed control in transplanted pepper is about 25–45 days after planting.
  - The most important weeds are Cyperus rotundus, Datura stramonium, Xanthium strumarium, Portulaca oleracea, Solanum nigrum and Amaranthus spp.
  - Hoeing often compensates for poor herbicide efficacy.
  - C. rotundus pierces plastic films; paper mulch in Spain controls all weeds including C. rotundus.
  — [EWRS weeds and weed management in peppers](https://www.ewrs.org/en/documents/WGs-Weed-Management-Systems-in-Vegetables-Pannacci-et-al-weeds-and-weed-management-in-peppers.pdf)
- Puerto Rico sweet pepper: critical period of weed competition was 24–36 days. Economic thresholds were 2–5 plants/m² for pigweed, 5–10/m² for jungle rice (Echinochloa colona) and 20/m² for horse purslane (Trianthema). — [AgEcon Search 261593](https://ideas.repec.org/p/ags/cfcs84/261593.html)
- Mexican jalapeño: the most difficult weeds are Amaranthus spp., Cenchrus echinatus, Convolvulus arvensis, Cuscuta indecora/umbellata and Cyperus esculentus. — [Revista Fitotecnia Mexicana 45(3)](https://revistafitotecniamexicana.org/documentos/45-3/13a.pdf)
- Egypt: black polyethylene mulching or a hoeing pass after pre-emergence herbicide was recommended to prevent early competition. — [EKB journal article](https://journals.ekb.eg/article_79106_b5a92890d47a8d529fc4a7f2176f56fa.pdf)
- Black nightshade (Solanum nigrum) is a species complex that is hard to identify, and hosts pests and diseases of tomato, potato and pepper. — [UC IPM weed gallery](https://ipm.ucanr.edu/weeds-identification-gallery/black-nightshade/)
- In the hot-pepper dataset, some weeds "look almost identical" to the crop. — [AmeriGEOSS hot pepper dataset](https://data.amerigeoss.org/dataset/weeddetection_hotpeppers)
- No deep-learning paper on S. nigrum vs pepper/tomato was found. The closest is TrackSolanum, a real-time detector and tracker for Solanum rostratum in drone and ground-vehicle video. The extract's attribution to Frontiers Plant Sci 2024 (fpls.2024.1493322) is uncertain. — [Frontiers fpls.2024.1493322](https://frontiersin.org/articles/10.3389/fpls.2024.1493322/full)

### Inferences
- (Inference) Look-alike risk is highest in the row: Solanum nigrum, and to a lesser degree Datura (also Solanaceae), emerging next to the pepper stem. In the inter-row everything green is a weed anyway, so species does not matter there. Near the row, priors help: transplant position on the row line, much larger size and a woody single stem early in the season, and plant spacing. Learned features are needed only for in-row seedlings, and a "do not treat" margin around the crop stem is the safe default.
- (Inference) Visual traits a model can exploit:
  - Portulaca: prostrate, succulent and often reddish-stemmed, so it is distinctive in close-up RGB.
  - Cyperus and grasses: narrow linear leaves, a thin stem at ground level and little ground cover. ExG masks may fragment them, so they need high resolution, which the low camera provides.
  - Amaranthus: broad ovate leaves and fast growth, making it the most urgent for size-limited tools.
  
  Under a canopy all of these are lower-contrast. These traits are general botany and were not verified against sources in this session.
- (Inference) Cyperus regrows from tubers, so a single shoot hit (laser or stamp) is unlikely to give lasting control. A rover would revisit and repeatedly defoliate it rather than "kill" it in one pass.

### Gaps
- No pepper-field weed surveys were found for Chenopodium, Galinsoga or Echinochloa crus-galli specifically. These were requested but not confirmed as top pepper weeds in the sources retrieved.
- No study was found that quantifies S. nigrum vs Capsicum confusion rates for any detector.

## 8. Published end-to-end low-cost RGB-only weeding systems on Raspberry Pi or Jetson

### Takeaway
The only mature, open, RGB-only Raspberry Pi system is OpenWeedLocator: green-on-brown, Pi 4/5, field recall 52–96% depending on site, camera and speed, with high precision. Low-cost laser prototypes (LiteWeed: Pi 5, YOLOX-nano, ~CAD 500) work but are slow (stop-and-align, ~0.01–0.03 ha/day) and report weak detection mAP (0.32). Jetson-based research robots report 47–72% laser elimination (with stereo) and 82–91% spraying or weeding rates, but on more powerful hardware or with larger models. No published system matches the target concept: a 15 × 20 cm under-canopy RGB-only rover in pepper.

### Cited Findings
- **OpenWeedLocator:**
  - Hardware: Pi 4 (8 GB), HQ Camera, ExG+HSV at 416 × 320, triggering solenoids. — [Sci Rep 2022](https://springerlink.bibliotecabuap.elogim.com/content/pdf/10.1038/s41598-021-03858-9.pdf)
  - Fallow fields: 79% precision / 52% recall overall, up to 92% / 74% at individual sites. — [CAWS 2022](https://caws.org.nz/wp-content/uploads/2023/02/0218.pdf)
  - Recall up to 95.7% at 5 km/h with the AR0234 global-shutter camera. — [arXiv 2305.10311](https://arxiv.org/pdf/2305.10311)
  - Pi 5 support since 2024; green-on-green still in development. — [OWL GitHub](https://github.com/geezacoleman/OpenWeedLocator)
- **LiteWeed (low-cost laser):**
  - Hardware: 4 W blue diode laser, YOLOX-nano on Raspberry Pi 5, onboard AI, ~CAD 500.
  - Tested on pigweed, purslane and nutsedge in bare-ground plots in Vancouver and Arusha (Tanzania).
  - Detection mAP 0.32 at ~118 ms per detection; stop-and-align coverage ~0.01–0.03 ha/day.
  - No per-target hit rate was retrieved.
  — [Global Agriculture news](https://www.global-agriculture.com/ag-tech-research-news/low-cost-laser-weeding-robot-built-for-smallholder-farms-passes-field-tests-in-canada-and-tanzania/); [srsweb preprint](https://radiopharmaconnect.srsweb.org/doi/full/10.22541/au.173297211.11959193/v1)
- **Mwitta et al. 2024 (cotton, diode laser, stereo vision):** 47% single-pass elimination without tracking (9.5 s per weed); 72.35% with DeepSORT tracking (8 s per weed). — [Frontiers in Agronomy 2024](https://www.frontiersin.org/articles/10.3389/fagro.2024.1388452/full)
- **Older laser precedents:**
  - A 2017 indoor prototype with colour segmentation and two laser pointers reported a 97% hit rate (indoors, not comparable to field). — [MDPI Agronomy 10:1616 (citing study)](https://mdpi-res.com/d_attachment/agronomy/agronomy-10-01616/article_deploy/agronomy-10-01616-v2.pdf)
  - A YOLOX-based blue-laser weeding robot in corn (Kunming Univ. of Sci. & Tech., 2022). — [Frontiers Plant Sci 10.3389/fpls.2022.1017803](https://www.frontiersin.org/articles/10.3389/fpls.2022.1017803/full)
- **Jetson-class systems:**
  - SAMBot: ~US$500, Jetson Nano, weed classification and spraying in flax, with autonomous recharge and navigation in narrow rows. — [arXiv 2112.02162](https://ar5iv.labs.arxiv.org/html/2112.02162)
  - UBI (Remote Sensing 2022): Jetson Nano segmentation driving a spray mechanism, 25 FPS at 513 × 513 with TensorRT. — [MDPI Remote Sensing 14(17):4217](https://mdpi.com/2072-4292/14/17/4217)
  - Farm-ng Amiga: YOLO11n on Jetson Orin Nano, canopy-aware spraying, field trials in soybean and corn. — [arXiv 2507.05432](https://arxiv.org/html/2507.05432v1)
  - YOLOv4 on Jetson AGX Orin with TeeJet solenoids: 90.6% effective spray in the field (93.33% indoors). — [JAFR 2024 (DOAJ)](https://doaj.org/article/4ef11f36bbdd4cd9b4129b456c7e0f22)
- **Mechanical weeding robots (cited in a 2025 Frontiers paper; the attribution of the extract to that paper is probable):**
  - Lettuce intra-row: crop localisation accuracy 89.27% at 3.28 km/h, intra-row weeding rate 83.73%.
  - Paddy YOLOv5x robot (Ju et al.): 90.05% accuracy, 82.4% weeding rate, 2.8% seedling injury.
  — [Frontiers Plant Sci 10.3389/fpls.2025.1540722](https://www.frontiersin.org/articles/10.3389/fpls.2025.1540722/full)
- **Vegetable seedling field:** an attention-improved YOLOv5 reached 95.7% average detection accuracy (detection only, no actuation). — [HIT scholar record](https://scholar.hit.edu.cn/en/publications/research-on-weed-detection-in-vegetable-seedling-fields-based-on-/)
- **Tertill:** a commercial very small solar garden robot, non-camera (capacitive height rule + string trimmer). It is a design precedent for a "small plants = weeds" rule and for physical crop guards. — [Tertill support](https://support.tertill.com/article/20-sensing-plants-and-weeds)
- A benchmarking paper notes that "very few studies take a detection model all the way through to real robotic actuation and deployment on an edge device". — [Atlantis Press (Shaikh & Jakhete)](https://www.atlantis-press.com/article/126028647.pdf)

### Inferences
- (Inference) The most credible low-cost architecture for this rover combines published components:
  - OWL-style ExG/HSV green mask (Pi-class, real-time) under its own LED lighting.
  - A small learned "pepper / pepper-stem" detector (nano YOLO or a lightweight segmentation network) on a Pi 5 + Hailo-8L/8, or on an Orin Nano Super, to veto the crop with a keep-out radius.
  - Encoder- and timestamp-based tracking from camera to tool.
  - A cm-tolerant tool (micro-hoe, stamp or string trimmer) for the inter-row, and stop-and-align only if a mm-tolerant tool (laser) is used near the stem.
  
  Published field rates suggest that, even with good detection, single-pass actuation success is typically 50–90%, so repeated passes should be planned.
- (Inference) Detection accuracy is not the main limiter in low-cost systems. Hit rate is driven by tracking/latency (Mwitta: 47 → 72% with tracking), speed and blur (OWL recall falls with speed), and lighting (JAFR: 93 → 91% field drop attributed to light, shadows and wind).

### Gaps
- No end-to-end hit rates were found for any Raspberry Pi green-on-green system in a vegetable crop.
- No published under-canopy (5–25 cm) RGB weeding rover in pepper or chili was found.
- LiteWeed's per-weed kill rate and publication venue were not retrievable.
- Hardware details (camera, compute) of the lettuce and paddy robots were not confirmed.
