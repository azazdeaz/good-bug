extends Spatial

onready var state = $"/root/State"
var markers = []

func _ready():
	state.connect("localized_detections_update", self, "on_localized_detections_update")
	$DetectionMarker.visible = false
#	on_localized_detections_update([
#		{'class': 1, 'center': Vector3(1,0,1), "id": 1},
#		{'class': 2, 'center': Vector3(2,0,2), "id": 2},
#	])

func on_localized_detections_update(detections):
	if !detections:
		return
		
	for i in range(detections.size()):
		if !range(markers.size()).has(i):
			var marker = $DetectionMarker.duplicate()
			add_child(marker);
			markers.push_back(marker)
	
	for i in range(markers.size()):
		var marker = markers[i];
		if !range(detections.size()).has(i):
			marker.visible = false
		else:
			marker.visible = true
			marker.set_text("Weed %d #%d" % [detections[i].class, detections[i].id]);
			marker.translation.x = detections[i].center.x
			marker.translation.z = detections[i].center.z

