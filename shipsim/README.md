This is the ships computer. It's a RISC-V 64imdb core. 

I'm still finalizing the specs, but it'll have something like 64K of ram
and each ship will have a given number of instructions they can execute
each simulation tick. There are some special function calls (basic trig
and vector utilities) that only incure the cost of a single cycle. That list is
still being finalized, but the emulator supports some trig functions right now.
