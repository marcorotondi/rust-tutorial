fn to_camel_case(text: &str) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = false;
    text.split("-|_").for_each(|word| {
        for c in word.chars() {
            if c == '-' || c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                camel_case.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                camel_case.push(c);
            }
        }
    });

    camel_case
}

fn main() {
    println!("using only test!")
}


#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("", "");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}