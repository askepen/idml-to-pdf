use ndarray::{arr1, arr2, Array2, Ix2};
use ndarray_linalg::Inverse;

#[derive(Debug)]
pub struct Transform {
    matrix: Array2<f64>,
    inverse: Option<Array2<f64>>,
}

/// Returns the identity matrix
pub fn identity() -> Transform {
    let m = arr2(&[
        [1_f64, 0_f64, 0_f64],
        [0_f64, 1_f64, 0_f64],
        [0_f64, 0_f64, 1_f64],
    ]);

    Transform {
        matrix: m,
        inverse: None,
    }
}

/// Creates a transformation matrix as specified in the IDML specification, page 102
/// https://wwwimages.adobe.com/content/dam/acom/en/devnet/indesign/sdk/cs6/idml/idml-specification.pdf
pub fn from_values(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Transform {
    let m = arr2(&[
        [a.to_owned(), b.to_owned(), 0_f64],
        [c.to_owned(), d.to_owned(), 0_f64],
        [e.to_owned(), f.to_owned(), 1_f64],
    ]);
    Transform {
        matrix: m,
        inverse: None,
    }
}

/// Creates a transformation matrix from a Option of a Vec of the form [a,b,c,d,e,f]
pub fn from_vec(transform: &Option<Vec<f64>>) -> Transform {
    match transform {
        Some(transform) => match transform.as_slice() {
            &[a, b, c, d, e, f] => from_values(a, b, c, d, e, f),
            _ => identity(),
        },
        _ => identity(),
    }
}

pub fn transform_point(x: &f64, y: &f64, transform: &Transform) -> Vec<f64> {
    // Has the form [x, y, 1] in order to match dimensions of transformation matrix
    let homogenous_coords = &arr1(&[x.to_owned(), y.to_owned(), 1_f64]);
    let point = homogenous_coords.dot(&transform.matrix);
    vec![point[[0]], point[[1]]]
}

pub fn _transform_point_reverse(x: &f64, y: &f64, transform: &Transform) -> Vec<f64> {
    transform_point(x, y, &reverse(transform))
}

pub fn reverse(transform: &Transform) -> Transform {
    // If we know the inverse, just return that
    if let Some(inv) = &transform.inverse {
        Transform {
            matrix: inv.to_owned(),
            inverse: Some(transform.matrix.to_owned()),
        }
    } else {
        let inv = transform
            .matrix
            .inv()
            .expect(format!("Failed to invert the matrix {:#?}", transform.matrix).as_str());
        Transform {
            matrix: inv,
            inverse: Some(transform.matrix.to_owned()),
        }
    }
}

/// Returns dot product of matrices
pub fn combine(transform1: &Transform, transform2: &Transform) -> Transform {
    Transform {
        matrix: transform1.matrix.dot(&transform2.matrix),
        inverse: None,
    }
}

impl Transform {
    pub fn apply_to_point(&self, x: &f64, y: &f64) -> Vec<f64> {
        transform_point(x, y, &self)
    }

    pub fn combine_with(&self, transform: &Transform) -> Transform {
        combine(&self, transform)
    }

    pub fn reverse(&self) -> Transform {
        reverse(&self)
    }

    fn with_updated_value_at_index(self, value: f64, index: Ix2) -> Transform {
        let mut new_matrix = self.matrix;
        new_matrix[index] = value;
        Transform {
            matrix: new_matrix,
            inverse: None,
        }
    }

    pub fn with_a(self, a: f64) -> Transform {
        self.with_updated_value_at_index(a, Ix2(0, 0))
    }

    pub fn with_b(self, b: f64) -> Transform {
        self.with_updated_value_at_index(b, Ix2(0, 1))
    }

    pub fn with_c(self, c: f64) -> Transform {
        self.with_updated_value_at_index(c, Ix2(1, 0))
    }

    pub fn with_d(self, d: f64) -> Transform {
        self.with_updated_value_at_index(d, Ix2(1, 1))
    }

    pub fn with_e(self, e: f64) -> Transform {
        self.with_updated_value_at_index(e, Ix2(2, 0))
    }

    pub fn with_f(self, f: f64) -> Transform {
        self.with_updated_value_at_index(f, Ix2(2, 1))
    }

    pub fn with_transpose(self, x: f64, y: f64) -> Transform {
        self.with_e(x).with_f(y)
    }

    pub fn with_scale(self, x_scale: f64, y_scale: f64) -> Transform {
        self.with_a(x_scale).with_d(y_scale)
    }
}
