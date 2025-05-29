// Copyright JSK

#pragma once

#include <stdint.h>

typedef uint64_t size_t;

struct __attribute__((packed, aligned(8))) vec3 {
  double x;
  double y;
  double z;
};

typedef struct vec3 vec3_t;

struct __attribute__((packed, aligned(8))) keplarian_elements {
  double e;
  double a;
  double b;
  double p;
  double ra;
  double rp;
};

extern struct keplarian_elements keplarian_elements[10];

extern struct __attribute__((packed, aligned(8))) {
  double current;
} chrono;

extern double savedd[128];
extern uint64_t savedi[16];

extern double fcn(double d);
extern void x(double *v);
extern void xx();
extern void k_pos(vec3_t *res, struct keplarian_elements *k, double t);
extern int snprintf(char *buffer, size_t bufsz, const char *format, ...);
extern void debuglog(char *, size_t bufsz);
extern void debuglogf(const char *format, ...);
extern void *memset(void *dest, int ch, size_t count);


extern int64_t lfabs(int64_t);
extern double  fabs(double);
extern double  fmax(double, double);
extern double  fmin(double, double);
extern double  exp(double);
extern double  expm1(double);
extern double  log(double);
extern double  log1p(double);
extern double  sqrt(double);
extern double  cbrt(double);
extern double  cube(double);
extern double  hypot2(double, double);
extern double  ceil(double);
extern double  floor(double);
extern double  round(double);
extern double  pow(double, double);
extern int64_t modpow(int64_t i, int64_t p, int64_t m);
extern void    divmod(int64_t *q, int64_t *r, int64_t i, int64_t m);
extern double  sin(double);
extern double  cos(double);
extern double  tan(double);
extern double  asin(double);
extern double  acos(double);
extern double  atan(double);
extern double  atan2(double, double);
extern double  sinh(double);
extern double  cosh(double);
extern double  tanh(double);
extern double  asinh(double);
extern double  acosh(double);
extern double  atanh(double);
// erf
// erfc
// quadraticroots
// evalpoly
// kepler
//
extern double dist33(double *a, double *b);
extern void add33(double *res, double *a, double *b);
extern void sub33(double *res, double *a, double *b);
extern void muls3(double *res, double a, double *b);
extern void divs3(double *res, double a, double *b);
extern void dot33(double *res, double *a, double *b);
extern void cross33(double *res, double *a, double *b);
extern double norm2(double *a, double *b);
extern void normalize2(double *a);
extern double norm3(double *a, double *b);
extern void normalize3(double *a);
extern void lerp33s(double *res, double *a, double *b, double t);

extern double vdist33(vec3_t *res, vec3_t *a, vec3_t *b);
extern void vadd33(vec3_t *res, vec3_t *a, vec3_t *b);
extern void vsub33(vec3_t *res, vec3_t *a, vec3_t *b);
extern void vmuls3(vec3_t *res, double a, vec3_t *b);
extern void vdivs3(vec3_t *res, double a, vec3_t *b);
extern void vdot33(vec3_t *res, vec3_t *a, vec3_t *b);
extern void vcross33(vec3_t *res, vec3_t *a, vec3_t *b);
extern double vnorm3(vec3_t *a, vec3_t *b);
extern void vnormalize3(vec3_t *a);
extern void vlerp33s(vec3_t *res, vec3_t *a, vec3_t *b, double t);

#define M_E           2.71828182845904523536
#define M_LN2         0.69314718055994530941
#define M_LN10        2.30258509299404568401
#define M_SQRT2       1.41421356237309504880
#define M_SQRT1_2     0.70710678118654752440
#define M_SQRT3       1.73205080756887729352
#define M_PI          3.14159265358979323846
#define M_TAU         6.28318530717958647692
#define M_PI_2        1.57079632679489661922
#define M_PI_4        0.78539816339744830961

#define M_C           299792458  // m s^−1

#define MU_G0         6.67430e-11  // m^3 kg^−1 s^−2

#define M_AU          1.495978707e11  // m

#define KE_SUN                      0
#define KE_MERCURY                  1
#define KE_VENUS                    2
#define KE_EARTH_MOON_BARYCENTER    3
#define KE_MARS                     4
#define KE_JUPITER                  5
#define KE_SATURN                   6
#define KE_URANUS                   7
#define KE_NEPTUNE                  8
// ... for the moons https://ssd.jpl.nasa.gov/sats/elem/
// ... and the small bodies https://ssd.jpl.nasa.gov/sb/elem_tables.html
