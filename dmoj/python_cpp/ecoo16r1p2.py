for _ in range(10):
    input()  # skip N
    spinner = set(map(int, input().split()))
    targets = list(map(int, input().split()))

    can_make = [False] * len(targets)

    for spin1 in spinner:
        for roll1 in [5, 6]:
            for spin2 in spinner:
                for roll2 in [5, 6]:
                    for spin3 in spinner:
                        score = spin1
                        if roll1 == 5:
                            score += spin2
                        else:  # roll1 == 6
                            score *= spin2

                        if roll2 == 5:
                            score += spin3
                        else:  # roll2 == 5
                            score *= spin3

                        if score in targets:
                            where = targets.index(score)
                            can_make[where] = True

    out = ""
    for value in can_make:
        if value:
            out += "T"
        else:
            out += "F"
    print(out)
