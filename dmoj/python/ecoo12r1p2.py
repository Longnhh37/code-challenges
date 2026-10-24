#  ecoo12r1p2
def rev_com(s, mp):  # reverse complement
    return "".join(mp.get(c) for c in reversed(s))


def com(s, mp):
    return "".join(mp.get(c) for c in s)


for dataset in range(5):
    strand = input()

    dna_map = {"A": "T", "T": "A", "C": "G", "G": "C"}
    rna_map = {"A": "U", "T": "A", "C": "G", "G": "C"}

    promoter = strand.index("TATAAT") + 10
    terminator = promoter

    found = False

    while not found:
        i = terminator + 6
        sequence = strand[terminator:i]
        if rev_com(sequence, dna_map) in strand[i:]:
            found = True
        else:
            terminator += 1

    transcription_unit = strand[promoter:terminator]
    rna = com(transcription_unit, rna_map)

    print(f"{dataset + 1}: {rna}")
