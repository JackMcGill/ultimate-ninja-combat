#random is used for generating the computers move
import random


moves_list = ["Punch of Fury","Kick of Punishment","Sword of Justice","Shuriken of Vengence","Nunchucks of Anger","Knife of Freedom"]


#this function displays the instructions
def instructions():
    print("")
    print("""Instructions: You'll be fighting against the computer, and the winner gets bragging rights.
To begin with, both you and the computer have $100 each. At the start of each round, you can place a bet in multiples of $5.
After the bet, you'll be asked to choose from one of the moves below:""")
    display_moves()
    print("If your balance drops below $0 you immediately lose the game, and if the computer's balance drops below $0 you immediately win the game")
    

#this function displays the moves
def display_moves():
    print("")
    print("List of moves")
    for i in range(len(moves_list)):
        print(i+1,moves_list[i])


#this function determines the new balances based off the players bet and the round outcome
def betting(bet,player_balance,computer_balance,outcome):
    if outcome == 0:
        player_balance = player_balance + bet
        computer_balance = computer_balance - bet
    elif outcome == 1:
        player_balance = player_balance - bet
        computer_balance = computer_balance + bet

    return player_balance,computer_balance
    

#this function displays a breakdown of each round to the user
def game_history(player_moves,computer_moves,outcomes,rounds,moves,name,player_balance,computer_balance):
    print("You played a total of",rounds,"rounds")
    print("You WON",outcomes.count(0),"rounds")
    print("You LOST",outcomes.count(1),"rounds")
    print("You TIED",outcomes.count(2),"rounds")
    print("")    
    for i in range(rounds):
        print("After Round {}/{}: {}'s balance: {} | Computer's balance: {}".format(i+1,rounds,name,player_balance[i+1],computer_balance[i+1]))
        print("{}'s move: {} | Computer's move: {}".format(name,moves[player_moves[i]],moves[computer_moves[i]]))
        outcome = compare_moves(player_moves[i],computer_moves[i])
        if outcome == 0:
            print(name,"Won!")
        elif outcome == 1:
            print("Computer Won!")
        else:
            print("This round was a tie")
        print("")                        


#this function finds the modal outcome
def find_mode(scores):
    wins = scores.count(0)
    lost = scores.count(1)
    ties = scores.count(2)
    totals = [wins,lost,ties]
    mode_total = totals.index(max(totals))
    
    if totals[0] == totals[1] == totals[2]:
        print("You won, lost and tied equal times!")
    elif totals[0] == totals[1] and totals[0] > totals[2]:
        print("You won and lost an equal number of games!")
    elif totals[0] == totals[2] and totals[0] > totals[1]:
        print("You won and tied an equal number of games!")
    elif totals[1] == totals[2] and totals[1] > totals[0]:
        print("You lost and tied an equal number of games!")
    else:
        if mode_total == 0:
            print("You won the majority of rounds!")
        elif mode_total == 1:
            print("You lost the majority of rounds!")
        else:
            print("You tied the majority of rounds!")
    

