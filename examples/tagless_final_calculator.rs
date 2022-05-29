#![feature(try_trait_v2)]

use std::{convert, fmt, ops};

#[derive(Debug)]
struct StringCalcF(String);

impl ops::FromResidual<<Self as ops::Try>::Residual> for StringCalcF {
    fn from_residual(residual: <Self as ops::Try>::Residual) -> Self {
        match residual {}
    }
}

impl ops::Try for StringCalcF {
    type Output = String;
    type Residual = convert::Infallible;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        StringCalcF(output)
    }

    #[inline]
    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        ops::ControlFlow::Continue(self.0)
    }
}

impl fmt::Display for StringCalcF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add for StringCalcF {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        <StringCalc as Calculator<String>>::plus(self, rhs)
    }
}

trait LiftTry<Input>: Sized {
    type Output;
    type F: ops::Try<Output = Self::Output>;

    fn lift(value: Input) -> Self::F;
}

trait Calculator<A>: LiftTry<A> {
    fn plus(lhs: Self::F, rhs: Self::F) -> Self::F;
}

struct StringCalc;

impl<A: fmt::Display> LiftTry<A> for StringCalc {
    type Output = String;
    type F = StringCalcF;

    fn lift(value: A) -> Self::F {
        StringCalcF(format!("{}", value))
    }
}

impl<A: fmt::Display> Calculator<A> for StringCalc {
    fn plus(lhs: Self::F, rhs: Self::F) -> Self::F {
        StringCalcF(format!("{} + {}", lhs.0, rhs.0))
    }
}

#[derive(Debug)]
struct CalcF<T>(Result<T, CalcError>);

impl<T> ops::FromResidual for CalcF<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: <Result<T, CalcError> as ops::Try>::Residual) -> Self {
        Self(<Result<T, CalcError> as ops::FromResidual>::from_residual(
            residual,
        ))
    }
}

impl<T> ops::Try for CalcF<T> {
    type Output = T;
    type Residual = <Result<T, CalcError> as ops::Try>::Residual;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        CalcF(<Result<T, CalcError> as ops::Try>::from_output(output))
    }

    #[inline]
    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        self.0.branch()
    }
}

struct Calc;

#[derive(Debug)]
enum CalcError {
    Overflow,
}

impl<A> LiftTry<A> for Calc {
    type Output = A;
    type F = CalcF<Self::Output>;

    fn lift(value: A) -> Self::F {
        CalcF(Ok(value))
    }
}

impl<A> Calculator<A> for Calc
where
    A: num::CheckedAdd<Output = A>,
{
    fn plus(lhs: Self::F, rhs: Self::F) -> Self::F {
        let l = lhs?;
        let r = rhs?;
        CalcF(l.checked_add(&r).ok_or(CalcError::Overflow))
    }
}

impl<A> ops::Add for CalcF<A>
where
    A: num::CheckedAdd<Output = A>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        <Calc as Calculator<A>>::plus(self, rhs)
    }
}

fn program<A, T>(a: A, b: A, c: A) -> T::F
where
    T: Calculator<A>,
    T::F: ops::Add<Output = T::F>,
{
    T::lift(a) + T::lift(b) + T::lift(c)
}

fn main() {
    println!("string: {:?}", program::<_, StringCalc>(3, 2, 1));

    println!("generic: {:?}", program::<_, Calc>(3, 2, 1));

    println!("overflow: {:?}", program::<u8, Calc>(254, 254, 1));
}
