class Game (object):
    __game: list = []
    __player_play: int = 0


    def __init__ (self) -> object:
        self.__game = ["-"] * 9
        self.__player_play = 1

    def show_board (self) -> None:
        print("+---+---+---+")
        print("| {} | {} | {} |".format(self.__game[0], self.__game[1], self.__game[2]))
        print("+---+---+---+")
        print("| {} | {} | {} |".format(self.__game[3], self.__game[4], self.__game[5]))
        print("+---+---+---+")
        print("| {} | {} | {} |".format(self.__game[6], self.__game[7], self.__game[8]))
        print("+---+---+---+")

        print("\n\n\nO -> {}".format("Player 1"))
        print("X -> {}".format("Player 2"))

    def read_input (self) -> None:
        try:
            position: int = int(input("Position: "))

        except Exception as e:
            print(e)

        else:
            if self.__check_play(position):
                self.__update_game(position)

            else:
                return False

    def __check_play (self, position) -> bool:
        if (
            type(position) == int
            and 1 <= position <= 9
            and self.__game[position - 1] == "-"
        ):
            return True

        else:
            return False

    def __update_game (self, position):
        match self.__player_play:
            
            case 1:
                self.__game[position - 1] = "O"

            case 2:
                self.__game[position - 1] = "X"

    def check_winner (self) -> int:
        winning_cases: list = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ]

        for win in winning_cases:
            match self.__player_play:

                case 1:
                    if (
                        self.__game[win[0]] == "O"
                        and self.__game[win[1]] == "O"
                        and self.__game[win[2]] == "O"
                    ):
                        self.show_board()
                        print("Player {} won".format(self.__player_play))
                        return self.__player_play

                case 2:
                    if (
                        self.__game[win[0]] == "X"
                        and self.__game[win[1]] == "X"
                        and self.__game[win[2]] == "X"
                    ):
                        self.show_board()
                        print("Player {} won".format(self.__player_play))
                        return self.__player_play

        return 0

    def next_player (self) -> None:
        match self.__player_play:
            
            case 1:
                self.__player_play = 2

            case 2:
                self.__player_play = 1