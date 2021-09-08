extends Spatial
func _ready():
	$Sprite3D.set_texture($Viewport.get_texture())
	
func set_text(text):
	$Viewport/Label.text = text
	$Viewport.size = $Viewport/Label.rect_size
