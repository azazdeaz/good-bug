extends Node

var close_icon = load("res://assets/icons/Remove.svg")

var TELEOP = "teleop"
var GOAL = "goal"
var WAYPOINTS = "waypoints"

export(NodePath) var _teleop_container:NodePath
onready var teleop_container:Node = get_node(_teleop_container)
export(NodePath) var _goal_container:NodePath
onready var goal_container:Node = get_node(_goal_container)
export(NodePath) var _waypoints_container:NodePath
onready var waypoints_container:Node = get_node(_waypoints_container)
export(NodePath) var _goal_marker:NodePath
onready var goal_marker:Node = get_node(_goal_marker)
export(NodePath) var _waypoint_markers:NodePath
onready var waypoint_markers:Node = get_node(_waypoint_markers)

var option_id_to_mode = [TELEOP, GOAL, WAYPOINTS]

var current_navigation_mode = TELEOP

func set_navigation_mode(mode):
	current_navigation_mode = mode
	teleop_container.visible = mode == TELEOP
	goal_container.visible = mode == GOAL
	goal_marker.visible = mode == GOAL
	waypoints_container.visible = mode == WAYPOINTS
	waypoint_markers.visible = mode == WAYPOINTS
	
	$VBox/Header/ModeSelector.select(option_id_to_mode.find(mode))

# Called when the node enters the scene tree for the first time.
func _ready():
	var item1 = $VBox/WaypointContainer/WaypointsList.create_item()
	item1.set_text(0, "item1")
	item1.add_button(0, close_icon)


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_OptionButton_item_selected(index):
	var mode = option_id_to_mode[index]
	set_navigation_mode(mode)
