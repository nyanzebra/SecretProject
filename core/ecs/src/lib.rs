use std::{fmt::Binary, ops::Add};

pub mod component;
pub mod entity;
pub mod system;
pub mod world;

type Base = usize;

/// A special `Id` that represents nothing, ultimately invalid.
/// This is useful for preloading some data and for other cases
/// where a unique id that is uninitialized helps.
pub(crate) const NULL_ID: Id = Id::new(0);

pub(crate) const ONE_ID: Id = Id::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Id(Base);

impl From<Base> for Id {
    fn from(value: Base) -> Self {
        Self::new(value)
    }
}

impl From<Id> for Base {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl Binary for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, f)
    }
}

impl Id {
    const fn new(val: Base) -> Self {
        Self(val)
    }

    #[inline]
    fn is_set(&self, bit: u8) -> bool {
        assert!((bit as u32) < Base::BITS, "bit too far");
        let bit = 1 << bit as Base;
        (self.0 & bit) != 0
    }

    #[inline]
    fn is_unset(&self, bit: u8) -> bool {
        assert!((bit as u32) < Base::BITS, "bit too far");
        let bit = 1 << bit as Base;
        (self.0 & bit) == 0
    }

    #[inline]
    fn set(&mut self, bit: u8) {
        assert!((bit as u32) < Base::BITS, "bit too far");
        let bit = 1 << bit as Base;
        self.0 |= bit;
    }

    #[inline]
    fn unset(&mut self, bit: u8) {
        assert!((bit as u32) < Base::BITS, "bit too far");
        let bit = 1 << bit as Base;
        self.0 &= !bit;
    }
}

impl Add for Id {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn id_bit_manip() {
        let mut id: Id = 1.into();
        println!("{:b}", id);
        assert!(id.is_set(0));
        println!("{:b}", id);
        id.unset(0);
        assert!(!id.is_set(0));
        id.set(4);
        println!("{:b}", id);
        assert!(id.is_set(4));
    }
}
