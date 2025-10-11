#include <stdio.h>
#include <stdlib.h>

int main() {
  int size = 12;
  const char* text = "hello world!";
  for (int i=0;i<size;i++) {
    printf("%c",text[i]);
  }
  return 0;
}

