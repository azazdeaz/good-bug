extends CSGTorus


onready var state = $"/root/State"

func _ready():
	state.connect("state_update", self, "on_state_update")

func on_state_update(state):
	var goal = state.navigator_state.goal;
	if goal:
		visible = true
		translation.x = goal.x
		translation.z = goal.z
	else:
		visible = false
