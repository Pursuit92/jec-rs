use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use std::fmt::Display;
use std::fmt;


#[derive(Clone,Debug)]
pub struct RcCow<T>
    where T: 'static
{
    data: Rc<T>,
}

impl<T> RcCow<T> {
    pub fn to_mut(&mut self) -> &mut T
        where T: Clone
    {
        Rc::make_mut(&mut self.data)
    }

    pub fn into_owned(self) -> T
        where T: Clone
    {
        match Rc::try_unwrap(self.data.clone()) {
            Ok(v) => v,
            Err(_) => T::clone(&self.data),
        }
    }

    pub fn unwrap(self) -> Rc<T> {
        return self.data;
    }
}

impl<T> Display for RcCow<T>
    where T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Deref for RcCow<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data.deref()
    }
}

impl<T> DerefMut for RcCow<T>
    where T: Clone
{
    fn deref_mut(&mut self) -> &mut T {
        Rc::make_mut(&mut self.data)
    }
}

impl<T> RcCow<T> {
    pub fn new(val: T) -> RcCow<T> {
        RcCow { data: Rc::new(val) }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stuff() {
        let mut a = RcCow::new(5i32);
        let mut b = a.clone();

        *b += 1;
        let mut c = b.clone();
        *b += 2;

        panic!("a: {}, b: {}, c: {}", a, b, c);
    }
}
