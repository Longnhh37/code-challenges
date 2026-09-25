with open('mixmilk.in', 'r') as fin:
    c1, m1 = list(map(int, fin.readline().split()))
    c2, m2 = list(map(int, fin.readline().split()))
    c3, m3 = list(map(int, fin.readline().split()))


    for i in range(33):
        amount = m1
        to_pour = min(m1, c2 - m2)
        m1 -= to_pour
        m2 += to_pour
        
        amount = m2
        to_pour = min(m2, c3 - m3)
        m2 -= to_pour
        m3 += to_pour

        amount = m3
        to_pour = min(m3, c1 - m1)
        m3 -= to_pour
        m1 += to_pour


    # 100
    amount = m1
    to_pour = min(m1, c2 - m2)

    m1 -= to_pour
    m2 += to_pour

with open('mixmilk.out', 'w') as fout:
    fout.write(str(m1) + '\n')
    fout.write(str(m2) + '\n')
    fout.write(str(m3))
        
        
        
        
        