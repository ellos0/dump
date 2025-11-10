#include <iostream>
using namespace std;

class Person {
  string name;
  int age;
  int adult;
};

void getSum(int a, int b) {
  int _sum = a + b;
  cout << "sum: " << _sum;
}

int main() {
  getSum(2, 10);

  return 0;
}
