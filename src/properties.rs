use std::collections::HashMap;
use std::iter::Map;

pub fn properties(content: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    content.split("\n").for_each(|line| {
        if !line.trim().is_empty() {
            let mut parts = line.splitn(2, "=");
            let key = parts.next().unwrap().to_string();
            let valueOpt = parts.next();
            match valueOpt {
                None => {}
                Some(value) => { result.insert(key, value.to_string()); }
            }
        }
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

    #[test]
    fn test_empty_lines() {
        let target = properties("\n\nname=foo\nage=42");

        assert_eq!("foo", target["name"]);
        assert_eq!("42", target["age"]);
    }

    #[test]
    fn test_eq_blank() {
        let target = properties("name=");
        assert_eq!("", target["name"]);

        let target = properties("\nname=\n");
        assert_eq!("", target["name"]);
    }

    #[test]
    fn test_without_equal() {
        let target = properties("name\n\n");
        assert!(!target.contains_key("name"));

        let target = properties("\nname\n");
        assert!(!target.contains_key("name"));
    }
}