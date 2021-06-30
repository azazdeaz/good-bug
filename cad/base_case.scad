$fn=40;

width = 80;
depth = 114;
height = 28;
wall = 2.3;

switch_hole_width = 12;
switch_hole_from_back = 5;

charge_hole_width = 8;
charge_hole_height = 6;
charge_hole_from_back = 9;
charge_hole_from_bottom = 2;

antenna_hole_diameter = 6.9;
antenna_hole_from_bottom = 15;
antenna_hole_one_from_left = 15;
antenna_hole_two_from_right = 10;

mount_screw_hole_diameter = 2.85;
mount_screw_hole_padding_diameter = 6;
mount_screw_hole_length = 5;
mount_hole_distance = 20;
mount_plate_size = 28;
mount_plate_thickness = 6;
mount_plate_bottom_from_back = 18;
mount_plate_bottom_from_front = 1.5;

wheel_mount_from_front = 5;
wheel_mount_from_back = 8;

motor_cable_in_hole_diameter=4;
motor_cable_tube_diameter=12;
motor_cable_tube_from_front=50;

board_bottom_height = mount_screw_hole_length-wall;

module body() {
    difference() {
        translate([-wall,-wall,-wall])
        cube([width+wall*2,depth+wall*2,height+wall]);
        cube([width,depth,height]);
    }
}

module switch_hole() {
    translate([
        -wall, 
        depth-(switch_hole_from_back+switch_hole_width), 
        board_bottom_height
    ])
    cube([wall, switch_hole_width, height]);
}

module charge_hole() {
    translate([
        width, 
        depth-(charge_hole_from_back+charge_hole_width), 
        charge_hole_from_bottom + board_bottom_height
    ])
    cube([wall, charge_hole_width, charge_hole_height]);
}

module antenna_hole(from_left) {
    translate([from_left,depth+wall,antenna_hole_from_bottom])
    rotate([90,0,0])
    cylinder(h=wall,d=antenna_hole_diameter);
}

module antenna_holes() {
    #union() {
        antenna_hole(antenna_hole_one_from_left);
        antenna_hole(width - antenna_hole_two_from_right);
    }
}

module insert_hole(x=0, y=0, z=0, rx=0, ry=0, rz=0) {
    difference() {
        union() {
            translate([x,y,z])
            rotate([rx,ry,rz])
            cylinder(
                h=mount_screw_hole_length, 
                d=mount_screw_hole_padding_diameter
            );
            children();
        }
        translate([x,y,z])
        rotate([rx,ry,rz])
        cylinder(
            h=mount_screw_hole_length, 
            d=mount_screw_hole_diameter
        );
    }
}

module mount_plate_bottom(from_front=0, from_left=0) {
    close = (mount_plate_size - mount_hole_distance) / 2;
    far = close + mount_hole_distance;

    insert_hole(x=close+from_left, y=close+from_front, z=-wall)
    insert_hole(x=far+from_left, y=close+from_front, z=-wall)
    insert_hole(x=close+from_left, y=far+from_front, z=-wall)
    insert_hole(x=far+from_left, y=far+from_front, z=-wall)
    children();
}

module mount_bottom() {
    front = mount_plate_bottom_from_front;
    back = depth-(mount_plate_size+mount_plate_bottom_from_back);
    left = -wall;
    right = (width+wall)-mount_plate_size;
    
    mount_plate_bottom(from_front=front, from_left=left)
    mount_plate_bottom(from_front=front, from_left=right)
    mount_plate_bottom(from_front=back, from_left=left)
    mount_plate_bottom(from_front=back, from_left=right)
 
    mount_plate_bottom(from_front=front+5, from_left=left)
    mount_plate_bottom(from_front=front+5, from_left=right)
    mount_plate_bottom(from_front=back-5, from_left=left)
    mount_plate_bottom(from_front=back-5, from_left=right)
    
//    mount_plate_bottom(from_front=front+10, from_left=left)
//    mount_plate_bottom(from_front=front+10, from_left=right)
//    mount_plate_bottom(from_front=back-10, from_left=left)
//    mount_plate_bottom(from_front=back-10, from_left=right)
    children();
}

module mount_hole_front(x, y) {
    step = mount_hole_distance/2;
    start_from_left = width/2;
    start_from_bottom = height/2;
    
    insert_hole(x=start_from_left+step*x, z=start_from_bottom+step*y, rx=90)
    children();
}

module mount_front() {
    mount_hole_front(-3, -1)
    mount_hole_front(-2, -1)
    mount_hole_front(-1, -1)
    mount_hole_front(0, -1)
    mount_hole_front(1, -1)
    mount_hole_front(2, -1)
    mount_hole_front(3, -1)
    mount_hole_front(-3, 0)
    mount_hole_front(-2, 0)
    mount_hole_front(-1, 0)
    mount_hole_front(0, 0)
    mount_hole_front(1, 0)
    mount_hole_front(2, 0)
    mount_hole_front(3, 0)
    mount_hole_front(-3, 1)
    mount_hole_front(-2, 1)
    mount_hole_front(-1, 1)
    mount_hole_front(0, 1)
    mount_hole_front(1, 1)
    mount_hole_front(2, 1)
    mount_hole_front(3, 1)
    union() {
        translate([-wall,-mount_screw_hole_length,-wall])
        cube([width+wall*2,mount_screw_hole_length,height+wall]);
        children();
    }
}


module motor_cable_hole(from_front=0, from_left=0) {
    translate([from_left, from_front, -wall])
    cylinder(h=wall, d=motor_cable_in_hole_diameter);
}

module motor_cable_holes() {
    front = mount_plate_size/2+mount_plate_bottom_from_front;
    back = depth-(mount_plate_size/2+mount_plate_bottom_from_back);
    left = mount_plate_size+motor_cable_in_hole_diameter/2;
    right = width-(mount_plate_size+motor_cable_in_hole_diameter/2);
    union() {
        motor_cable_hole(from_front=front, from_left=left);
        motor_cable_hole(from_front=front, from_left=right);
        motor_cable_hole(from_front=back, from_left=left);
        motor_cable_hole(from_front=back, from_left=right);
    }
}

module motor_cable_tube() {
    d = motor_cable_tube_diameter;
    difference() {
        union() {
            translate([width,motor_cable_tube_from_front,-wall])
            cylinder(h=height+wall, d=d);
            children();
        }
        translate([width,motor_cable_tube_from_front,0])
        cylinder(h=height, d=d-wall*2);
        translate([width-d/2,motor_cable_tube_from_front-d/2,0])
        cube([d/2,d,height]);
    }
}
intersection() {
//    translate([-wall,-wall,2])
//    cube([width+wall*2,depth+wall*2,1]);
    mount_front()
    motor_cable_tube()
    mount_bottom()
    difference() {  
        body();
        switch_hole();
        charge_hole();
        antenna_holes();
        motor_cable_holes();
    }
}

