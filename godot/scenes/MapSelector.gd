extends OptionButton

onready var state = $"/root/State"

var names = []
var NO_MAP = "no map"

func _ready():
	state.connect("state_update", self, "on_state_update")

func on_state_update(state):
	clear()
	names.clear()
	
	names.push_back(NO_MAP)
	add_item(NO_MAP)
	
	if state.maps:
		for map in state.maps:
			var id = names.size()
			names.push_back(map.name)
			add_item(map.name, id)
			if state.current_map_name == map.name:
				select(id)
				
	

func _on_MapSelector_item_selected(index):
	var name = names[index]
	if name == NO_MAP:
		name = ""
	state.set_current_map_name(name)
