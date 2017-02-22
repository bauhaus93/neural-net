
use std::clone::Clone;

use std::slice::{ Iter, IterMut };

pub struct Vector2<T> {
    size_row: usize,
    size_col: usize,
    data: Vec<T>
}

pub struct Vector3<T> {
    size_a: usize,
    size_b: usize,
    size_c: usize,
    data: Vec<T>
}

impl<T> Vector2<T>
    where T: Clone + Copy {

    pub fn new(init_val: T, row: usize, col: usize) -> Vector2<T> {
        Vector2{
            size_row: row,
            size_col: col,
            data: vec![init_val; row * col]
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.size_col + col]
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.size_col + col]
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter(&mut self) -> Iter<T> {
        self.data.iter()
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        &self.data[row * self.size_col..(row + 1) * self.size_col]
    }

    pub fn clear(&mut self, value: T) {
        for e in &mut self.data {
            *e = value;
        }
    }

}

impl<T> Vector3<T>
    where T: Clone + Copy{

    pub fn new(init_val: T, dim_a: usize, dim_b: usize, dim_c: usize) -> Vector3<T> {
        Vector3{
            size_a: dim_a,
            size_b: dim_b,
            size_c: dim_c,
            data: vec![init_val; dim_a * dim_b * dim_c]
        }
    }

    pub fn get_mut(&mut self, pos_a: usize, pos_b: usize, pos_c: usize) -> &mut T {
        &mut self.data[pos_a * self.size_b * self.size_c + pos_b * self.size_c + pos_c]
    }

    pub fn get(&self, pos_a: usize, pos_b: usize, pos_c: usize) -> &T {
        &self.data[pos_a * self.size_b * self.size_c + pos_b * self.size_c + pos_c]
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter(&mut self) -> Iter<T> {
        self.data.iter()
    }

    pub fn clear(&mut self, value: T) {
        for e in &mut self.data {
            *e = value;
        }
    }
}
