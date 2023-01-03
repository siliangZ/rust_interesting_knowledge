fn call(mut f: impl FnMut() + Copy) {
    f()
}

fn g(mut f: impl FnMut() + Copy) {
    f();
    call(f);
    f();
}

#[cfg(test)]
mod tests {
    use super::g;

    #[test]
    fn closure_inner() {
        let mut i = 0i32;
        g(move || {
            i += 1;
            println!("{}", i);
        })
        // what is the stdoutput?
        // 122
    }
}
