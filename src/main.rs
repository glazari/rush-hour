mod game;
mod tui;

fn main() {
    //tui::clear();

    //let game = game::Game::full_grid();
    //tui::draw((3, 10), &game);

    //let game = game::Game::example_game();
    //tui::draw((12, 10), &game);

    //tui::draw_win((21, 10), &game);

    //let game = game::Game::hard_example_game();
    //let game = game::Game::example_game();
    //let moves = game.solve().expect("solution");
    let game = game::Game::from_string(
        "F H (0:0),P V (1:0),B V (4:0),X H (2:1),Q V (1:3),R H (5:2),C H (4:4),O V (0:5)",
    );
    let game = game::Game::from_file("games/level2.txt");
    tui::clear();
    tui::draw((21, 10), &game);

    let moves = game.solve().expect("solution");
    tui::animate_game(game.clone(), moves.clone());
    println!("\n\n");
}
