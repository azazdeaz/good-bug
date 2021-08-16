extends VBoxContainer

var close_icon = load("res://assets/icons/Remove.svg")

onready var state = $"/root/State"

var items = []

func _ready():
	state.connect("waypoints_update", self, "on_waypoints_update")

func on_waypoints_update(waypoints):
	$WaypointsList.clear()
	items.clear()
	
	var item = $WaypointsList.create_item()
	item.set_text(0, "Waypoints")
	
	if waypoints:
		for i in range(waypoints.size()):
			item = $WaypointsList.create_item()
			item.set_text(0, "Waypoint %s" % i)
			item.add_button(0, close_icon)
			items.push_back(item)

func _on_WaypointsList_button_pressed(item, column, id):
	var idx = items.find(item)
	state.remove_waypoint(idx)
