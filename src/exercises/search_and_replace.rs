// Perform a search and replace on the sentence using the arguments provided and return the new sentence.

// First argument is the sentence to perform the search and replace on.

// Second argument is the word that you will be replacing (before).

// Third argument is what you will be replacing the second argument with (after).

// Note: Preserve the case of the first character in the original word when you are replacing it.
// For example if you mean to replace the word Book with the word dog, it should be replaced as Dog
use regex::Regex;

#[allow(dead_code)]
#[allow(unused_variables)]
fn my_replace(s: &str, before: &str, after: &str) -> String {
    let re = Regex::new(r"^[A-Z]").unwrap();
    let mut after = after.to_string();
    if re.is_match(before){
       after = first_to_upper(&after);
    }else if !re.is_match(before)&&re.is_match(&after) {
        after = after.to_lowercase();
    }
    s.replace(before, &after)
}

fn first_to_upper(s:&str)->String{
     let mut c = s.chars();

     match c.next() {
         None => String::new(),
         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
     }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            my_replace("Let us go to the store", "store", "mall"),
            "Let us go to the mall"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            my_replace("He is Sleeping on the couch", "Sleeping", "sitting"),
            "He is Sitting on the couch"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            my_replace("I think we should look up there", "up", "Down"),
            "I think we should look down there"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            my_replace("This has a spellngi error", "spellngi", "spelling"),
            "This has a spelling error"
        );
    }
}
