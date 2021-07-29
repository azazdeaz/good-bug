extends ViewportContainer
# HACK pass events to the sub-viewport https://godotengine.org/qa/49795/input-on-area2d-node-child-of-viewport-does-not-work
func _input(event):
	event = event.xformed_by(Transform2D())
	if "position" in event:
		event.position = $Viewport.get_mouse_position()
	$Viewport.unhandled_input(event)
