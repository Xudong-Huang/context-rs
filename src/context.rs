// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use detail::{Registers, initialize_call_frame, swap_registers, load_registers, save_registers};

#[derive(Debug)]
pub struct Context {
    /// Hold the registers while the task or scheduler is suspended
    regs: Registers,
}

pub type InitFn = fn(usize, *mut usize); // first argument is task handle, second is thunk ptr

impl Context {
    pub fn empty() -> Context {
        Context {
            regs: Registers::new(),
        }
    }

    /// Create a new context that will resume execution by running start
    ///
    /// The `init` function will be run with `arg` and the `start` procedure
    /// split up into code and env pointers. It is required that the `init`
    /// function never return.
    ///
    /// FIXME: this is basically an awful the interface. The main reason for
    ///        this is to reduce the number of allocations made when a green
    ///        task is spawned as much as possible
    pub fn new(init: InitFn, arg: usize, start: *mut usize, stack: *const usize) -> Context {
        let mut ctx = Context::empty();
        ctx.init_with(init, arg, start, stack);
        ctx
    }

    pub fn init_with(&mut self, init: InitFn, arg: usize, start: *mut usize, stack: *const usize) {
        let sp: *mut usize = stack as *mut usize;
        // Save and then immediately load the current context,
        // which we will then modify to call the given function when restoredtack
        initialize_call_frame(&mut self.regs, init, arg, start, sp);
    }

    /// Switch contexts

    /// Suspend the current execution context and resume another by
    /// saving the registers values of the executing thread to a Context
    /// then loading the registers from a previously saved Context.
    pub fn swap(out_context: &mut Context, in_context: &Context) {
        debug!("swapping contexts");
        let out_regs: &mut Registers = match out_context {
            &mut Context { regs: ref mut r, .. } => r
        };
        let in_regs: &Registers = match in_context {
            &Context { regs: ref r, .. } => r
        };

        debug!("register raw swap");

        unsafe {
            swap_registers(out_regs, in_regs)
        }
    }

    /// Save the current context.
    #[inline(always)]
    pub fn save(context: &mut Context) {
        let regs: &mut Registers = &mut context.regs;

        unsafe {
            save_registers(regs);
        }
    }

    /// Load the context and switch. This function will never return.
    ///
    /// It is equivalent to `Context::swap(&mut dummy_context, &to_context)`.
    #[inline(always)]
    pub fn load(to_context: &Context) {
        let regs: &Registers = &to_context.regs;

        unsafe {
            load_registers(regs);
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::transmute;

    use stack::Stack;
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

    #[test]
    fn test_swap_context() {
        static mut VAL: bool = false;
        let mut cur = Context::empty();

        fn callback() {
            unsafe { VAL = true; }
        }

        let stk = Stack::new(MIN_STACK);
        let ctx = Context::new(init_fn, unsafe { transmute(&cur) }, unsafe { transmute(callback) }, stk.end());

        Context::swap(&mut cur, &ctx);
        unsafe { assert!(VAL); }
    }

    #[test]
    fn test_load_save_context() {
        static mut VAL: bool = false;
        let mut cur = Context::empty();

        fn callback() {
            unsafe { VAL = true; }
        }

        let stk = Stack::new(MIN_STACK);
        let ctx = Context::new(init_fn, unsafe { transmute(&cur) }, unsafe { transmute(callback) }, stk.end());

        Context::save(&mut cur);
        Context::load(&ctx);
        unsafe { assert!(VAL); }
    }
}
