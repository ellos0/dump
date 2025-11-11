#include <iostream>
using namespace std;

namespace my_space {
  int add(int a, int b) {
    return a+b;
  }
}

int main() {
  cout << my_space::add(1,2) << endl;
}
