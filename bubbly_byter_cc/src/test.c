// Copyright JSK

#include "lib.h"

double v[3];

double *saved_doubles;

int main() {
  saved_doubles =  (double *)saved;
  double cc = saved_doubles[10];

  for (int i = 0; i < 10; i++) {
    cc = cos(cc);
    saved_doubles[11] = atan2(cc, cc) * 4;
  }

  saved_doubles[10] = cc;
  return 0;
}
