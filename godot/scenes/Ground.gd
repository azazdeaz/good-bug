extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_StaticBody_input_event(camera, event, click_position, click_normal, shape_idx):
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT and event.doubleclick:
			print("Mouse Click/Unclick at: ", event.position, " shape:", shape_idx)
			click_position = Vector3(click_position)
			print(click_position)
			
			get_node("/root/Game").select_target(
				click_position.x,
				click_position.y,
				click_position.z
			)
			
#			var mark = CSGCylinder.new()
#			mark.radius = 0.3
#			mark.height = 0.1
#			mark.sides = 20
#			mark.translation = click_position
#			get_node("../..").add_child(mark)
