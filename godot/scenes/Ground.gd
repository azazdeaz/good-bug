extends Spatial

signal map_click(position)


func _on_StaticBody_input_event(camera, event, click_position, click_normal, shape_idx):
	print("ground event", event)
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT and event.doubleclick:
			print("Mouse Click/Unclick at: ", event.position, " shape:", shape_idx)
			click_position = Vector3(click_position)
			print(click_position)
			emit_signal("map_click", click_position)
			return
	var cam = get_node("../CamTarget/TrackballCamera")
	cam.input(event)
