pub mod rpc;

pub fn rpc_api_add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rpc_api_add(2, 2);
        assert_eq!(result, 4);
    }
}
