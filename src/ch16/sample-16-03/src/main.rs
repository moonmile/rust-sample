use std::os::raw::c_char;
extern "C" {
    fn puts(s: *const c_char);
    fn strlen(s: *const c_char) -> usize;
}

use std::ffi::CString;
fn main() {
    let s = "hello rust world.";
    let s_null_terminated = CString::new(s).unwrap();
    unsafe {
        puts(s_null_terminated.as_ptr());
    }

    let n = unsafe { strlen(s_null_terminated.as_ptr()) };
    println!("s.len is {}", n);
}
