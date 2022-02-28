pub struct Game {
    pub cars: Vec<Car>,
}

pub struct Car {
    pub color: Color,
    pub vertical: bool,
    pub position: (u8, u8),
}

impl Car {
    pub fn new(color: Color, vertical: bool, position: (u8, u8)) -> Car {
        Car {
            color,
            vertical,
            position,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Green,
}
