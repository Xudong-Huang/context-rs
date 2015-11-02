extern crate context;

use context::Context;

fn main() {

    let mut flag = Box::new(false);

    let mut context = Context::empty();
    Context::save(&mut context);

    if !*flag {
        *flag = true;
        println!("Modifing the flag ...");
        Context::load(&context);
    }

    println!("Now it becomes true");
}
