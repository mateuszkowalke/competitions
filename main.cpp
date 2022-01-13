#include <bits/stdc++.h>
#include <iostream>
#include <limits>
#include <vector>

int main() {
  using namespace std;
  int t;
  cin >> t;

  for (int i = 0; i < t; i++) {
    unsigned long long r = 0, k, n, p = 1;
    cin >> k >> n;
    k -= 1;

    while (p < n) {
      r += 1;
      k = k < p ? 0 : k - p;
      p *= 2;
    }

    r += k % n == 0 ? k / n : k / n + 1;

    cout << r << endl;
  }
}
