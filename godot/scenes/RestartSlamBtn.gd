extends Button

func _on_RestartSlamBtn_pressed():
	var game = get_node("/root/Game")
	game.restart_slam()
