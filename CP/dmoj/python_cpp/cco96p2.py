DIGITS = "0123456789"


def guess_consistent(number, code, correct, misplaced):
    number = list(number)
    code = list(code)
    num_correct = 0
    num_misplaced = 0

    for i in range(len(number)):
        if number[i] == code[i]:
            num_correct += 1
            number[i] = ""
            code[i] = ""

    for i in range(len(number)):
        if number[i] != "" and number[i] in code:
            where = code.index(number[i])
            code[where] = ""
            num_misplaced += 1
    return correct == num_correct and misplaced == num_misplaced


def all_guess_consistent(number, guesses):
    for guess in guesses:
        code = guess[0]
        correct = guess[1]
        misplaced = guess[2]
        if not guess_consistent(number, code, correct, misplaced):
            return False
    return True


def correctness_string(guesses):
    num_consistent = 0
    for d1 in DIGITS:
        for d2 in DIGITS:
            for d3 in DIGITS:
                for d4 in DIGITS:
                    number = d1 + d2 + d3 + d4
                    if all_guess_consistent(number, guesses):
                        answer = number
                        num_consistent += 1
                        if num_consistent > 1:
                            return "indeterminate"
    if num_consistent == 0:
        return "impossible"
    else:
        return answer


N = int(input())

for _ in range(N):
    G = int(input())
    guesses = []
    for _ in range(G):
        line = input()
        code = line[:4]
        correct = int(line[5])
        misplaced = int(line[7])
        guesses.append((code, correct, misplaced))

    print(correctness_string(guesses))
