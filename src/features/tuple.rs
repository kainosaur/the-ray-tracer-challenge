use crate::features::compare_equal;

#[derive(Clone, Copy, Debug)]
pub struct Tuple<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    x: T,
    y: T, 
    z: T
}

#[derive(Clone, Copy, Debug)]
pub struct Vector<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Tuple<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {x, y, z, w}
    }
    
}

impl Tuple<f32> {
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }

    pub fn is_point(&self) -> bool {
        compare_equal(self.w(), 1.0)
    }
    
    pub fn is_vector(&self) -> bool {
        compare_equal(self.w(), 0.)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn negate(&self) -> Self {
        Tuple::new(
            -self.x,
            -self.y,
            -self.z,
            -self.w
        )
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {x, y, z}
    }
}

impl Point<f32> {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Vector<f32> {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
    
    pub fn negate(&self) -> Self {
        Vector::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

pub trait TFTuple {
    fn as_tuple(&self) -> Tuple<f32>;
}

impl TFTuple for Point<f32> {
    fn as_tuple(&self) -> Tuple<f32> {
        return Tuple::new(self.x, self.y, self.z, 1.0)
    }
}

impl TFTuple for Vector<f32> {
    fn as_tuple(&self) -> Tuple<f32> {
        return Tuple::new(self.x, self.y, self.z, 0.0)
    }
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {x, y, z}
    }
}

struct Iter<'a> {
    inner: &'a Tuple<f32>,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a f32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.x,
            1 => &self.inner.y,
            2 => &self.inner.z,
            3 => &self.inner.w,
            _ => return None,
        };

        self.index += 1;
        Some(ret)
    }
}

impl PartialEq for Tuple<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(left, right)| compare_equal(*left, *right))
    }
}

impl PartialEq for Vector<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.as_tuple().iter()
            .zip(other.as_tuple().iter())
            .all(|(left, right)| compare_equal(*left, *right))
    }
}

impl PartialEq for Point<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.as_tuple().iter()
            .zip(other.as_tuple().iter())
            .all(|(left, right)| compare_equal(*left, *right))
    }
}