mod game;
mod tui;

// use game::Car;

fn main() {
    tui::clear();

    let game = game::Game::full_grid();
    tui::draw((3, 10), &game);

    let game = game::Game::example_game();
    tui::draw((12, 10), &game);
}
