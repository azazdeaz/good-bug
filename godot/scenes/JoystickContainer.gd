extends PanelContainer


func _physics_process(_delta):
	var joy = $Joystick
	if joy.is_working:
		print(joy.output," angle: ", joy.output.angle(), " lenght:",joy.output.length())
		print(vecToDiff(joy.output, -1.0, 1.0))


func vecToDiff(vec: Vector2, minSpeed, maxSpeed):	
	if vec.x == 0 and vec.y == 0:
		return [0, 0]
	

	# and in degrees
	var angle = vec.angle() * 180 / PI

	# Now angle indicates the measure of turn
	# Along a straight line, with an angle o, the turn co-efficient is same
	# this applies for angles between 0-90, with angle 0 the coeff is -1
	# with angle 45, the co-efficient is 0 and with angle 90, it is 1

	var tcoeff = -1 + (angle / 90) * 2
	var turn = tcoeff * abs(abs(vec.y) - abs(vec.x))
	turn = round(turn * 100) / 100

	# And max of y or x is the movement
	var mov = max(abs(vec.y), abs(vec.x))

	# First and third quadrant
	var rawLeft = 0
	var rawRight = 0
	if (vec.x >= 0 and vec.y >= 0) or (vec.x < 0 and vec.y < 0):
		rawLeft = mov
		rawRight = turn
	else:
		rawRight = mov
		rawLeft = turn

	# Reverse polarity
	if vec.y < 0:
		rawLeft = 0 - rawLeft
		rawRight = 0 - rawRight

	# minJoystick, maxJoystick, minSpeed, maxSpeed
	# Map the values onto the defined rang
#	var rightOut = map(rawRight, minJoystick, maxJoystick, minSpeed, maxSpeed)
#	var leftOut = map(rawLeft, minJoystick, maxJoystick, minSpeed, maxSpeed)

	return [rawLeft, rawRight]
