a, b, c, d, s = [int(input()) for _ in range(5)]

nikky = byron = 0

n_steps = s
while True:
    if n_steps > a:
        nikky += a
        n_steps -= a
    else:
        nikky += n_steps
        break

    if n_steps > b:
        nikky -= b
        n_steps -= b
    else:
        nikky -= n_steps
        break

b_steps = s
while True:
    if b_steps > c:
        byron += c
        b_steps -= c
    else:
        byron += b_steps
        break

    if b_steps > d:
        byron -= d
        b_steps -= d
    else:
        byron -= b_steps
        break


if nikky > byron:
    print("Nikky")
elif nikky < byron:
    print("Byron")
else:
    print("Tied")
