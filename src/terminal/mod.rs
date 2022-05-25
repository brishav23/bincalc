use libc::{termios, tcgetattr, cfmakeraw, tcsetattr, TCSANOW};
use std::alloc::{alloc, dealloc, Layout};

pub struct Termios {
    cstruct: *mut termios,
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

    pub fn restore_tty(&self) {
        unsafe {
            tcsetattr(0i32, TCSANOW, &*self.cstruct as *const termios);
        }
    }

    pub fn set_raw() {
        let term: Termios = Termios::new().unwrap();
        unsafe {
            cfmakeraw(&mut *term.cstruct as *mut termios);
            tcsetattr(0i32, TCSANOW, &*term.cstruct as *const termios);
        }
    }
}

impl Drop for Termios {
    fn drop(&mut self) {
        let ptr: *mut u8 = (self.cstruct as *mut termios) as *mut u8;
        let layout = Layout::new::<termios>();
        unsafe {
            dealloc(ptr, layout);
        }
    }
}
