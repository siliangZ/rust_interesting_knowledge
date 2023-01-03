struct S;

impl Drop for S {
    fn drop(&mut self) {
        println!("1")
    }
}

struct Student {
    name: String,
}

impl Drop for Student {
    fn drop(&mut self) {
        println!("1")
    }
}

#[cfg(test)]
mod tests {
    use crate::drop_order::S;

    use super::Student;

    #[test]
    fn test_drop() {
        let s = S;
        let _ = s; // is s moved here?
        println!("2");
        // what is the stdout
        // 21
    }

    #[test]
    fn test_drop_2() {
        let s = Student {
            name: "a1".to_string(),
        };
        let _ = s;
        println!("2");
        // what is the stdoout?
        // 21
    }
}
