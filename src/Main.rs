// internal
mod settings;
mod game;

use settings::Settings::Settings;
use game::Game::Game;


fn main () {
    let settings = Settings {};
    settings.initial_menu();

    loop {
        let option = settings.read_input();
        match option {
            1 => {
                let mut game = Game::new();
    
                loop {
                    game.show_board();
                    let input_valid = game.read_input();

                    if !input_valid {
                        game.invalid_option();
                        continue;
                    }
    
                    if game.check_winner() != 0 || game.is_finished() == true {
                        break;
                    }
    
                    game.next_player();
                    settings.clear_terminal();
                }

                break;
            }

            2 => {
                settings.clear_terminal();
                settings.end_menu();
                break;
            }
    
            _ => {
                settings.invalid_option();
            }
        }
    }
}