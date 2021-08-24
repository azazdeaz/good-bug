extends HBoxContainer

func _on_Game_system_status(system_status):
	$Label.text = JSON.print(system_status)
