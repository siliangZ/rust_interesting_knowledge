fn while_let_ref() {
    let s1 = String::from("hello world");
    let mut o1 = Some(&s1);
    while let Some(bind) = o1.as_ref() {
        println!("{bind}");
        o1 = None;
    }
}

#[cfg(test)]
mod tests {
    use super::while_let_ref;

    #[test]
    fn test_while_let() {
        while_let_ref();
    }
}
