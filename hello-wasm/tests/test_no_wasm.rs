#[cfg(test)]
mod test {
    use hello_wasm::{get_string, greet};
    use rpc_api::rpc_api_add;

    #[test]
    fn it_works() {
        let result = rpc_api_add(2, 2);
        assert_eq!(result, 4);
    }
}