use darling_exercise::Model;

// `#[darling(supports(struct_named))]` on `ModelOpts` rejects everything that
// isn't a named struct — here an enum — with a clear error, and the macro never
// has to `match` on `Data` by hand.
#[derive(Model)]
enum Shape {
    Circle,
    Square,
}

fn main() {}
