// Convert a string to spinal case. Spinal case is all-lowercase-words-joined-by-dashes.

use regex::Regex;
#[allow(dead_code)]
#[allow(unused_variables)]
fn spinal_case(mut s: String) -> String {
    let re = Regex::new(r"(?P<lower>[a-z])(?P<upper>[A-Z])").unwrap();

    s=re.replace_all(&s, "${lower} ${upper}").to_string();

    let re = Regex::new(r"\s|_").unwrap();

    re.split(&s).collect::<Vec<&str>>().into_iter().map(|s|s.to_lowercase()).collect::<Vec<String>>().join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            spinal_case("This Is Spinal Tap".to_string()),
            "this-is-spinal-tap"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            spinal_case("thisIsSpinalTap".to_string()),
            "this-is-spinal-tap"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            spinal_case("The_Andy_Griffith_Show".to_string()),
            "the-andy-griffith-show"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            spinal_case("Teletubbies say Eh-oh".to_string()),
            "teletubbies-say-eh-oh"
        );
    }
}
