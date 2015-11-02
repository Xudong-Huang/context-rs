extern crate context;
use std::mem::transmute;                                               

use context::Stack;
use context::Context;

const MIN_STACK: usize = 2 * 1024 * 1024;

fn init_fn(arg: usize, f: *mut usize) {
    let func: fn() = unsafe {
        transmute(f)
    };
    func();

    let ctx: &Context = unsafe { transmute(arg) };
    Context::load(ctx);
}

fn main() {
    let mut cur = Context::empty();

    fn callback() {
        println!("asdfasdf");
    }

    let stk = Stack::new(MIN_STACK);
    let ctx = Context::new(init_fn, unsafe { transmute(&cur)  }, unsafe { transmute(callback)  }, stk.end()); 

    let mut _no_use = Box::new(true);

    Context::save(&mut cur);
    if *_no_use {
        *_no_use = false;
        Context::load(&ctx);
    }
}
