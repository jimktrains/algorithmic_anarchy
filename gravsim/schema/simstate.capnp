@0x9a9c8fb678dbe6d5;

struct SimState {
  time @0 :Float64;
  bodies @1 :List(SimBody);
  struct SimBody {
    spkid @0 :UInt64;
    x @1 :Float64;
    y @2 :Float64;
    z @3 :Float64;
  }
}
