struct X(i32);
macro_rules! x {
    ($n:expr) => {
        let a = X($n);
    };
}

impl Drop for X {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::X;

    #[test]
    fn test_macro() {
        let a = X(0);
        x!(2);
        println!("{}", a.0);
        // what is the stdout?
        // 020
        // 0 => println!
        // 2 => Drop(a in macro)
        // 0 => Drop(a on line 20)
    }
}
