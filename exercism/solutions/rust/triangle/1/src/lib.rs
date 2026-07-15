#[derive(Debug)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Copy + PartialOrd + Into<f64>,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let (a, b, c) = (sides[0], sides[1], sides[2]);

        let (a0, b0, c0) = (a.into(), b.into(), c.into());

        // > 0
        if a0 <= 0.0 || b0 <= 0.0 || c0 <= 0.0 {
            return None;
        }

        // triangle inequality
        if a0 + b0 <= c0 || a0 + c0 <= b0 || b0 + c0 <= a0 {
            return None;
        }

        Some(Self { a, b, c })
    }

    fn approx_eq(x: f64, y: f64) -> bool {
        (x - y).abs() < 1e-9
    }

    pub fn is_equilateral(&self) -> bool {
        let (a, b, c) = (self.a.into(), self.b.into(), self.c.into());
        Self::approx_eq(a, b) && Self::approx_eq(b, c)
    }

    pub fn is_isosceles(&self) -> bool {
        let (a, b, c) = (self.a.into(), self.b.into(), self.c.into());
        Self::approx_eq(a, b)
            || Self::approx_eq(b, c)
            || Self::approx_eq(c, a)
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }
}
