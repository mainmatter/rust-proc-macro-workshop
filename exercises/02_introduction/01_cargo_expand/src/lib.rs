/// Run `cargo expand --lib` in this crate's directory to see what
/// `#[derive(Clone)]` generates for `Rgb`.
///
/// Then, based on what you see in the expanded output, write a manual
/// `Clone` implementation for `Hsl` that does the same thing.
#[derive(Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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
        let c2 = c.clone();
        assert_eq!(c.r, c2.r);
        assert_eq!(c.g, c2.g);
        assert_eq!(c.b, c2.b);
    }

    #[test]
    fn hsl_clone() {
        let c = Hsl {
            h: 0.5,
            s: 0.8,
            l: 0.3,
        };
        let c2 = c.clone();
        assert_eq!(c.h, c2.h);
        assert_eq!(c.s, c2.s);
        assert_eq!(c.l, c2.l);
    }
}
