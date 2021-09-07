extends Spatial
func _ready():
	$Sprite3D.texture.viewport_path = $Sprite3D/Viewport.get_path()
	
func set_text(text):
	$Sprite3D/Viewport/Label.text = text
