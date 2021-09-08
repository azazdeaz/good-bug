extends Spatial

onready var state = $"/root/State"
onready var NavMode = state.NavMode
var markers = []

func set_markers(poses):
	if !poses:
		return
		
	for i in range(poses.size()):
		if !range(markers.size()).has(i):
			var marker = $Waypoint.duplicate()
			var static_body: StaticBody = marker.get_node("StaticBody")
			static_body.connect("input_event", self, "on_StaticBody_input_event", [i])
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

func _ready():
	$Waypoint.visible = false
	state.connect("state_update", self, "on_state_update")

func on_state_update(state_data):
	var map = state.get_current_map()
	
	visible = state_data.nav_mode == NavMode.WAYPOINTS
	
	if map:
		set_markers(map.waypoints)
	else:
		set_markers([])
		
var grabbed_idx = -1
		
func on_StaticBody_input_event(camera, event, click_position, click_normal, shape_idx, wp_idx):
#	get_node("/root").set_input_as_handled()
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT:
			if event.is_pressed():
				grabbed_idx = wp_idx
	elif event is InputEventMouseMotion:
		if grabbed_idx >= 0:
			var wp = {
				"x": click_position.x,
				"z": click_position.z,
			}
			state.update_waypoint(grabbed_idx, wp)

func _input(event):
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT and !event.is_pressed():
			grabbed_idx = -1
