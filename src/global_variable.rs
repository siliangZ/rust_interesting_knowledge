static mut SHUT_DOWN: bool = false;

fn access_global() {
    // access global variable is unsafe
    unsafe {
        SHUT_DOWN = true;
    }
}
