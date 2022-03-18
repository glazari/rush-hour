mod game;
mod tui;

fn main() {
    tui::clear();

    let game = game::Game::full_grid();
    tui::draw((3, 10), &game);

    let game = game::Game::example_game();
    tui::draw((12, 10), &game);

    tui::draw_win((21, 10), &game);

    let game = game::Game::example_game();
    let moves = game.solve().expect("solution");
    tui::clear();
    tui::draw((21, 10), &game);

    tui::animate_game(game.clone(), moves.clone());
    println!("{:?}", moves);
    println!("{}", game.to_string());
}
