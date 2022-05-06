use std::fmt::{Debug, Display, Formatter};

pub enum Either<T, U> {
    A(T),
    B(U)
}

impl<A: Debug, B: Debug> Debug for Either<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::A(a) => write!(f, "{:?}", a),
            Either::B(b) => write!(f, "{:?}", b),
        }
    }
}

impl<A: Display, B: Display> Display for Either<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::A(a) => write!(f, "{}", a),
            Either::B(b) => write!(f, "{}", b),
        }
    }
}

impl<A: Clone, B: Clone> Clone for Either<A, B> {
    fn clone(&self) -> Self {
        return match self {
            Either::A(a) => Either::A(a.clone()),
            Either::B(b) => Either::B(b.clone()),
        }
    }
}