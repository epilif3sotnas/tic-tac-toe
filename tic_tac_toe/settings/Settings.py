class Settings (object):

    def initial_menu (self) -> None:
        print("----- Initial Menu -----")
        print("\t1 - Start Game.")
        print("\t2 - Exit.")

    def end_menu (self) -> None:
        print("----- Nice to Spend Time With You -----")
        print("----- Bye -----")

    def read_input (self) -> int:
        try:
            return int(input())

        except Exception as e:
            print(e)

    def clear_terminal (self) -> None:
        print(chr(27) + "[2J")