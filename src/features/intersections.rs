use std::ops::Index;

use crate::{features::compare_equal, objects::Object};

#[derive(Clone, Copy, Debug)]
pub struct Intersection{
    pub t: f32,
    pub object: Object,
}

#[derive(Clone, Debug)]
pub struct Intersections (Vec<Intersection>);

impl Intersections {
    pub fn new(i: Vec<Intersection>) -> Self {
        Self(i)
    }

    //Returns closest hit we can see (?)
    pub fn hit(&self) -> Option<Intersection> {
        let mut hit: Option<Intersection> = None;

        for i in 0..self.len() {
            if self[i].t >= 0.0 {
                match hit {
                    None => hit = Some(self[i]),
                    Some(current) => 
                        if current.t > self[i].t { 
                            hit = Some(self[i])
                        }
                }
            }
        }

        hit
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, i: Intersection) {
        self.0.push(i);
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Intersection {
    pub fn new(t: f32, object: Object) -> Self {
        Self {
            t,
            object
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        compare_equal(self.t, other.t) && self.object == other.object
    }
}