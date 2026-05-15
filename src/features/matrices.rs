use crate::features::compare_equal;

#[derive(Clone, Copy, Debug)]
pub struct Matrix2x2<T>(pub [T; 2], pub [T; 2]);


#[derive(Clone, Copy, Debug)]
pub struct Matrix3x3<T>(pub [T; 3], pub [T; 3], pub [T; 3]);


#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4<T>(pub [T; 4], pub [T; 4], pub [T; 4], pub [T; 4]);

impl<T> Matrix2x2<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self([a, b], [c, d])
    }
}

impl<T> Matrix3x3<T> {
    pub fn new(a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T, i: T) -> Self {
        Self([a, b, c], [d, e, f], [g, h, i])
    }
}

impl<T> Matrix4x4<T> {
        pub fn new(a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T, i: T, j: T, k: T, l: T, m: T, n: T, o: T, p: T) -> Self {
        Self([a,b,c,d], [e,f,g,h], [i,j,k,l], [m,n,o,p])
    }
}

impl Matrix2x2<f32> {
    pub fn at(&self, row: usize, col: usize) -> f32 {
        match row {
            0 => self.0[col],
            1 => self.1[col],
            _ => panic!("Row out of bounds."),
        }
    }

    pub fn assign(&mut self, row: usize, col: usize, num: f32) -> Self {
        match row {
            0 => self.0[col] = num,
            1 => self.1[col] = num,
            _ => panic!("Assign val matrix fail, out of bounds.")
        }
        *self
    }
    pub fn new_zero() -> Self {
        Self([0.,0.], [0.,0.])
    }

    pub fn determinant(&self) -> f32 {
        self.0[0] * self.1[1] - self.1[0] * self.0[1]
    }
}

impl Matrix3x3<f32> {
    pub fn at(&self, row: usize, col: usize) -> f32 {
        match row {
            0 => self.0[col],
            1 => self.1[col],
            2 => self.2[col],
            _ => panic!("Row out of bounds."),
        }
    }

    pub fn new_zero() -> Self {
        Self([0.,0.,0.], [0.,0.,0.], [0.,0.,0.])
    }

    pub fn assign(&mut self, row: usize, col: usize, num: f32) -> Self {
        match row {
            0 => self.0[col] = num,
            1 => self.1[col] = num,
            2 => self.2[col] = num,
            _ => panic!("Assign val matrix fail, out of bounds.")
        }
        *self
    }
    
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix2x2<f32> {
        let mut values = [0.0; 4];
        let mut index = 0;
        for row in 0..3 {
            if row == remove_row {
                continue;
            }
            
            for col in 0..3 {
                if col == remove_col {
                    continue;
                }

                values[index] = self.at(row, col);
                index += 1;
            }
        }
        Matrix2x2::new(values[0], values[1], values[2], values[3])
    }
    
    pub fn minor( &self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.submatrix(row, col).determinant()
        } else {
            - self.submatrix(row, col).determinant()
        }
    }

    pub fn determinant(&self) -> f32 {
       self.at(0,0) * self.cofactor(0, 0) + 
       self.at(0, 1) * self.cofactor(0, 1) + 
       self.at(0, 2) * self.cofactor(0, 2)
    }
}

impl Matrix4x4<f32> {
    pub fn at(&self, row: usize, col: usize) -> f32 {
        match row {
            0 => self.0[col],
            1 => self.1[col],
            2 => self.2[col],
            3 => self.3[col],
            _ => panic!("Row out of bounds."),
        }
    }

    pub fn assign(&mut self, row: usize, col: usize, num: f32) -> Self {
        match row {
            0 => self.0[col] = num,
            1 => self.1[col] = num,
            2 => self.2[col] = num,
            3 => self.3[col] = num,
            _ => panic!("Assign val matrix fail, out of bounds.")
        }
        *self
    }

    pub fn identity() -> Self {
        Self(
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        )
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix4x4::identity();
        for row in 0..4 {
            for col in 0..4 {
                m.assign(
                    col, 
                    row, 
                    self.at(row, col)
                );
            }
        }
        m
    }

    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix3x3<f32> {
        let mut values = [0.0; 9];
        let mut index = 0;
        for row in 0..4 {
            if row == remove_row {
                continue;
            }
            
            for col in 0..4 {
                if col == remove_col {
                    continue;
                }

                values[index] = self.at(row, col);
                index += 1;
            }
        }

        Matrix3x3::new(values[0], values[1], values[2], values[3],values[4], values[5], values[6], values[7], values[8])
    }
    
    pub fn minor( &self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.submatrix(row, col).determinant()
        } else {
            - self.submatrix(row, col).determinant()
        }
    }

    pub fn determinant(&self) -> f32 {
       self.at(0,0) * self.cofactor(0, 0) + 
       self.at(0, 1) * self.cofactor(0, 1) + 
       self.at(0, 2) * self.cofactor(0, 2) +
       self.at(0, 3) * self.cofactor(0, 3)
    }

    pub fn is_invertible(&self) -> bool {
        if !compare_equal(self.determinant(), 0.) {
            true
        } else {
            false
        }
    }

    pub fn inverse(&self) -> Self {
        // u better be checking this first or else i will get you
        if self.is_invertible() == false {
            panic!("Matrix is not invertible.");
        };
        
        let mut m = Matrix4x4::identity();
        let det = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                m.assign(col, row, c / det);
            }
        }

        m
    }
}

impl PartialEq for Matrix2x2<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..1 {
            if !compare_equal(self.0[i], other.0[i]) || !compare_equal(self.1[i], other.1[i]) {
                return false
            }
        }
        true
    }
}

impl PartialEq for Matrix3x3<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..2 {
            if !compare_equal(self.0[i], other.0[i]) || !compare_equal(self.1[i], other.1[i])
            || !compare_equal(self.2[i], other.2[i]) {
                return false
            }
        }
        true
    }
}

impl PartialEq for Matrix4x4<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if !compare_equal(self.0[i], other.0[i]) || !compare_equal(self.1[i], other.1[i])
            || !compare_equal(self.2[i], other.2[i]) || !compare_equal(self.3[i], other.3[i]) {
                return false
            }
        }
        true
    }
}

