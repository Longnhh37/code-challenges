#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  for (int _ = 0; _ < 10; _++) {
    int N;
    cin >> N;

    vector<int> spins(N);
    for (int &x : spins) {
      cin >> x;
    }

    vector<int> targets(5);
    for (int &x : targets) {
      cin >> x;
    }

    unordered_set<int> ij;
    for (int i : spins)
      for (int j : spins) {
        ij.insert(i + j);
        ij.insert(i * j);
      }

    unordered_set<int> hit;
    for (int t : targets)
      for (int k : spins) {
        if (ij.count(t - k))
          hit.insert(t);
        if ((t % k == 0) && ij.count(t / k))
          hit.insert(t);
      }

    for (int t : targets)
      cout << (hit.count(t) ? 'T' : 'F');
    cout << '\n';
  }
}
