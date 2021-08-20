extends ViewportContainer

func _input(event):
	# HACK skip event from outside the viewport
	if "position" in event:
		var is_outside_this_viewport = (event.position.x<rect_position.x 
		or event.position.y<rect_position.y 
		or event.position.x>(rect_position.x+rect_size.x) 
		or event.position.y>(rect_position.y+rect_size.y))
		
		if is_outside_this_viewport:
			return
		
	# HACK pass events to the sub-viewport https://godotengine.org/qa/49795/input-on-area2d-node-child-of-viewport-does-not-work
	event = event.xformed_by(Transform2D())
	if "position" in event:
		event.position = $Viewport.get_mouse_position()
	$Viewport.unhandled_input(event)
	
	
