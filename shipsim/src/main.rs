use rvemu::exception::Exception;

use shipsim::computers;
use shipsim::computers::Computer;

use shipsim::ships;

fn main() -> Result<(), Exception> {
    let ship = ships::new_basic();
    println!("{:#?}", ship);

    let mut sys = computers::new(computers::System::BubblyByter);
    sys.load_kernel("../bubbly_byter_cc/build/kernel.img");

    sys.execute(2000)
}
