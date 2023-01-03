// In Rust, every function(or every distinct instantiation of a generic function) has its own unique type. In particular, even two functions with the same function signature would have different types
// It's different from other languages when it needs to use function pointer in runtime and makes it not zero-overhead abstraction
fn d<T>(_f: T) {
    match std::mem::size_of::<T>() {
        0 => print!("0"),
        1 => print!("1"),
        _ => print!("2"),
    }
}

fn a<T>(f: fn(T)) {
    d(f);
}

// a::<u32> has a zero-sized type
// and it could be coerced into a function pointer
