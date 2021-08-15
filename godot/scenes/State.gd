extends Node

signal state_update(state)
signal current_map_update(map)
signal maps_update(maps)
signal waypoints_update(waypoints)
signal nav_mode_update(nav_mode)
signal goal_update(goal)	

enum NavMode { TELEOP, GOAL, WAYPOINTS }

var state = {
	"nav_mode": NavMode.TELEOP,
	"current_map_name": "no_map", 
	"goal": null,
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
	yield(get_tree().create_timer(0.1),"timeout")
	get_node("/root/Game").connect("robot_params", self, "_on_Game_robot_params")
	emit_all()
	
func get_current_map():
	for map in state.maps:
		if state.current_map_name == map.name:
			return map
	
func emit_all():
	emit_signal("state_update", state)
	emit_signal("maps_update", state.maps)
	var map = get_current_map()
	emit_signal("current_map_update", map)
	emit_signal("waypoints_update", null if !map else map.waypoints)
	emit_signal("nav_mode_update", state.nav_mode)
	emit_signal("goal_update", state.goal)

func add_waypoint(waypoint):
	var map = get_current_map()
	if map:
		map.waypoints.push_back(waypoint)
		get_node("/root/Game").set_waypoints(map.waypoints)
		emit_all()

func set_goal(goal):
	get_node("/root/Game").select_target(
		goal.x,
		goal.z
	)
	state.goal = goal
	emit_all()
		
func set_navigation_mode(nav_mode):
	state.nav_mode = nav_mode
	emit_all()
	
func set_current_map_name(name: String):
	state.current_map_name = name
	emit_all()

func _on_Game_robot_params(robot_params):
	state.current_map_name = robot_params.current_map_name
	state.maps = robot_params.maps
	emit_all()
	
