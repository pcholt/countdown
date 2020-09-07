use std::env::args;

fn main() {
    let a = match args().next() {
        None => {"abcdefg".to_string()},
        Some(word) => {word},
    };
    let words = words::match_words(a.as_str());
    let mut words = words.clone();
    words.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    for word in words.iter().take(5) {
        println!("word = {}", word);
    }
}

mod words {
    use std::fs;

    pub fn match_words(jumble: &str) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for i in fs::read_to_string("/usr/share/dict/words")
            .expect("Dictionary missing")
            .trim()
            .split("\n")
            .filter(|letter| contained_within(letter, jumble))
        {
            v.push(i.to_string());
        }
        return v;
    }

    pub fn contained_within(candidate: &str, jumble: &str) -> bool {

        // bitmap reference to jumble
        let mut found = [false; 1024];

        // loop through all chars in candidate
        for letter in candidate.chars() {
            let mut isfound = false;
            for (i, jumble_char_allowed) in jumble.chars().enumerate() {
                if !found[i] && letter == jumble_char_allowed {
                    found[i] = true;
                    isfound = true;
                    break;
                }
            }
            if !isfound {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::words;

    #[test]
    fn simple_containment_check() {
        assert!(words::contained_within("bbba", "bbbbbba"));
        assert!(words::contained_within("", "bbbbbba"));
        assert!(words::contained_within("www", "wwww"));
        assert!(!words::contained_within("wwww", "www"));
    }
}