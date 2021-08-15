extends OptionButton

onready var state = $"/root/State"

var names = []

func _ready():
	state.connect("state_update", self, "on_state_update")

func on_state_update(state):
	clear()
	names.clear()
	
	if state.maps:
		for map in state.maps:
			var id = names.size()
			names.push_back(map.name)
			add_item(map.name, id)
			if state.current_map_name == map.name:
				select(id)

func _on_MapSelector_item_selected(index):
	state.set_current_map_name(names[index])
