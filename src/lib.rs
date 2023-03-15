mod auto_ref;
mod bit_manipulation;
mod closure;
mod drop_order;
mod global_variable;
mod macro_hygiene;
mod while_let;

#[cfg(test)]
mod tests {
    #[test]
    fn float_operation() {
        assert!(0.1 + 0.2 > 0.3);
    }
}