#this is where the game takes place.
def game(player_name,player_balance):       
    computer_balance = int(100)
    bet = int
    players_move = int
    correct_value_entered = bool(False)
    computers_move = int
    outcome = int
    outcome_list = ["You WIN!","You have LOST","This game is a TIE"]
    score_history = []
    player_move_history = []
    player_balance_history = [100]
    computer_move_history = []
    computer_balance_history = [100]
    average_score = int
    menu_input = str("C")
    game_round = int(0)
    
    while menu_input != "Q":
        if player_balance <= 0 or computer_balance <= 0:
            break
        if menu_input == "C":
            display_moves()
            print("")
            game_round = game_round + 1
            print("ROUND",game_round)
            
            print("{}'s balance: ${} | Computer's balance: ${}".format(player_name,player_balance,computer_balance))
            bet = int(input("How much would you like to bet? (bet can only be in multiples of 5): "))
            while bet < 0 or bet > player_balance or bet > computer_balance or bet%5 != 0: 
                bet = int(input("Your bet was invalid, please re-enter a bet. Remember - it must be a multiple of 5, and no greater than your balance or the computers: ")) 
            players_move = int(input("Choose your move: "))
            print("")
            correct_value_entered = False
            
            while correct_value_entered == False:            
                if players_move <= 6 and players_move > 0:
                    players_move = players_move - 1
                    computers_move = random.randint(0,5)
                    print(player_name, "chose", moves_list[players_move])
                    print("Computer chose",moves_list[computers_move])       
                    outcome = compare_moves(players_move,computers_move)
                    player_balance,computer_balance = betting(bet,player_balance,computer_balance,outcome)
                    print("Your new balance is: ${} | The computer's new balance is: ${}".format(player_balance,computer_balance))
                    print("")
                    print(outcome_list[outcome])
                    score_history.append(outcome)
                    player_move_history.append(players_move)
                    computer_move_history.append(computers_move)
                    player_balance_history.append(player_balance)
                    computer_balance_history.append(computer_balance)
                    correct_value_entered = True
                else:
                    print("incorrect value entered")
                    players_move = int(input("Choose your move: "))
                    correct_move_entered = False

                print("")
                menu_input = input("[C]ontinue playing, [I]nstructions, [Q]uit: ").upper()
                
        elif menu_input == "I":
            instructions()
            menu_input = input("[C]ontinue playing, [I]nstructions, [Q]uit: ").upper()
            
        else:
            print("The option you entered is invalid, please use one of the following:")
            menu_input = input("[C]ontinue playing, [I]nstructions, [Q]uit: ").upper()
                
                
    if player_balance <= 0 or computer_balance <= 0:
        if player_balance <= 0:            
            print("GAME OVER - your balance dropped below $0!")
        else:
            print("YOU WIN THE GAME! The computer's balance dropped below $0!")            
    view_history_choice = str(input("Would you like to see a breakdown of the rounds? [Y]es, [N]o: ").upper())
    
    while view_history_choice != "N":
        if view_history_choice == "Y":
            find_mode(score_history)
            game_history(player_move_history,computer_move_history,score_history,game_round,moves_list,player_name,player_balance_history,computer_balance_history)
            view_history_choice = "N"
        else:
            view_history_choice = ("The option you entered isn't available. Please enter either [Y]es or [N]o: ")
            
    return player_balance
   

#this function compares the players move against the computers move, returning the outcome of the game
def compare_moves(players_move,computers_move):
    punch_outcomes = [2,1,1,0,0,1]
    kick_outcomes = [0,2,1,1,0,1]
    sword_outcomes = [0,0,2,1,1,0]
    shuriken_outcomes = [1,0,0,2,1,0]
    nunchuck_outcomes = [1,1,0,0,2,1]
    knife_outcomes = [0,0,1,1,0,2]
    move_outcome_list = [punch_outcomes,kick_outcomes,sword_outcomes,shuriken_outcomes,nunchuck_outcomes,knife_outcomes]
    outcome = move_outcome_list[players_move][computers_move]

    return outcome


#this is main. a main menu with instructions is provided
def main():
    player_name = str("")
    menu_input = str("")
    player_balance = int(100)
    retain_balance_choice = str("")
    print("Players current balance: ${}".format(player_balance))
    player_name = input("Enter your name: ")
    print("Welcome {}, this is Ultimate Ninja Battle Combat!".format(player_name))    
    menu_input = input("[I]nstructions, [P]lay, [Q]uit: ").upper()
    while menu_input != "Q":                
        if menu_input == "P" or menu_input == "I":
            if menu_input == "P":
                player_balance = game(player_name,player_balance)
                print("")
                print("Thanks for playing {}! Your final balance was ${}".format(player_name,player_balance))
                print("***************************************")
                print("")
                menu_input = input("[I]nstructions, [P]lay, [Q]uit: ").upper()
                if menu_input == "P":
                    print("Would you like to retain your balance of ${} from the previous game, or start over with a balance of $100?".format(player_balance))
                    retain_balance_choice = input("[R]retain [S]tart over: ").upper()
                    if retain_balance_choice == "S":
                        player_balance = int(100)
            else:
                instructions()
                print("")
                menu_input = input("[I]nstructions, [P]lay, [Q]uit: ").upper()
        else:
            print("The option you entered is invalid, please use one of the following:")
            menu_input = input("[I]nstructions, [P]lay, [Q]uit: ").upper()
    print("GAME TERMINATED")
            
#this runs main, and subsequently starts the game
main()
