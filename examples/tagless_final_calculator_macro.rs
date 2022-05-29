#![feature(try_trait_v2)]

use derive_more::Display;
use derive_try::{IdTry, Try};
use std::{fmt, ops};

#[derive(Debug, IdTry, Display)]
struct StringCalcF(String);

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

#[derive(Debug, Try)]
struct CalcF<T>(Result<T, CalcError>);

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
