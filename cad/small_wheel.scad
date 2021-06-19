$fn=100;

shaft_d = 3.1;
shaft_d_flat = 2.5;
shaft_l = 7;
shaft_outer_d = 7;

rim_d = 28;

pick_width = 3.78;
pick_diameter = 5.67;
PICK_DEPTH = 10;
PW_START = pick_width + 0.05;
PW_END = pick_width - 0.15;
PICK_TIGHT_ANGLE = atan2(PW_START-PW_END, PICK_DEPTH);
echo(PICK_TIGHT_ANGLE);
R1 = pick_diameter / 2;
R2 = 6;
R3 = R2 + 24;
teeth_depth = 6;
teeth_width = 8;
teeth_twist=30;
teeth_steps=30;
tire_thickness=3;
teeth_stand_out_angle = 0;
join_stand_out = 3;

// Close teeth

teeth_depth = 0.8;
teeth_width = 3;
teeth_twist=10;
teeth_steps=12;
tire_thickness=1.4;
tire_width = 16;
teeth_stand_out_angle = 8;

fasten_hole_shaft_d = 2.8;
fasten_hole_tire_d = 3.5;
fasten_hole_height = 3;

// Small settings
//WIDTH = 12;
//R3 = 16;
//teeth_depth = 2;
//teeth_width = 8;
//teeth_twist=5;
//teeth_steps=20;
//tire_thickness=1;

module rotate_about_pt(r, pt) {
    translate(pt)
    rotate(r)
    translate(-pt)
    children();   
}

module shaft() {
    difference() {
        cylinder(shaft_l, d=shaft_outer_d);
        difference() {
            cylinder(shaft_l, d1=shaft_d*1.1, d2=shaft_d*1.1);
            translate([-shaft_d/2,shaft_d_flat-shaft_d/2,0])
            cube([shaft_d, shaft_d, shaft_l]);
        }
        // screw hole for locking the motor
        translate([0,0,fasten_hole_height])
        rotate([-90,0,0])
        cylinder(shaft_outer_d/2, d=fasten_hole_shaft_d);
    }
}


module rim() {
    start = shaft_outer_d/2 - 0.5;
    r = rim_d/2 - start;
    spike_w = 1;
    for(i=[180:120:180*3]) {
        rotate([0,0,i])
        difference() {
            translate([spike_w/2,start,0])
            rotate([0,-90,0])
            cube([tire_width, r, spike_w]);
            
            translate([spike_w/2,start,tire_width])
            scale([1.1,1,1])
            translate([0.01,0,0])
            resize([spike_w,r*2,tire_width*2 - shaft_l], auto=true)
            rotate([0,-90,0])
            cylinder(spike_w, r=r);
        }
    }
}

module tire() {
    module toot() { 
        translate([rim_d/2,0,0]) 
        rotate([0,0,teeth_stand_out_angle]) 
        translate([0,-teeth_width,0]) 
        square([teeth_depth, teeth_width]);
    }
    difference() {
        union() {
            difference() {
                cylinder(tire_width, d=rim_d+tire_thickness);
                translate([0,0,-0.5])
                cylinder(tire_width+1, d=rim_d);
            }
//            rotate_extrude()
//            translate([rim_d/2,0,0]) 
//            square([tire_thickness, tire_width]);

            

            for(i=[0:teeth_steps:360]) {
                rotate([0,0,i])
                linear_extrude(tire_width/2, twist=teeth_twist)
                toot();
                
                rotate([0,0,i])
                translate([0,0,tire_width/2])
                linear_extrude(tire_width/2, twist=-teeth_twist)
                toot();
            }
        }
        
        // hole for accessing the fastening screw
        translate([0,rim_d/2,fasten_hole_tire_d])
        rotate([-90,0,0])
        cylinder(shaft_outer_d/2, d=fasten_hole_height, center=true);
    }
}


!shaft();
rim();
tire();
