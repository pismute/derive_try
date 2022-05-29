#![feature(try_trait_v2)]

use std::{convert, ops};

#[derive(Debug)]
struct Id<T>(T);

impl<T> ops::FromResidual<<Self as ops::Try>::Residual> for Id<T> {
    fn from_residual(residual: <Self as ops::Try>::Residual) -> Self {
        match residual {}
    }
}

impl<T> ops::Try for Id<T> {
    type Output = T;
    type Residual = convert::Infallible;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Id(output)
    }

    #[inline]
    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        ops::ControlFlow::Continue(self.0)
    }
}

fn id_sample() -> Id<u8> {
    Id(Id(1)? + Id(2)? + Id(3)?)
}

#[derive(Debug)]
struct New<T>(T);

impl<T> ops::FromResidual for New<T>
where
    T: ops::FromResidual + ops::Try,
{
    #[inline]
    #[track_caller]
    fn from_residual(residual: <T as ops::Try>::Residual) -> Self {
        Self(<T as ops::FromResidual>::from_residual(residual))
    }
}

impl<T> ops::Try for New<T>
where
    T: ops::Try,
{
    type Output = <T as ops::Try>::Output;
    type Residual = <T as ops::Try>::Residual;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self(<T as ops::Try>::from_output(output))
    }

    #[inline]
    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        self.0.branch()
    }
}

fn new_sample() -> New<Result<u8, String>> {
    New(Ok(New(Ok(1))? + New(Ok(2))? + New(Ok(3))?))
}

fn main() {
    println!("id: {:?}", id_sample());
    println!("new: {:?}", new_sample());
}
