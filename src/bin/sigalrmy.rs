// This will either complete without error (overflowing the signal stack!!), or with a signal stack
// guard page, crash with a segmentation fault.

use std::mem;
use std::ptr;
use libc::{sigaction, sighandler_t, SA_ONSTACK, SA_SIGINFO, SIGSTKSZ};
use libc::{MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE, SIGSEGV, SIGALRM};
use libc::mmap;

unsafe extern "C" fn stacky_handler(
    signum: libc::c_int,
    _info: *mut libc::siginfo_t,
    _data: *mut libc::c_void,
) {
    let mut padding = [0u8; 65535]; // 64k is enough for anybody, right?
    for i in padding.iter_mut() {
        *i = signum as u8; // write some unknown value so this array doesn't get trivially deleted
    }
}

fn main() {
    unsafe {
        println!("signal stack size is {}", SIGSTKSZ);

        // do a lot of small mmap so we likely allocate a read/write page immediately after
        // libstd's signal stack
        for _ in 0..1024 {
            mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0);
        }

        let mut action: sigaction = mem::zeroed();
        action.sa_flags = SA_SIGINFO | SA_ONSTACK;
        action.sa_sigaction = stacky_handler as sighandler_t;

        sigaction(SIGALRM, &action, ptr::null_mut());
        println!("handler installed... and away we go!");
        libc::kill(libc::getpid(), SIGALRM);
    }
}
