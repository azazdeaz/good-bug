extends HBoxContainer


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

var idx = 0;

func save_image():
	var filename = "image_%s.jpg" %  idx
	idx += 1;
	get_node("/root/Game").save_image($SaveImageFolder.text, filename)
	
func _on_SaveImageBtn_pressed():
	save_image()

func _unhandled_input(event):
	if event is InputEventJoypadButton:
		if event.button_index == 0 and event.is_pressed() and !event.is_echo():
			save_image()
			
