# internal
from settings.Settings import Settings
from game.Game import Game


def main ():
    settings: Settings = Settings()
    settings.initial_menu()

    option = settings.read_input()

    match option:

        case 1:
            game = Game()
            while True:
                game.show_board()
                game.read_input()
                winner: int = game.check_winner()

                if winner:
                    break

                game.next_player()
                settings.clear_terminal()

        case other:
            settings.clear_terminal()
            settings.end_menu()

if __name__ == '__main__':
    main()