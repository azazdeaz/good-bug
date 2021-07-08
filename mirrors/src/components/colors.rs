use gdnative::prelude::Color;

pub struct C {
    r: i32,
    g: i32,
    b: i32,
}
impl C {
    pub fn as_godot(&self) -> Color {
        Color::rgb(
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        )
    }
}

pub const FRAME: C = C {
    r: 0xe7,
    g: 0x83,
    b: 0xfc,
};
pub const EDGE: C = C {
    r: 0x63,
    g: 0x92,
    b: 0xff,
};
pub const CURRENT_FRAME: C = C {
    r: 0xff,
    g: 0x77,
    b: 0x5e,
};
pub const LANDMARK1: C = C {
    r: 0x1c,
    g: 0xff,
    b: 0x9f,
};
pub const LANDMARK2: C = C {
    r: 0x96,
    g: 0xff,
    b: 0x08,
};
