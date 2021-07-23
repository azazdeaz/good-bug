extends CheckButton

func _on_EnableRaw_toggled(enable):
	var game = get_node("/root/Game")
	game.enable_raw_preview(enable)
