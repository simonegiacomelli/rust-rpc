mod rpc;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rpc::add(2, 2);
        assert_eq!(result, 4);
    }
}
