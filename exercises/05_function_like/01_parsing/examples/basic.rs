use parse_input_exercise::avg;
use pretty_assertions::assert_eq;

fn main() {
    // Plain literals.
    let a = avg!(2.0, 4.0, 6.0);
    assert_eq!(a, 4.0);

    // A single argument averages to itself.
    let single = avg!(10.0);
    assert_eq!(single, 10.0);

    // The arguments are arbitrary *expressions*, not just literals.
    let x = 1.0;
    let b = avg!(x, x + 1.0, x + 2.0);
    assert_eq!(b, 2.0);

    // Integer expressions are cast to f64, so the average can be fractional.
    let c = avg!(1, 2, 3, 4);
    assert_eq!(c, 2.5);
}
