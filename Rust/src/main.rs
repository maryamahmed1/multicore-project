mod heat;
mod matrix_multiplier;
mod programmability;
// mod racecondition;

use heat::main as heat_main;
use matrix_multiplier::main as mat_mul_main;
use programmability::main as prog_main;
// use racecondition::main as rc_main;

fn main() {
    println!("max threads  =  {}", rayon::current_num_threads());
    // For mat mult
    // mat_mul_main();

    // For heat
    // heat_main();

    // For race condition example
    // rc_main();

    // For programmability example
    prog_main();
}
