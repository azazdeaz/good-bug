padd_left = 14;
padd_right = 14;
hdmi_height = 19.7;
usb_height = 17.1;
lan_height = 15.4;
hdmi_width = 19;
usb_width = 34;
lan_width = 17;
depth = 21;

step1_height = 1;
step1_width = padd_left + hdmi_width;

step2_height = step1_height + (hdmi_height - usb_height);
step2_width = usb_width;

step3_height = step1_height + (hdmi_height - lan_height);
step3_width = lan_width + padd_right;

translate([0,0,-step1_height])
cube([step1_width, depth, step1_height]);

translate([step1_width,0,-step2_height])
cube([step2_width, depth, step2_height]);


translate([step1_width + step2_width,0,-step3_height])
cube([step3_width, depth, step3_height]);

