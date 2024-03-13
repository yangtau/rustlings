// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    let mut lo = 0;
    let mut hi = 0;
    let mut not_space = false;
    for (i, c) in input.chars().enumerate() {
        if !not_space {
            if c != ' ' {
                not_space = true;
                lo = i;
            }
        } else {
            if c != ' ' {
                hi = i + 1;
            }
        }
    }

    String::from(&input[lo..hi])
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s = String::from(input);
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut s = String::new();
    for (i, c) in input.chars().enumerate() {
        s.push(c);
        if c == 's' && i >= 3 && &input[i - 3..i + 1] == "cars" {
            s.pop();
            s.pop();
            s.pop();
            s.pop();
            s.push_str("balloons");
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("     "), "");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
        assert_eq!(replace_me("cars"), "balloons");
        assert_eq!(replace_me("cars cars"), "balloons balloons");
    }
}
