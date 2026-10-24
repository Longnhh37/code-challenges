from collections import defaultdict

for _ in range(int(input())):
    left, right = input().split("->")
    d = defaultdict(int)

    for sign, side in [(1, left), (-1, right)]:
        for mol in side.split("+"):
            if mol[0].isdecimal():
                n_mol = int(mol[0])
                mol = mol[1:]
            else:
                n_mol = 1

            for i in range(len(mol)):
                atom = mol[i]

                if i < len(mol) - 1:
                    next_atom = mol[i + 1]
                else:
                    next_atom = "1"

                if atom.isalpha() and not next_atom.isdecimal():
                    d[atom] += sign * n_mol
                elif atom.isalpha() and next_atom.isdecimal():
                    d[atom] += sign * n_mol * int(next_atom)

    if all(v == 0 for v in d.values()):
        print("DA")
    else:
        print("NE")
