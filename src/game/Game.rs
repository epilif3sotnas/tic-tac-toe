// rust
use std::io::stdin;


pub struct Game<'a> {
    game: [&'a str; 9],
    player_play: u8,
    num_plays: u8
}

impl Game<'_> {
    
    pub fn new () -> Game<'static> {
        return Game {
            game: ["-"; 9],
            player_play: 1,
            num_plays: 0
        };
    }

    pub fn show_board (&self) {
        println!("+---+---+---+");
        println!("| {} | {} | {} |", self.game.get(0).unwrap(), self.game.get(1).unwrap(), self.game.get(2).unwrap());
        println!("+---+---+---+");
        println!("| {} | {} | {} |", self.game.get(3).unwrap(), self.game.get(4).unwrap(), self.game.get(5).unwrap());
        println!("+---+---+---+");
        println!("| {} | {} | {} |", self.game.get(6).unwrap(), self.game.get(7).unwrap(), self.game.get(8).unwrap());
        println!("+---+---+---+");

        println!("\n\n\nO -> {}", "Player 1");
        println!("X -> {}", "Player 2");
    }

    pub fn read_input (&mut self) {
        println!("Player {}\nLocation: ", self.player_play);
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let position: usize = input
                                    .trim()
                                    .parse()
                                    .expect("Failed to parse");

        if self.check_play(&position) {
            self.update_game(&position)
        }
    }

    fn check_play (&self, &position: &usize) -> bool {
        if position >= 1 && position <= 9 && self.game[position - 1] == "-" {
            return true;
        } else {
            return false;
        }
    }

    fn update_game (&mut self, &position: &usize) {
        match self.player_play {
            1 => {
                self.game[position - 1] = "O";
                self.num_plays += 1;
            }

            2 => {
                self.game[position - 1] = "X";
                self.num_plays += 1;
            }

            other => {}
        }
    }

    pub fn check_winner (&self) -> u8 {
        let winning_cases = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for win in winning_cases {
            match self.player_play {
                1 => {
                    if self.game[win[0]] == "O" && self.game[win[1]] == "O" && self.game[win[2]] == "O" {
                        self.show_board();
                        println!("Player {} won", self.player_play);
                        return self.player_play;
                    }
                }

                2 => {
                    if self.game[win[0]] == "X" && self.game[win[1]] == "X" && self.game[win[2]] == "X" {
                        self.show_board();
                        println!("Player {} won", self.player_play);
                        return self.player_play;
                    }
                }

                other => {}
            }
        }
        return 0;
    }

    pub fn is_finished (&self) -> bool {
        return if self.num_plays == 9 {
            self.show_board();
            println!("Game finished. Result is a draw");
            true 
        } else {
            false
        };
    }
        
    pub fn next_player (&mut self) {
        match self.player_play {
            1 => {
                self.player_play = 2;
            }

            2 => {
                self.player_play = 1;
            }

            other => {}
        }
    }
}