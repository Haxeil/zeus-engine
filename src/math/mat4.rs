use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use super::{vec3::Vec3, vec4::Vec4};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]

pub struct Mat4 {
    pub elements: [f32; 4 * 4],
}

impl Mat4 {
    #[inline]
    pub fn new(diagonal: f32) -> Self {
        let mut result = Mat4::zero();

        result.elements[0 + 0 * 4] = diagonal;
        result.elements[1 + 1 * 4] = diagonal;
        result.elements[2 + 2 * 4] = diagonal;
        result.elements[3 + 3 * 4] = diagonal;

        result
    }

    #[inline]
    pub fn new_identity() -> Self {
        Self::new(1.0)
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            elements: [0.0; 4 * 4],
        }
    }

    /// Access a column as a reference to Vec4 (without copying data)
    #[inline]
    pub fn column(&self, index: usize) -> &Vec4 {
        assert!(index < 4, "Column index out of bounds");
        unsafe { &*(self.elements.as_ptr().add(index * 4) as *const Vec4) }
    }

    /// Access a mutable column as a reference to Vec4 (without copying data)
    #[inline]
    pub fn column_mut(&mut self, index: usize) -> &mut Vec4 {
        assert!(index < 4, "Column index out of bounds");
        unsafe { &mut *(self.elements.as_mut_ptr().add(index * 4) as *mut Vec4) }
    }
}

impl Mat4 {
    #[inline]
    pub fn orthographic(left: f32, right: f32, top: f32, buttom: f32, near: f32, far: f32) -> Mat4 {
        let mut result = Mat4::new(1.0);

        result.elements[0 + 0 * 4] = 2.0 / (right - left);
        result.elements[1 + 1 * 4] = 2.0 / (top - buttom);
        result.elements[2 + 2 * 4] = 2.0 / (near - far);

        result.elements[0 + 3 * 4] = (left + right) / (left - right);
        result.elements[1 + 3 * 4] = (buttom + top) / (buttom - top);
        result.elements[2 + 3 * 4] = (far + near) / (far - near);

        result
    }

    #[inline]
    pub fn prespective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let mut result = Mat4::new(1.0);

        let q = 1.0 / (fov * 0.5).tan();
        let a = q / aspect_ratio;

        let b = (near + far) / (near - far);
        let c = (2.0 * near + far) / (near - far);

        result.elements[0 + 0 * 4] = a;
        result.elements[1 + 1 * 4] = q;

        result.elements[2 + 2 * 4] = b;
        result.elements[3 + 2 * 4] = -1.0;

        result.elements[2 + 3 * 4] = c;

        result
    }

    #[inline]
    pub fn translation(translation: &Vec3) -> Self {
        let mut result = Mat4::new(1.0);

        result.elements[0 + 3 * 4] = translation.x;
        result.elements[1 + 3 * 4] = translation.y;
        result.elements[2 + 2 * 4] = translation.z;

        result
    }

    #[inline]
    pub fn rotation(angle: f32, axis: &Vec3) -> Self {
        let mut result = Mat4::new(1.0);

        let r = angle.to_radians();
        let c = r.cos();
        let s = c.sin();
        let omc = 1.0 - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        result.elements[0 + 0 * 4] = x * omc + c;
        result.elements[1 + 0 * 4] = y * x * omc + z * s;
        result.elements[2 + 0 * 4] = x * z * omc - y * s;

        result.elements[0 + 1 * 4] = x * y * omc - z * s;
        result.elements[1 + 1 * 4] = y * omc + c;
        result.elements[2 + 1 * 4] = y * z * omc + x * s;

        result.elements[0 + 2 * 4] = x * z * omc + y * s;
        result.elements[1 + 2 * 4] = y * z * omc - x * s;
        result.elements[2 + 2 * 4] = z * omc + c;

        result
    }

    #[inline]
    pub fn scale(scale: &Vec3) -> Self {
        let mut result = Mat4::new(1.0);

        result.elements[0 + 0 * 4] = scale.x;
        result.elements[1 + 1 * 4] = scale.y;
        result.elements[2 + 2 * 4] = scale.z;

        result
    }
}

// Implement indexing to access and modify columns using array syntax
impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        self.column(index)
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.column_mut(index)
    }
}

impl Add for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Self) -> Self::Output {
        let mut elements = [0.0; 4 * 4];

        for i in 0..elements.len() {
            elements[i] = self.elements[i] + rhs.elements[i];
        }

        Self { elements }
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut elements = [0.0; 4 * 4];

        for i in 0..elements.len() {
            elements[i] = self.elements[i] - rhs.elements[i];
        }

        Self { elements }
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut elements = [0.0; 4 * 4];

        for y in 0..4 {
            for x in 0..4 {
                let mut sum = 0.0;
                for e in 0..4 {
                    sum += elements[x + e * 4] * rhs.elements[e + y * 4]
                }
                elements[x + y * 4] = sum;
            }
        }

        Self { elements }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: Self) -> Self::Output {
        panic!("not defined");
    }
}
