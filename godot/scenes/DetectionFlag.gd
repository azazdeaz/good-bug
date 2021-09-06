extends Spatial
func _ready():
	print("DM is ready")
	$Sprite3D.texture.viewport_path = $Sprite3D/Viewport.get_path()
	
func set_text(text):
	print("setting text", text);
	$Sprite3D/Viewport/Label.text = text
	
	print("set text", $Sprite3D/Viewport/Label.text);
