#![feature(fnbox)]

extern crate context;
use std::mem;
use std::boxed::FnBox;

use context::{Context, Stack};

const STACK_SIZE: usize = 2 * 1024 * 1024; // 2MB

fn init_fn(arg: usize, f: *mut usize) {
    // Transmute it back to the Box<Box<FnBox()>>
    {
        let func: Box<Box<FnBox()>> = unsafe {
            Box::from_raw(f as *mut Box<FnBox()>)
        };

        // Call it
        func();

        // The `func` must be destroyed here,
        // or it will cause memory leak.
    }

    // The argument is the context of the main function
    let ctx: &Context = unsafe { mem::transmute(arg) };

    // Switch back to the main function and will never comeback here
    Context::load(ctx);
}

fn main() {
    // Initialize an empty context
    let mut cur = Context::empty();

    let callback: Box<FnBox()> = Box::new(move|| {
        let a = 10;
        let b = 2;
        println!("{}, {} Inside your function!", a, a+b);
    });

    let stk = Stack::new(STACK_SIZE);
    let ctx = Context::new(init_fn, unsafe { mem::transmute(&cur) },
                           Box::into_raw(Box::new(callback)) as *mut usize, stk.end());

    println!("Before switch");

    // Switch!
    Context::swap(&mut cur, &ctx);

    println!("Back to main function");
}
