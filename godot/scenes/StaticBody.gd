extends StaticBody

signal select_nav_goal(x, y, z)

func _ready():
	pass # Replace with function body.

func _on_StaticBody_input_event(camera, event, click_position, click_normal, shape_idx):
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT and event.doubleclick:
			print("Mouse Click/Unclick at: ", event.position, " shape:", shape_idx)
			click_position = Vector3(click_position)
			print(click_position)
			
			emit_signal(
				"select_nav_goal",
				click_position.x,
				click_position.y,
				click_position.z
			)
			
			var mark = CSGCylinder.new()
			mark.radius = 0.3
			mark.height = 0.1
			mark.sides = 20
			mark.translation = click_position
			get_node("../..").add_child(mark)
