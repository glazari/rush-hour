pub struct Game {
    pub cars: Vec<Car>,
}

pub struct Car {
    pub color: u8,
    pub background: u8,
    pub vertical: bool,
    pub position: (u8, u8),
}

impl Car {
    pub fn new(color: u8, background: u8, vertical: bool, position: (u8, u8)) -> Car {
        Car {
            color,
            background,
            vertical,
            position,
        }
    }
}
