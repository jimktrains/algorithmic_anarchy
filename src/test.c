// Copyright JSK

#include "lib.h"

double v[] = {1.0, 2.0, 3.0};
#define BUF_LEN 128
char buf[128];
uint64_t *i = &savedi[0];

int main() {
  memset(buf, 0, BUF_LEN);
  double *t = &savedd[0];
  xx();
  v[0] = fcn(chrono.current);
  v[1] = keplarian_elements[KE_MERCURY].ra;
  xx();
  struct vec3 pos;
  k_pos(&pos, &keplarian_elements[KE_VENUS], *t);
  v[2] = pos.x;
  *t = pos.y;
  savedd[3] = v[1];
  savedd[4] = pos.z;

  debuglogf("Hello! %f\n", v[0]);

  x(v);
  return 0;
}
