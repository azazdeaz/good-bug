tool
extends Viewport
func _ready():
	$Label.text = "ready"
func _process(delta):
	size = $Label.rect_size
