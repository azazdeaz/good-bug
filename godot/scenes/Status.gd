extends Node

signal current_map_update(map)
signal maps_update(maps)
signal waypoints_update(waypoints)

var state = {
	"current_map_name": "no_map", 
	"maps":[{
		"db_path": "/good_bug/map.db", 
		"name": "no_map", 
		"waypoints":[]
	}, {
		"db_path": "/good_bug/saved_images.db,", 
		"name":"saved_images", 
		"waypoints":[]
	}]
}


# Called when the node enters the scene tree for the first time.
func _ready():
	emit_all()
	
func get_current_map():
	for map in state.maps:
		if state.current_map_name == map.name:
			return map
	
func emit_all():
	emit_signal("maps_update", state.maps)
	var map = get_current_map()
	emit_signal("current_map_update", map)
	emit_signal("waypoints_update", null if !map else map.waypoints)

func add_waypoint(waypoint: Vector3):
	var map = get_current_map()
	if map:
		map.waypoints.push(waypoint)
		emit_all()

func _on_Game_robot_params(robot_params):
	state.current_map_name = robot_params.current_map_name
	state.maps = robot_params.maps
	
