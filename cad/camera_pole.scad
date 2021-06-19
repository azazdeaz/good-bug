$fn=40;

pole_width = 28;
pole_depth = 5;
pole_height = 120;
mount_screw_hole_start_diameter = 7;
mount_screw_hole_end_diameter = 3.2;
mount_screw_hole_end_thickness = 1;
mount_hole_distance = 20;

module screw_hole(x, y) {
    tx = pole_width/2 + mount_hole_distance/2 * x;
    ty = pole_width/2 + mount_hole_distance/2 * y;
    union() {
        translate([tx, 0, ty])
        rotate([-90,0,0])
        cylinder(
            h=pole_depth - mount_screw_hole_end_thickness,
            d=mount_screw_hole_start_diameter
        );
        
        translate([tx, 0, ty])
        rotate([-90,0,0])
        cylinder(
            h=pole_depth,
            d=mount_screw_hole_end_diameter
        );
    }
}

difference() {
    cube([pole_width, pole_depth, pole_height]);
    screw_hole(-1,-1);
    screw_hole(-1,0);
    screw_hole(-1,1);
    screw_hole(0,-1);
    screw_hole(0,0);
    screw_hole(0,1);
    screw_hole(1,-1);
    screw_hole(1,0);
    screw_hole(1,1);
}