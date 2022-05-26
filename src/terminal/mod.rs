use libc::{termios, tcgetattr, cfmakeraw, tcsetattr, TCSANOW, VINTR, ISIG};
use std::alloc::{alloc, dealloc, Layout};

pub struct Termios {
    pub cstruct: *mut termios,
}

impl Termios {
    pub fn new() -> Option<Termios> {
        unsafe {
            let layout = Layout::new::<termios>();
            let ptr: *mut u8 = alloc(layout);
            if ptr.is_null() {
                None
            } else {
                Some(Termios {
                    cstruct: ptr as *mut termios,
                })
            }
        }
    }

    pub fn backup_tty(&mut self) {
        unsafe {
            let res: i32 = tcgetattr(0i32, self.cstruct);
            if res == -1 {
                panic!("Problem getting stdin tcattr");
            }
        }
    }

    // unsafe because pointer is stored as mutable static variable
    // so it can be used from the signal handler,
    // therefore restoring it in the signal handler is unsafe
    // because it is using a mutable static variable
    pub unsafe fn restore_tty(t: *const termios) {
        tcsetattr(0i32, TCSANOW, t);
    }

    pub fn set_raw() {
        let term: Termios = Termios::new().unwrap();
        unsafe {
            // cfmakeraw(&mut *term.cstruct as *mut termios);
            cfmakeraw(term.cstruct);
            (*term.cstruct).c_cc[VINTR] = 0x3;
            (*term.cstruct).c_lflag = ISIG;
            tcsetattr(0i32, TCSANOW, term.cstruct as *const termios);
        }
    }
}

impl Drop for Termios {
    fn drop(&mut self) {
        let ptr: *mut u8 = self.cstruct as *mut u8;
        let layout = Layout::new::<termios>();
        unsafe {
            dealloc(ptr, layout);
        }
    }
}
