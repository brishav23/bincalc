use std::alloc::{dealloc, Layout, alloc};
use std::ptr;
use libc::{sigaction, SA_SIGINFO, SA_RESTART, siginfo_t};

pub struct Sigaction {
    cstruct: *mut sigaction,
}

impl Sigaction {
    // takes a signal and handler as argument and calls sigaction to set handler
    pub fn new(sig: i32, f: fn(i32, siginfo_t, usize)) -> Option<Sigaction> {
        let layout = Layout::new::<sigaction>();
        unsafe {
            let rawptr: *mut u8 = alloc(layout);
            if rawptr.is_null() {
                None
            } else {
                let sa = Sigaction {
                    cstruct: rawptr as *mut sigaction,
                };
                (*sa.cstruct).sa_flags = SA_SIGINFO | SA_RESTART;
                (*sa.cstruct).sa_sigaction = (f as *const fn(i32, siginfo_t, usize)) as usize;
                sigaction(sig, sa.cstruct as *const sigaction, ptr::null_mut() as *mut sigaction);
                Some(sa)
            }
        }
    }
}

impl Drop for Sigaction {
    fn drop(&mut self) {
        let ptr: *mut u8 = self.cstruct as *mut u8;
        let layout = Layout::new::<sigaction>();
        unsafe {
            dealloc(ptr, layout);
        }
    }
}