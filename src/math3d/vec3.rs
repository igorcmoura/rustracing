use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    e: [f64; 3],
}

// Point3 is just an alias for Vector3, but useful for geometric clarity in the code.
pub type Point3 = Vector3;

impl Vector3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn zero() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Vector3 {
        *self / self.length()
    }
}

impl ops::Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    #[inline]
    fn add(self, v: Self) -> Self::Output {
        Self::new(self[0] + v[0], self[1] + v[1], self[2] + v[2])
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, v: Self) {
        self[0] += v[0];
        self[1] += v[1];
        self[2] += v[2];
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, v: Self) -> Self::Output {
        Self::new(self[0] - v[0], self[1] - v[1], self[2] - v[2])
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, v: Self) -> Self::Output {
        Self::new(self[0] * v[0], self[1] * v[1], self[2] * v[2])
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    #[inline]
    fn mul(self, v: Vector3) -> Self::Output {
        Vector3::new(self * v[0], self * v[1], self * v[2])
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, t: f64) {
        self[0] *= t;
        self[1] *= t;
        self[2] *= t;
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
