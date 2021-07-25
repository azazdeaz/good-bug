extends TextureRect


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var detections = []
var frame_scale = 1

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func update_detections(_detections):
	detections = _detections
	print("DETECTIONS:", detections)
	update_frame_scale()
	update()


func draw_circle_arc(center, radius, angle_from, angle_to, color):
	var nb_points = 32
	var points_arc = PoolVector2Array()

	for i in range(nb_points + 1):
		var angle_point = deg2rad(angle_from + i * (angle_to-angle_from) / nb_points - 90)
		points_arc.push_back(center + Vector2(cos(angle_point), sin(angle_point)) * radius)

	for index_point in range(nb_points):
		draw_line(points_arc[index_point], points_arc[index_point + 1], color)


func _draw():
	for detection in detections:
		var a = Vector2(detection.xmin * frame_scale, detection.ymin * frame_scale)
		var b = Vector2(detection.xmax * frame_scale, detection.ymin * frame_scale)
		var c = Vector2(detection.xmax * frame_scale, detection.ymax * frame_scale)
		var d = Vector2(detection.xmin * frame_scale, detection.ymax * frame_scale)
		var color = Color(1.0, 0.0, 0.0)
		draw_line(a, b, color)
		draw_line(b, c, color)
		draw_line(c, d, color)
		draw_line(d, a, color)
#	var center = Vector2(200, 200)
#	var radius = 80
#	var angle_from = 75
#	var angle_to = 195
#	var color = Color(1.0, 0.0, 0.0)
#	draw_circle_arc(center, radius, angle_from, angle_to, color)



func _on_CameraImage_item_rect_changed():
	update_frame_scale()
	
	
func update_frame_scale():
	var _frame_scale = get_size().x / get_texture().get_size().x
	if _frame_scale != frame_scale:
		frame_scale = _frame_scale
		update()
