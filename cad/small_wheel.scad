$fn=100;

shaft_d = 3.3;
shaft_d_flat = 2.6;
shaft_l = 7;
shaft_outer_d = 7;

fasten_hole_shaft_d = 2.75;
fasten_hole_tire_d = 3.5;
fasten_hole_height = 3.5;

rim_d = 48;

teeth_depth = 2.4;
teeth_width = 5;
teeth_twist=10;
teeth_steps=12;
tire_thickness=1.5;
tire_width = 23;
teeth_stand_out_angle = 10;

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
            cylinder(shaft_l, d1=shaft_d, d2=shaft_d);
            translate([-shaft_d/2,shaft_d_flat-shaft_d/2,0])
            cube([shaft_d, shaft_d, shaft_l]);
        }
        // screw hole for locking the motor
        translate([0,0,fasten_hole_height])
        rotate([-90,0,0])
        cylinder(shaft_outer_d/2, d=fasten_hole_shaft_d);
        
        // wider the hole entrance
        #cylinder(shaft_d/2, d1=shaft_d+0.4, d2=0);
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
        translate([rim_d/2-teeth_depth,0,0]) 
        rotate([0,0,teeth_stand_out_angle]) 
        translate([0,-teeth_width,0]) 
        square([teeth_depth, teeth_width]);
    }
    difference() {
        union() {
            cylinder(tire_width, d=rim_d);

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
        
        // cut the inside path of the wheel
        translate([0,0,-0.5])
        cylinder(tire_width+1, d=rim_d-tire_thickness);
        
        // hole for accessing the fastening screw
        translate([0,rim_d/2,fasten_hole_height])
        rotate([-90,0,0])
        cylinder(shaft_outer_d/2, d=fasten_hole_tire_d, center=true);
    }
}


shaft();
rim();
tire();
