// CSG.scad - Basic example of CSG usage

$fn=100;
//translate([-24,0,0]) {
//    union() {
//        cube(15, center=true);
//        sphere(10);
//    }
//}
//
//intersection() {
//    cube(15, center=true);
//    sphere(10);
//}
//
//translate([24,0,0]) {
//    difference() {
//        cube(15, center=true);
//        sphere(10);
//    }
//}

PICK_WIDTH = 3.78;
PICK_DIAMETER = 5.67;
PICK_DEPTH = 10;
PW_START = PICK_WIDTH + 0.05;
PW_END = PICK_WIDTH - 0.15;
PICK_TIGHT_ANGLE = atan2(PW_START-PW_END, PICK_DEPTH);
echo(PICK_TIGHT_ANGLE);
R1 = PICK_DIAMETER / 2;
R2 = 6;
WIDTH = 25.0;
R3 = R2 + 24;
TEETH_DEPTH = 6;
TEETH_WIDTH = 8;
TEETH_TWIST=30;
TEETH_STEPS=30;
TIRE_THICKNESS=3;
TEETH_STAND_OUT_ANGLE = 0;
JOIN_STAND_OUT = 3;

// Close teeth
TEETH_DEPTH = 2.1;
TEETH_WIDTH = 9;
TEETH_TWIST=10;
TEETH_STEPS=12;
TIRE_THICKNESS=1.6;
TEETH_STAND_OUT_ANGLE = 8;

// Small settings
//WIDTH = 12;
//R3 = 16;
//TEETH_DEPTH = 2;
//TEETH_WIDTH = 8;
//TEETH_TWIST=5;
//TEETH_STEPS=20;
//TIRE_THICKNESS=1;


module rotate_about_pt(r, pt) {
    translate(pt)
    rotate(r)
    translate(-pt)
    children();   
}

//translate([0,WIDTH/2,0])
//rotate([90, 0, 0]) 
difference() {
    cylinder(WIDTH+JOIN_STAND_OUT, PICK_DIAMETER, PICK_DIAMETER);
    translate([0,0,(WIDTH+JOIN_STAND_OUT)-PICK_DEPTH]) {
        cylinder(PICK_DEPTH, 0, R1);
        intersection() {
            translate([0,0,0.1])
            cylinder(PICK_DEPTH, R1, R1);
            intersection() {
                translate([-PICK_DIAMETER/2,-PW_START/2, 0])
                    rotate_about_pt([PICK_TIGHT_ANGLE,0,0],[0,0,PICK_DEPTH])
                    cube([PICK_DIAMETER,PW_START,PICK_DEPTH]);   
                translate([-PICK_DIAMETER/2,-PW_START/2, 0])
                    rotate_about_pt([-PICK_TIGHT_ANGLE,0,0],[0,PW_START,PICK_DEPTH])
                    cube([PICK_DIAMETER,PW_START,PICK_DEPTH]);  
            }
        }
    }
    
}

rotate_extrude()
translate([R3,0,0]) 
square([TIRE_THICKNESS, WIDTH]);

module toot() { 
    translate([R3,0,0]) 
    rotate([0,0,TEETH_STAND_OUT_ANGLE]) 
    translate([0,-TEETH_WIDTH,0]) 
    square([TEETH_DEPTH, TEETH_WIDTH]);
}

for(i=[0:TEETH_STEPS:360]) {
    rotate([0,0,i])
    linear_extrude(WIDTH/2, twist=TEETH_TWIST)
    toot();
    
    rotate([0,0,i])
    translate([0,0,WIDTH/2])
    linear_extrude(WIDTH/2, twist=-TEETH_TWIST)
    toot();
    
}
for(i=[0:60:360]) {
    rotate([0,0,i])
    difference() {
        translate([1,R2,0])
        scale([1,1.1,1])
        translate([0,-1,0])
        rotate([0,-90,0])
        cube([WIDTH, R3-R2,2]);
        
        translate([1,R2+(R3-R2)/2,WIDTH])
        scale([1.1,1,1])
        translate([0.1,0,0])
        rotate([0,-90,0])
        cylinder(2,r=(R3-R2)/1.94);
    }
}

echo(version=version());
// Written by Marius Kintel <marius@kintel.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain worldwide. This software is distributed without any
// warranty.
//
// You should have received a copy of the CC0 Public Domain
// Dedication along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
