extends Tree

var close_icon = load("res://assets/icons/close.svg")
# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	var item1 = create_item()
	item1.set_text(0, "item1")
	item1.add_button(0, close_icon)


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Tree_button_pressed(item, column, id):
	pass # Replace with function body.
