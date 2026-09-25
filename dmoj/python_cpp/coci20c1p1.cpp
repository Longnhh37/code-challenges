#include <algorithm>
#include <iostream>
#include <optional>
#include <string>
#include <utility>
#include <vector>

using namespace std;

optional<int> dir_check(int i, int j, const vector<string> &grid) {
  int d = 0;

  while (true) {
    char cur = grid[i][j];
    if (cur == '.')
      break;

    d++;
    if (cur == '>')
      j++;
    else if (cur == '<')
      j--;
    else if (cur == '^')
      i--;
    else if (cur == 'v')
      i++;
    else
      return d; // cur = 'x'
  }
  return nullopt;
}

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  int r, s;
  cin >> r >> s;

  vector<string> grid(r);
  for (int i = 0; i < r; i++)
    cin >> grid[i];

  int x0 = -1, y0 = -1;
  for (int i = 0; i < r; i++) {
    auto pos = grid[i].find('o');
    if (pos != string::npos) {
      x0 = i;
      y0 = pos;
      break;
    }
  }

  constexpr int dx[4] = {-1, 1, 0, 0};
  constexpr int dy[4] = {0, 0, -1, 1};
  constexpr char dirChar[4] = {'N', 'S', 'W', 'E'};

  vector<pair<int, char>> out;

  for (int k = 0; k < 4; k++) {
    int nx = x0 + dx[k];
    int ny = y0 + dy[k];
    auto d = dir_check(nx, ny, grid);
    if (d.has_value()) {
      out.push_back({*d, dirChar[k]});
    }
  }

  if (out.empty()) {
    cout << ":(";
  } else {
    auto best = min_element(out.begin(), out.end());
    cout << ":)\n";
    cout << best->second;
  }

  return 0;
}
