use std::collections::HashMap;
use std::iter::Map;

pub fn properties(content: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    content.split("\n").for_each(|line| {
        let mut parts = line.splitn(2, "=");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        result.insert(key.to_string(), value.to_string());
    });
    result
}

#[cfg(test)]
mod test {
    use std::iter::Map;

    use crate::properties::*;

    #[test]
    fn test() {
        let target = properties("name=foo\nage=42");

        assert_eq!("foo", target["name"]);
        assert_eq!("42", target["age"]);
    }
}