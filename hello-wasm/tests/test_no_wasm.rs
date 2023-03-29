#[cfg(test)]
mod test {
    use hello_wasm::{get_string, greet};

    #[test]
    fn test_no_wasm() {
        assert_eq!(get_string(), "Hello, hello-wasm!")
    }
}