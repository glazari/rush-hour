fn main() {
    let game = Game {
        cars: vec![
            Car::new(1, 196, false, (0, 0)),
            Car::new(22, 64, true, (2, 2)),
        ],
    };
    game.draw()
}

struct Game {
    cars: Vec<Car>,
}

impl Game {
    fn draw(&self) {
        print!("\x1b[2J"); // clear screen
        print!("\x1b[0;0H"); // move cursor to top

        println!(" _________ ");
        println!("| o o o o |");
        println!("| o o o o  ");
        println!("| o o o o |");
        println!("| o o o o |");
        println!(" _________ ");

        for car in self.cars.iter() {
            Game::draw_car(car);
        }
        print!("\x1b[10;10H\n"); // move cursor to bottom
    }

    fn draw_car(car: &Car) {
        let (x, y) = (car.position.0 + 2, (car.position.1 * 2) + 3);
        print!("\x1b[{};{}H", x, y); // move cursor

        if car.vertical {
            println!(
                "\x1b[48;5;{}m\x1b[38;5;{}m^\x1b[1B\x1b[1D|\x1b[1B\x1b[0m",
                car.color, car.background
            );
        } else {
            print!(
                "\x1b[48;5;{}m\x1b[38;5;{}m-->\x1b[0m",
                car.color, car.background
            );
        }
    }
}

struct Car {
    color: u8,
    background: u8,
    vertical: bool,
    position: (u8, u8),
}

impl Car {
    fn new(color: u8, background: u8, vertical: bool, position: (u8, u8)) -> Car {
        Car {
            color,
            background,
            vertical,
            position,
        }
    }
    fn draw(&self) {
        let (x, y) = (self.position.0 + 2, (self.position.1 * 2) + 3);
        print!("\x1b[{};{}H", x, y); // move cursor

        if self.vertical {
            println!(
                "\x1b[48;5;{}m\x1b[38;5;{}m^\x1b[1B\x1b[1D|\x1b[1B\x1b[0m",
                self.color, self.background
            );
        } else {
            print!(
                "\x1b[48;5;{}m\x1b[38;5;{}m-->\x1b[0m",
                self.color, self.background
            );
        }
    }
}

/*
 * ANSI codes
 *
 * RESET:           \x1b[0m
 * COLOR 256:       \x1b[38;5;{n}
 * BACKGROUND 256:  \x1b[48;5;{n}
 * MOVE:            \x1b[{n}{dir}
 *      dirs: A (UP), B (DOWN), C (RIGHT), D (LEFT)
 *
 * CLEAR SCREEN:    \x1b[2J
 * SET POSITION:    \x1b[{row};{column}H
 */
