trait MyTrait {
    fn f(&self);
}

impl MyTrait for u32 {
    fn f(&self) {
        println!("1")
    }
}

impl<'a> MyTrait for &'a i32 {
    fn f(&self) {
        println!("2")
    }
}

#[cfg(test)]
mod tests {
    use super::MyTrait;

    #[test]
    fn test_auto_ref() {
        let x = &0;
        x.f();
        let y = 0;
        y.f();
        let z = &&0;
        z.f()
        // what is stdout
        // 112
    }
}
