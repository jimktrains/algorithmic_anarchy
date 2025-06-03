use rvemu::exception::Exception;

use bubbly_byter::base_system::BubblyByter;

fn main() -> Result<(), Exception> {
    let mut sys = BubblyByter::new();
    sys.load_kernel("../bubbly_byter_cc/build/kernel.img");

    sys.execute(2000)
}
