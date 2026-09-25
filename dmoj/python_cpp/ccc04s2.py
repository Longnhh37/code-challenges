N, K = map(int, input().split())

score = [0] * N
rank = [0] * N

for _ in range(K):
    s = list(map(int, input().split()))
    for i in range(N):
        score[i] += s[i]

    sorted_score = sorted(enumerate(score), key=lambda x: -x[1])

    cur_score = sorted_score[0][1]
    cur_rank = 1
    for i, v in sorted_score:
        if v == cur_score:
            if cur_rank > rank[i]:
                rank[i] = cur_rank
        else:  # v < cur_score
            cur_rank += 1
            if cur_rank > rank[i]:
                rank[i] = cur_rank


max_score = max(score)
for i in range(N):
    if score[i] == max_score:
        print(
            f"Yodeller {i + 1} is the TopYodeller: score {max_score}, worst rank {rank[i]}"
        )
