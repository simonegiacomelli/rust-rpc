use std::collections::HashMap;
use std::iter::Map;

pub fn properties(content: &str) -> HashMap<String, String> {

    fn process_line(line: &str) -> Option<(String, String)> {
        let tup: Option<(String, String)> = if line.trim().is_empty() {
            None
        } else {
            let mut parts = line.splitn(2, "=");
            let key = parts.next().unwrap().to_string();
            let valueOpt = parts.next();
            match valueOpt {
                None => { None }
                Some(value) => { Some((key, value.to_string())) }
            }
        };
        return tup;
    }

    content.replace("\r\n", "\n").split("\n").filter_map(|line| {
        process_line(line)
    }).into_iter().collect()
}

#[cfg(test)]
mod test {
    use std::iter::Map;

    use crate::properties::*;

    fn fix(string: &str, sep: &str) -> String {
        string.replace("\t", sep)
    }

    #[test]
    fn test() {
        fn rt(sep: &str) {
            let target = properties(&fix("name=foo\tage=42", sep));

            assert_eq!("foo", target["name"]);
            assert_eq!("42", target["age"]);
        }
        rt("\n");
        // rt("\r\n");
    }

    #[test]
    fn test_empty_lines() {
        fn rt(sep: &str) {
            let target = properties(&fix("\t\tname=foo\tage=42", sep));

            assert_eq!("foo", target["name"]);
            assert_eq!("42", target["age"]);
        }
        rt("\n");
        rt("\r\n");
    }

    #[test]
    fn test_eq_blank() {
        fn rt(sep: &str) {
            let target = properties(&fix("name=", sep));
            assert_eq!("", target["name"]);

            let target = properties(&fix("\tname=\t", sep));
            assert_eq!("", target["name"]);
        }
        rt("\n");
        rt("\r\n");
    }

    #[test]
    fn test_without_equal() {
        fn rt(sep: &str) {
            let target = properties(&fix("name\t\t", sep));
            assert!(!target.contains_key("name"));

            let target = properties(&fix("\tname\t", sep));
            assert!(!target.contains_key("name"));
        }
        rt("\n");
        rt("\r\n");
    }
}