extends Spatial

var markers = []

func set_markers(poses):
	for i in range(poses.size()):
		if !range(markers.size()).has(i):
			print("add marker")
			var marker = $Waypoint.duplicate()
			add_child(marker);
			markers.push_back(marker)
	
	for i in range(markers.size()):
		var marker = markers[i];
		if !range(poses.size()).has(i):
			marker.visible = false
		else:
			marker.visible = true
			marker.translation.x = poses[i].x
			marker.translation.z = poses[i].z

