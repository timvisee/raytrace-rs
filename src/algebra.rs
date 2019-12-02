use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

type Unit = f64;

/// 3 dimentional vector type used in this ray tracer.
///
/// Can also be used as point.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Vector(pub Unit, pub Unit, pub Unit);

impl Vector {
    /// Normalize this vector to a magnitude of 1.
    #[inline]
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    /// Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> Unit {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Cross product.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    /// Magnitude or length.
    #[inline]
    pub fn magnitude(self) -> Unit {
        self.magnitude_squared().sqrt()
    }

    /// Squared magnitude or length.
    #[inline]
    pub fn magnitude_squared(self) -> Unit {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
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

impl Div<Unit> for Vector {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self::Output {
        if rhs != 0.0 {
            Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
        } else {
            Vector::identity()
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::EPSILON;

    #[test]
    fn test_dot() {
        assert_unit_equal(Vector(0.0, 0.0, 0.0).dot(Vector(0.0, 0.0, 0.0)), 0.0);
        assert_unit_equal(Vector(1.0, 1.0, 1.0).dot(Vector(1.0, 1.0, 1.0)), 3.0);
        assert_unit_equal(Vector(1.0, 2.0, 3.0).dot(Vector(3.0, 4.0, 5.0)), 26.0);
        assert_unit_equal(Vector(1.0, 2.0, 3.0).dot(Vector(-1.0, -2.0, -3.0)), -14.0);
        assert_unit_equal(
            Vector(2.0, 9.5, 0.0).dot(Vector(14.2, 12.0, 36.0)),
            712.0 / 5.0,
        );
    }

    #[test]
    fn test_cross() {
        assert_vector_equal(
            Vector(0.0, 0.0, 0.0).cross(Vector(0.0, 0.0, 0.0)),
            Vector(0.0, 0.0, 0.0),
        );
        assert_vector_equal(
            Vector(1.0, 1.0, 1.0).cross(Vector(1.0, 1.0, 1.0)),
            Vector(0.0, 0.0, 0.0),
        );
        assert_vector_equal(
            Vector(1.0, 2.0, 3.0).cross(Vector(3.0, 4.0, 5.0)),
            Vector(-2.0, 4.0, -2.0),
        );
        assert_vector_equal(
            Vector(1.0, 2.0, 3.0).cross(Vector(-1.0, -2.0, -3.0)),
            Vector(0.0, 0.0, 0.0),
        );
        assert_vector_equal(
            Vector(2.0, 9.5, 0.0).cross(Vector(14.2, 12.0, 36.0)),
            Vector(342.0, -72.0, -110.9),
        );
    }

    /// Check whether units are almost equal, taking the epsilon into account.
    fn assert_unit_equal(a: Unit, b: Unit) {
        assert!(
            (a - b).abs() < EPSILON,
            "floats {} and {} are not almost equal",
            a,
            b
        );
    }

    /// Check whether units are almost equal, taking the epsilon into account.
    fn assert_vector_equal(a: Vector, b: Vector) {
        assert!(
            (a.0 - b.0).abs() < EPSILON
                && (a.1 - b.1).abs() < EPSILON
                && (a.2 - b.2).abs() < EPSILON,
            "vectors {:?} and {:?} are not almost equal",
            a,
            b
        );
    }
}
