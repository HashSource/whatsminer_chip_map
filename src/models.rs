/// Color coding mode for chip visualization
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorMode {
    #[default]
    Temperature,
    Errors,
    Crc,
}

impl ColorMode {
    pub const ALL: [ColorMode; 3] = [ColorMode::Temperature, ColorMode::Errors, ColorMode::Crc];
}

impl std::fmt::Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMode::Temperature => write!(f, "Temperature"),
            ColorMode::Errors => write!(f, "Errors"),
            ColorMode::Crc => write!(f, "CRC"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MinerData {
    pub slots: Vec<Slot>,
}

impl MinerData {
    pub fn total_chips(&self) -> usize {
        self.slots.iter().map(|s| s.chips.len()).sum()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Slot {
    pub id: i32,
    pub freq: i32,
    pub temp: f64,
    pub step: i32,
    pub nonce_valid: i64,
    pub nonce_rate: i32,
    pub errors: i32,
    pub crc: i32,
    pub chips: Vec<Chip>,
}

#[derive(Debug, Clone, Default)]
pub struct Chip {
    pub id: i32,
    pub freq: i32,
    pub vol: i32,
    pub temp: i32,
    pub nonce: i64,
    pub errors: i32,
    pub crc: i32,
    pub x: i32,
    pub repeat: i32,
    pub pct1: f32,
    pub pct2: f32,
}
