OUTPUT_ARCH( "riscv" )
ENTRY( _start )
MEMORY
{
  ram   (wxa!ri) : ORIGIN = 1024M, LENGTH = 64K
  rom   (rx) : ORIGIN = 0x80000400, LENGTH = 256M
  mmio  (rw) : ORIGIN = ORIGIN(rom) + LENGTH(rom), LENGTH = 256M
  nvram (rw) : ORIGIN = ORIGIN(mmio) + LENGTH(mmio), LENGTH = 256M
}

SECTIONS
{
  /* text: test code section */
  . = 0x200;
  .text :
  {
    *(.init)
    *(.text)
    *(.final)
 } > ram
 /* data: Initialized data segment */
 .data :
 {
   *(.data)
 } > ram
 /* End of uninitalized data segement */
 _end = .;

 .memmapio :
 {
   chrono = .; . += 8;
 } > mmio

 .nvram :
 {
   saved = .; . += 64K;
 } > nvram

 .systemprovided : {

   fcn   = .; . += 8;
   xx    = .; . += 8;
   k_pos = .; . += 8;
   x     = .; . += 8;
   snprintf  = .; . += 8;
   debuglog  = .; . += 8;
   debuglogf = .; . += 8;
   memset    = -.; . += 8;

    lfabs = .; . += 8;
    fabs = .; . += 8;
    fmax = .; . += 8;
    fmin = .; . += 8;
    exp = .; . += 8;
    expm1 = .; . += 8;
    log = .; . += 8;
    log1p = .; . += 8;
    sqrt = .; . += 8;
    cbrt = .; . += 8;
    cube = .; . += 8;
    hypot2 = .; . += 8;
    ceil = .; . += 8;
    floor = .; . += 8;
    round = .; . += 8;
    pow = .; . += 8;
    modpow = .; . += 8;
    divmod = .; . += 8;
    sin = .; . += 8;
    cos = .; . += 8;
    tan = .; . += 8;
    asin = .; . += 8;
    acos = .; . += 8;
    atan = .; . += 8;
    atan2 = .; . += 8;
    sinh = .; . += 8;
    cosh = .; . += 8;
    tanh = .; . += 8;
    asinh = .; . += 8;
    acosh = .; . += 8;
    atanh = .; . += 8;

    dist33 = .; . += 8;
    add33 = .; . += 8;
    sub33 = .; . += 8;
    muls3 = .; . += 8;
    divs3 = .; . += 8;
    dot33 = .; . += 8;
    cross33 = .; . += 8;
    norm2 = .; . += 8;
    normalize2 = .; . += 8;
    norm3 = .; . += 8;
    normalize3 = .; . += 8;
    lerp33s = .; . += 8;

    vdist33 = .; . += 8;
    vadd33 = .; . += 8;
    vsub33 = .; . += 8;
    vmuls3 = .; . += 8;
    vdivs3 = .; . += 8;
    vdot33 = .; . += 8;
    vcross33 = .; . += 8;
    vnorm3 = .; . += 8;
    vnormalize3 = .; . += 8;
    vlerp33s = .; . += 8;

   n_bodies_count = 512;

   celestials = .;
   . += n_bodies_count * (
     (6 * 8) + /* keplarian elements */
     8 + /* mass */
     8 + /* diameter */
     32 /* name */
   );
 } > rom

 }
