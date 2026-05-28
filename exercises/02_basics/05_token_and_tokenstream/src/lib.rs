use token_stream_macros::IdentCount;

#[derive(IdentCount)]
struct Unit;

#[derive(IdentCount)]
struct Pair {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_ident_count() {
        // `struct Unit` has 2 idents: the keyword "struct" and the name "Unit".
        assert_eq!(Unit::ident_count(), 2);
    }

    #[test]
    fn pair_ident_count() {
        // `struct Pair { x : i32 , y : i32 }` — at the top level, this is:
        //   Ident("struct"), Ident("Pair"), Group({ x : i32 , y : i32 })
        // You must recurse into the Group to count the idents inside it.
        // Full list: struct, Pair, x, i32, y, i32 → 6 idents.
        assert_eq!(Pair::ident_count(), 6);
    }
}
