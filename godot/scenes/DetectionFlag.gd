extends Spatial
func _ready():
	$Sprite3D.set_texture($Sprite3D/Viewport.get_texture())
	
func set_text(text):
	$Sprite3D/Viewport/Label.text = text
	$Sprite3D/Viewport.size = $Sprite3D/Viewport/Label.rect_size
