extends HBoxContainer

func _on_Game_system_status(system_status):
	print(system_status.used_memory / system_status.total_memory * 100)
	var text = "CPU%3d%% (%d°C) | Mem%3d%% | Battery%3d%%" % [
		system_status.cpu_usage, 
		system_status.cpu_temperature,
		float(system_status.used_memory) / system_status.total_memory * 100,
		system_status.battery * 100,
	]
	$Label.text = text
