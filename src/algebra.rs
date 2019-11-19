use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

type Unit = f64;

/// 3 dimentional vector type used in this ray tracer.
///
/// Can also be used as point.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Vector(pub Unit, pub Unit, pub Unit);

impl Vector {
    /// Vector with absolute values.
    #[inline]
    pub fn abs(self) -> Self {
        Vector(self.0.abs(), self.1.abs(), self.2.abs())
    }

    /// Normalize this vector to a magnitude of 1.
    #[inline]
    pub fn normalize(self) -> Self {
        self / self.abs()
    }

    /// Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> Unit {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Magnitude or length.
    #[inline]
    pub fn magnitude(self) -> Unit {
        self.magnitude_squared().sqrt()
    }

    /// Squared magnitude or length.
    #[inline]
    pub fn magnitude_squared(self) -> Unit {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1, -self.2)
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Unit> for Vector {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector(
            if rhs.0 != 0f64 { self.0 / rhs.0 } else { 0f64 },
            if rhs.1 != 0f64 { self.1 / rhs.1 } else { 0f64 },
            if rhs.1 != 0f64 { self.1 / rhs.1 } else { 0f64 },
        )
    }
}

/// Type that has an identity value.
///
/// Will be the zero point for points and vectors.
pub trait Identity {
    /// Construct an identity variant of this type.
    fn identity() -> Self;
}

impl Identity for Vector {
    fn identity() -> Self {
        Vector(0.0, 0.0, 0.0)
    }
}
