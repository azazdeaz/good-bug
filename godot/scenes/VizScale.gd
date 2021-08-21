extends Node

onready var state = $"/root/State"

func _ready():
	state.connect("state_update", self, "on_state_update")

func on_state_update(state_data):
	$SpinBox.value = state_data.ui_state.viz_scale
	
func _on_SpinBox_value_changed(value):
	get_node("/root/Game").set_viz_scale(value)

