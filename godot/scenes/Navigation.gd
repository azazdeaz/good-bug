extends Node


onready var state = $"/root/State"
onready var NavMode = state.NavMode
onready var option_id_to_mode = [NavMode.TELEOP, NavMode.GOAL, NavMode.WAYPOINTS]
onready var current_navigation_mode = NavMode.TELEOP

export(NodePath) var _teleop_container:NodePath
onready var teleop_container:Node = get_node(_teleop_container)
export(NodePath) var _goal_container:NodePath
onready var goal_container:Node = get_node(_goal_container)
export(NodePath) var _waypoints_container:NodePath
onready var waypoints_container:Node = get_node(_waypoints_container)
export(NodePath) var _goal_marker:NodePath
onready var goal_marker:Spatial = get_node(_goal_marker)
export(NodePath) var _waypoint_markers:NodePath
onready var waypoint_markers:Spatial = get_node(_waypoint_markers)




func on_nav_mode_update(mode):
	current_navigation_mode = mode
	teleop_container.visible = mode == NavMode.TELEOP
	goal_container.visible = mode == NavMode.GOAL
	goal_marker.visible = mode == NavMode.GOAL
	waypoints_container.visible = mode == NavMode.WAYPOINTS
	
	$VBox/Header/ModeSelector.select(option_id_to_mode.find(mode))
	
func on_waypoints_update(waypoints):
	waypoint_markers.set_markers(waypoints)

func on_goal_update(goal):
	if goal:
		goal_marker.translation.x = goal.x
		goal_marker.translation.z = goal.z

# Called when the node enters the scene tree for the first time.
func _ready():
	state.connect("nav_mode_update", self, "on_nav_mode_update")
	state.connect("waypoints_update", self, "on_waypoints_update")
	state.connect("goal_update", self, "on_goal_update")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass



func _on_OptionButton_item_selected(index):
	var mode = option_id_to_mode[index]
	state.set_navigation_mode(mode)


func _on_Ground_map_click(position):
	var goal = {
		"x": position.x,
		"z": position.z,
	}
	if current_navigation_mode == NavMode.GOAL:
		state.set_goal(goal)
	elif current_navigation_mode == NavMode.WAYPOINTS:
		state.add_waypoint(goal)
