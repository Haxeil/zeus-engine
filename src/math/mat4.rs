use std::ops::{Add, Div, Mul, Sub};

use super::{vec3::Vec3, vec4::Vec4};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]

pub struct Mat4 {
    elements: [f32; 4 * 4],
}

impl Mat4 {
    pub fn new(diagonal: f32) -> Self {
        let mut elements = [0.0; 4 * 4];

        elements[0 + 0 * 4] = diagonal;
        elements[1 + 1 * 4] = diagonal;
        elements[2 + 2 * 4] = diagonal;
        elements[3 + 3 * 4] = diagonal;

        Self { elements }
    }

    pub fn new_identity() -> Self {
        Self::new(1.0)
    }

    pub fn zero() -> Self {
        Self {
            elements: [0.0; 4 * 4],
        }
    }
}

impl Mat4 {
    pub fn orthographic(left: f32, right: f32, top: f32, buttom: f32, near: f32, far: f32) -> Mat4 {
        let mut result = Mat4::new(1.0);

        result.elements[0 + 0 * 4] = 2.0 / (right - left);
        result.elements[1 + 1 * 4] = 2.0 / (top - buttom);
        result.elements[2 + 2 * 4] = 2.0 / (near - far);

        result.elements[0 + 3 * 4] = (left + right) /  (left - right);
        result.elements[1 + 3 * 4] = (buttom + top) / (buttom - top);
        result.elements[2 + 3 * 4] = (far + near) / (far - near);

        result
    }

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

    pub fn translation(translation: &Vec3) -> Self {
        let mut result = Mat4::new(1.0);

        result.elements[0 + 3 * 4] = translation.x;
        result.elements[1 + 3 * 4] = translation.y;
        result.elements[2 + 2 * 4] = translation.z;

        result

    }

    pub fn rotation(angle: f32, axis: &Vec3) -> Self {
        Self { elements: todo!() }
    }

    pub fn scale(scale: &Vec3) -> Self {
        let mut result = Mat4::new(1.0);

        result.elements[0 + 0 * 4] = scale.x;
        result.elements[1 + 1 * 4] = scale.y;
        result.elements[2 + 2 * 4] = scale.z;

        result
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

impl Div for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: Self) -> Self::Output {
        panic!("not defined");
    }
}
