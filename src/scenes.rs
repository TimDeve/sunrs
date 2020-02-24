pub struct Scene {
    pub brightness: u16,
    pub kelvin: u16,
}

pub const DAYLIGHT: Scene = Scene {
    brightness: 0xFFFF,
    kelvin: 5000,
};

pub const BRIGHT: Scene = Scene {
    brightness: 0xFFFF,
    kelvin: 2700,
};

pub const COMPUTER: Scene = Scene {
    brightness: 19005,
    kelvin: 2700,
};

pub const READING: Scene = Scene {
    brightness: 32767,
    kelvin: 2000,
};

pub const DARK: Scene = Scene {
    brightness: 0x0001,
    kelvin: 2300,
};

pub const CHILL: Scene = Scene {
    brightness: 9830,
    kelvin: 2000,
};
