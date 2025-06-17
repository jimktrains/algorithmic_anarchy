@0x9a9c8fb678dbe6d5;

struct SimState {
  time @0 :Float64;
  bodies @1 :List(SimBody);
  struct SimBody {
    x @0 :Float64;
    y @1 :Float64;
    z @2 :Float64;
  }
}

struct PhysicalConstants {
  bodies @0 :List(PhysicalConstant);
  struct PhysicalConstant {
    spkid @0: UInt64;
    name @1 : Text;
    mass @2: Float64;
    radius @3 :Float64;
  }
}
