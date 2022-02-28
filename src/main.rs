fn main() {
    print!("\x1b[2J"); // clear screen
    print!("\x1b[0;0H"); // move cursor to top

    println!(" _________ ");
    println!("| o o o o |");
    println!("| o o o o  ");
    println!("| o o o o |");
    println!("| o o o o |");
    println!(" _________ ");

    // draw horizontal car
    let car = Car::new(1, 196, false, (0, 0));
    car.draw();

    let car = Car::new(22, 64, true, (2, 1));
    car.draw();

    print!("\x1b[10;10H\n"); // move cursor to bottom
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
