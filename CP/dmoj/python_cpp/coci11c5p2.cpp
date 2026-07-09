#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

int main() {
  int N, M;
  cin >> N >> M;

  vector<int> a(N);
  for (int i = 0; i < N; i++) {
    cin >> a[i];
  }
  sort(a.begin(), a.end(), greater<int>());

  int h_diff_prev = 0;
  int ans;

  for (int i = 1; i < N; i++) {
    int h_diff = 0;
    for (int j = 0; j < i; j++) {
      h_diff += a[j] - a[i];
    }

    if (h_diff > M) {
      ans = a[i - 1] - (M - h_diff_prev + i - 1) / i;
      break;
    } else if (h_diff == M) {
      ans = a[i];
      break;
    }

    h_diff_prev = h_diff;
  }

  cout << ans;
  return 0;
}
