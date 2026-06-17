/// Write a declarative macro `hashmap!` that creates a `HashMap` from a list of
/// key-value pairs.
///
/// Example:
///   let m = hashmap! { "a" => 1, "b" => 2 };
///
/// This is the kind of convenience macro that `macro_rules!` handles well —
/// no need for a procedural macro here.
///
/// ## Quick `macro_rules!` primer
///
/// A declarative macro uses pattern matching on tokens. Here's how `vec!` works:
///
/// ```rust
/// macro_rules! vec {
///     // Base case: no arguments → empty Vec
///     () => {
///         ::std::vec::Vec::new()
///     };
///     // Repeated elements: match a comma-separated list of expressions
///     // `$( ... ),*` means "zero or more repetitions separated by commas"
///     // `$elem:expr` captures each element as an expression
///     ( $( $elem:expr ),* $(,)? ) => {
///         {
///             let mut v = ::std::vec::Vec::new();
///             $( v.push($elem); )*  // repeat `push` for each captured $elem
///             v
///         }
///     };
/// }
/// ```
///
/// Your `hashmap!` macro needs to do the same thing, but with key-value pairs
/// separated by `=>` instead of single elements.
// TODO: fill in the body of the `hashmap` macro
macro_rules! hashmap {
    // Base case: no arguments → empty HashMap.
    // TODO: expand to a new, empty HashMap (look at how `vec!`'s base case works above).
    () => {
        todo!()
    };
    // Match comma-separated key => value pairs.
    // TODO: fill in the pattern and the body.
    //   - The pattern should capture $key:expr => $value:expr pairs,
    //     separated by commas (with an optional trailing comma).
    //   - The body should create a HashMap, insert each pair, and return it.
    // The surrounding `{ ... }` block is provided for you: a macro arm expands to a
    // single expression, so the statements (create, insert, return) must live inside a
    // block. Write your code where the `todo!()` is.
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{ todo!() }};
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let m: HashMap<&str, i32> = hashmap! {};
        assert!(m.is_empty());
    }

    #[test]
    fn single_entry() {
        let m: HashMap<&str, i32> = hashmap! { "a" => 1 };
        assert_eq!(m["a"], 1);
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn multiple_entries() {
        let m: HashMap<&str, i32> = hashmap! {
            "a" => 1,
            "b" => 2,
            "c" => 3,
        };
        assert_eq!(m.len(), 3);
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
        assert_eq!(m["c"], 3);
    }

    #[test]
    fn non_string_keys() {
        let m: HashMap<i32, &str> = hashmap! {
            1 => "one",
            2 => "two",
        };
        assert_eq!(m.len(), 2);
        assert_eq!(m[&1], "one");
        assert_eq!(m[&2], "two");
    }
}
