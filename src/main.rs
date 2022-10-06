mod settings;
mod game;

use settings::Settings::Settings;
use game::Game::Game;

fn main () {
    let settings = Settings {};
    settings.initial_menu();

    let option = settings.read_input();

    match option {
        1 => {
            let mut game = Game::new();

            loop {
                game.show_board();
                game.read_input();

                let winner = game.check_winner();

                if winner != 0 {
                    break;
                }

                game.next_player();
                settings.clear_terminal();
            }
        }

        _ => {
            settings.clear_terminal();
            settings.end_menu();
        }
    }
}