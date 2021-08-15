extends CheckButton

func _on_EnableAutoNavBtn_toggled(enable):
	get_node("/root/Game").enable_auto_nav(enable)
