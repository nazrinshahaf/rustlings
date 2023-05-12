// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        // self.iter_mut().for_each(|s| {
        //     // s.push_str("Bar");
        // });
        // for str in &self {
        //     println!("{}", str);
        //     str.push_str("Bar");
        // }
        // self.iter_mut().map(|str| str.push_str("Bar"));
        self.push(String::from("Bar"));
        self
    }
}

// TODO: Implement trait `AppendBar` for a vector of strings.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
