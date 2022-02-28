mod game;
mod tui;

use game::Car;

fn main() {
    let game = game::Game {
        cars: vec![
            Car::new(game::Color::RedMain, false, (0, 0)),
            Car::new(game::Color::Green, true, (2, 2)),
            Car::new(game::Color::Pink, true, (2, 3)),
            Car::new(game::Color::Purple, false, (1, 0)),
            Car::new(game::Color::Grey, false, (2, 0)),
            Car::new(game::Color::Yellow, false, (3, 0)),
            Car::new(game::Color::Blue, false, (4, 0)),
            Car::new(game::Color::Brown, false, (5, 0)),
            Car::new(game::Color::Orange, false, (0, 2)),
            Car::new(game::Color::LightYellow, false, (1, 2)),
            Car::new(game::Color::Bege, false, (4, 2)),
            Car::new(game::Color::Cyan, false, (5, 2)),
            Car::new(game::Color::LightPurple, false, (0, 4)),
            Car::new(game::Color::SeaBlue, false, (1, 4)),
            Car::new(game::Color::PukeGreen, false, (2, 4)),
            Car::new(game::Color::BlueGreen, false, (3, 4)),
        ],
    };
    tui::draw(&game);

    for i in 0..256 {
        print!("\x1b[48;5;{}m {} ", i, i);
        print!("\x1b[0m ");
    }
    println!("\x1b[0m");
}
