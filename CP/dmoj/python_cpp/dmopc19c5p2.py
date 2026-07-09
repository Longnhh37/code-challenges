def solve():
    """
    Game đấu Charlie vs Bot (turn-based)

    Turn order: Charlie turn 1, Bot turn 1, Charlie turn 2, Bot turn 2, ...

    Dodge mechanics:
    - Khi player dodge ở turn T:
      + Nếu opponent attack ở turn T+1 → dodge SUCCESS → player không nhận damage, opponent cũng KHÔNG deal damage
      + Nếu opponent KHÔNG attack ở turn T+1 → dodge FAIL → player nhận damage (self-humility)

    Edge case: Dodge ở move cuối cùng = failed dodge (không có turn sau)
    """
    n, h = map(int, input().split())

    # Đọc input
    charlie_moves = []
    for _ in range(n):
        action, damage = input().split()
        charlie_moves.append((action, int(damage)))

    bot_moves = []
    for _ in range(n):
        action, damage = input().split()
        bot_moves.append((action, int(damage)))

    # Tạo turn sequence: C1, B1, C2, B2, ..., Cn, Bn
    turns = []
    for i in range(n):
        turns.append(
            (0, charlie_moves[i][0], charlie_moves[i][1])
        )  # player=0 (Charlie)
        turns.append((1, bot_moves[i][0], bot_moves[i][1]))  # player=1 (Bot)

    health = [h, h]  # [Charlie, Bot]
    pending_dodge = [None, None]  # pending_dodge[i] = damage nếu player i vừa dodge

    for turn_idx in range(len(turns)):
        player, action, damage = turns[turn_idx]
        opponent = 1 - player

        # Bước 1: Check pending dodge của đối thủ từ lượt trước
        if pending_dodge[opponent] is not None:
            dodge_damage = pending_dodge[opponent]

            if action == "A":
                # Mình attack nhưng đối thủ đã dodge → dodge thành công
                # Đối thủ KHÔNG mất máu
                # Mình KHÔNG deal damage
                pending_dodge[opponent] = None
                continue  # Skip attack damage
            else:
                # Mình KHÔNG attack → đối thủ dodge thất bại (self-humility)
                health[opponent] -= dodge_damage
                if health[opponent] <= 0:
                    print("VICTORY" if opponent == 1 else "DEFEAT")
                    return
                pending_dodge[opponent] = None

        # Bước 2: Xử lý action hiện tại
        if action == "A":
            # Attack đối thủ (nếu đến đây nghĩa là đối thủ không dodge)
            health[opponent] -= damage
            if health[opponent] <= 0:
                print("VICTORY" if opponent == 1 else "DEFEAT")
                return
        else:  # action == 'D'
            # Dodge: lưu pending cho lượt sau
            pending_dodge[player] = damage

    # Bước 3: Edge case - xử lý pending dodge còn lại
    for player in range(2):
        if pending_dodge[player] is not None:
            # Dodge ở move cuối → failed dodge
            health[player] -= pending_dodge[player]
            if health[player] <= 0:
                print("DEFEAT" if player == 0 else "VICTORY")
                return

    print("TIE")


if __name__ == "__main__":
    solve()


def solve2():
    n, h = map(int, (input().split()))

    c_moves = []  # charlie moves
    for _ in range(n):
        a, d = input().split()
        c_moves.append((a, int(d)))

    b_moves = []  # bot moves
    for _ in range(n):
        a, d = input().split()
        b_moves.append((a, int(d)))

    ch = bh = h  # charlie, bot health

    i = 0
    while i < n and ch > 0 and bh > 0:
        c_act = c_moves[i][0]
        cd = c_moves[i][1]
        try:
            c_next_act = c_moves[i + 1][0]
        except IndexError:
            c_next_act = None

        b_act = b_moves[i][0]
        b_prev_act = b_moves[i - 1][0]
        bd = b_moves[i][1]

        if c_act == "A" and (i == 0 or b_prev_act == "A"):
            bh -= cd
        elif c_act == "D" and b_act == "D":
            ch -= cd

        if ch > 0 and bh > 0:  # check health
            if b_act == "A" and c_act == "A":
                ch -= bd

            elif b_act == "D" and (i == n - 1 or c_next_act == "D"):
                bh -= bd

        i += 1

    if ch <= 0:
        print("DEFEAT")
    elif bh <= 0:
        print("VICTORY")
    else:
        print("TIE")
