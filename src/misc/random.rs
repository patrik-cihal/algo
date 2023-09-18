use std::time::SystemTime;

pub static mut RNG: Option<XorShift> = None;

pub fn rand_u64() -> u64 {
    let rng = unsafe {
        RNG.get_or_insert(XorShift::new((SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos() & 0xFFFFFFFFFFFFFFFF) as u64))
    };
    rng.next()
}

pub struct XorShift {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl XorShift {
    pub fn new(seed: u64) -> Self {
        XorShift {
            x: 9018237498,
            y: 1982731389,
            z: 1894712904,
            w: seed,
        }
    }

    pub fn next(&mut self) -> u64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }
}