/// Run `cargo expand --lib` in this crate's directory to see what
/// `#[derive(Clone)]` generates for `Rgb`.
///
/// Then, based on what you see in the expanded output, write a manual
/// `Clone` implementation for `Hsl` that does the same thing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Note: no `Eq` here — `f64` is `PartialEq` but not `Eq`.
#[derive(Debug, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

// TODO: implement `Clone` for `Hsl` by hand, following the same pattern
//       you saw in the expanded output of `Rgb`.
impl Clone for Hsl {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_clone() {
        let c = Rgb {
            r: 255,
            g: 128,
            b: 0,
        };
        assert_eq!(c.clone(), c);
    }

    #[test]
    fn hsl_clone() {
        let c = Hsl {
            h: 0.5,
            s: 0.8,
            l: 0.3,
        };
        assert_eq!(c.clone(), c);
    }
}
