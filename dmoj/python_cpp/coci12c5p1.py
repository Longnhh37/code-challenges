# coci12c5p1

notes = input().split('|')

a_mol = sum(note[0] in 'ADE' for note in notes)
c_dur = sum(note[0] in 'CFG' for note in notes)

if a_mol > c_dur:
    print("A-mol")
elif c_dur > a_mol:
    print("C-dur")
else:
    print("C-dur" if notes[-1][-1] == 'C' else 'A-mol')
