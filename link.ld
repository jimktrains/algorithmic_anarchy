OUTPUT_ARCH( "riscv" )
ENTRY( _start )
MEMORY
{
  ram   (wxa!ri) : ORIGIN = 0x200, LENGTH = 64K
  rom   (rx) : ORIGIN = 128M, LENGTH = 128M
  mmio  (rw) : ORIGIN = 1024M, LENGTH = 128M
}

SECTIONS
{
  /* text: test code section */
  . = 0x200;
  .text :
  {
    *(.init)
    *(.text)
 } > ram
 /* data: Initialized data segment */
 .data :
 {
   *(.data)
 } > ram
 /* End of uninitalized data segement */
 _end = .;
 __stack_top = ORIGIN(ram) + LENGTH(ram) - 1;

 .memmapio :
 {
   savedd = .; . += (128 * 8);
   savedi = .; . += (16 * 8);
   chrono = .; . += 1;

 } > mmio

 .systemprovided : {
   n_bodies_data = 512;
   keplarian_elements = .; . += (6 * (8 * n_bodies_data));
   masses             = .; . += (8 * n_bodies_data);
   diameter           = .; . += (8 * n_bodies_data);
   body_names         = .; . += (30 * n_bodies_data);

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

 } > rom

 }
