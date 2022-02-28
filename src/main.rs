mod game;
mod tui;

fn main() {
    let game = game::Game {
        cars: vec![
            game::Car::new(game::Color::Red, false, (0, 0)),
            game::Car::new(game::Color::Green, true, (2, 2)),
        ],
    };
    tui::draw(&game);
}
