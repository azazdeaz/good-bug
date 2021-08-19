extends Node


func _ready():
	$ConnectionHistory.get_popup().connect("id_pressed", self, "select_history_address")

func _on_ReconnectBtn_pressed():
	var game = get_node("/root/Game")
	game.reconnect($ConnectionAddress.text)

func select_history_address(id):
	var address = $ConnectionHistory.get_popup().get_item_text(id);
	$ConnectionAddress.set_text(address)
	var game = get_node("/root/Game")
	game.reconnect($ConnectionAddress.text)
	


