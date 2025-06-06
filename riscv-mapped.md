The game runs a cycle by first computing the movement of every object
[viz](https://patrickyoussef.com/blog/nbody/).

Then, using a modified version of
[rvemu](https://github.com/d0iasm/rvemu) to run a set number of
instructions. Map spacecraft systems into the memory space.  Map common
functions into the address space, and (maybe?) handle them special in
the emulator such that they incure a reduced instruction count.

The other option is having all of the functions defined via macros and
inline assembly to be an ecall, but handling them as special cases of jal,
j, and jalr seems to be easier and preserves some type checking.

In addition to memory-mapped io, there is a section of memory that will
be saved and reloaded each run. All other memory and registeres are
reset.

The game can be interacted with via a shell accessible via ssh. New
programs can be uploaded via sftp.
