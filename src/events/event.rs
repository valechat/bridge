use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Event<'a, E> {
    pub name: &'a str,
    inner: E
}

impl<'a, E> Event<'a, E> {
    pub fn new(name: &'a str, inner: E) -> Self {
        Self {
            name,
            inner
        }
    }
}

impl<E> Event<'_, E> {
    pub fn into_inner(self) -> E {
        self.inner
    }
}

impl<E> Deref for Event<'_, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<E> DerefMut for Event<'_, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}