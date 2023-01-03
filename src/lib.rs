mod auto_ref;
mod bit_manipulation;
mod closure;
mod drop_order;
mod macro_hygiene;

#[cfg(test)]
mod tests {
    #[test]
    fn float_operation() {
        assert!(0.1 + 0.2 > 0.3);
    }
}
