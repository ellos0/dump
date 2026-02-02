#include <stdio.h>

int main() {
  FILE* f = fopen("output.ppm", "wb");
  int w = 16*60;
  int h = 9*60;
  fprintf(f, "P6\n");
  fprintf(f, "%d %d\n", w, h);
  fprintf(f, "255\n");
  for (int y=0;y<h;++y) {
    for (int x=0;x<w;++x) {
      fputc(x/4,f);
      fputc(0x00,f);
      fputc(x/4,f);
    }
  }
  fclose(f);
  return 0;
}
