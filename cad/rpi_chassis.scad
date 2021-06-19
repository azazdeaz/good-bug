width=71;
length=150;
main_height=20;
front_height=150;
wall=1;

camera_cable_hole_width=18;
camera_cable_hole_height=1;
camera_cable_hole_z=56;

power_hole_z = 38;
power_hole_y=length-18;
power_hole_r=6;

$fn=100;

difference() {
    cube([width+wall*2,length+wall,front_height+wall]);
    
    // inside
    translate([wall,0,wall])
    cube([width,length,front_height]);
    
    // curve
    translate([0,length-front_height,front_height+wall])
    rotate([0,90,0])
    cylinder(width+wall*2,r=front_height);
    
    // camera cable hole
    #translate([
        (width-camera_cable_hole_width)/2+wall,
        length,
        camera_cable_hole_z
    ])
    cube([camera_cable_hole_width,wall,camera_cable_hole_height]);
    
    //power hole
    #
    translate([0,power_hole_y,power_hole_z+wall])
    rotate([0,90,0])
    cylinder(h=wall,r=power_hole_r);
    
    // top cut
    #difference() {
        translate([0,0,front_height-width/2+wall])
        cube([width+wall*2,length+wall,width/2]);

        translate([width/2+wall,length+wall,front_height-(width/2+wall)])
        rotate([90,0,0])
        cylinder(h=length+wall,r=width/2+wall);
    }
}

