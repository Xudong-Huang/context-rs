extern crate context;

use context::Context;

fn main() {
    let mut context = Context::empty();
    Context::save(&mut context);
    println!("Modifing the flag ...");
    Context::load(&context);

    println!("Now it becomes true");
}
