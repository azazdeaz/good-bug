width=12;
height=10;
depth=24.5;
wall=1;
shaft_d=4.2;
screw_holes_from_center = 4.5;
screw_hole_d = 1.8;
screw_head_d = 2.8;
screw_head_depth = 0.6;
mounting_hole_distance = 20;
mounting_screw_d = 3.1;
mounting_plate_size = 28;

$fn=40;

module motor_box() {
    difference() {
        difference() {
            translate([-wall,0,-wall])
            cube([width+wall*2, depth+wall, height+wall*2]);
            cube([width, depth, height]);
        }
        translate([width/2,depth,height/2])
        rotate([-90,0,0])
        cylinder(wall, d=shaft_d);
    }
}

module screw_hole(offset) {
    translate([width/2+offset,depth,height/2])
    rotate([-90,0,0])
    union() {
        cylinder(wall, d=screw_hole_d);
        translate([0,0,wall-screw_head_depth])
            cylinder(screw_head_depth, d1=screw_hole_d, d2=screw_head_d);
    }
}

module mounting_screw_hole(x, y) {
    off = mounting_hole_distance / 2;
    translate([
        mounting_plate_size/2 + off*x,
        mounting_plate_size/2 + off*y,
        0
    ])
    cylinder(h=wall, d=mounting_screw_d);
}

module mounting_ears() {
    translate([
        width/2-mounting_plate_size/2,
        (depth+wall)-mounting_plate_size,
        height
    ])
    difference() { 
        cube([mounting_plate_size, mounting_plate_size, wall]);
        mounting_screw_hole(1,1);
        mounting_screw_hole(-1,1);
        mounting_screw_hole(1,-1);
        mounting_screw_hole(-1,-1);
    }
}

difference() {
    motor_box();
    screw_hole(screw_holes_from_center);
    screw_hole(-screw_holes_from_center);
}
mounting_ears();