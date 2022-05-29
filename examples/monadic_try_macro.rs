#![feature(try_trait_v2)]

use derive_try::{IdTry, Try};
use std::ops;

#[derive(Debug, IdTry)]
struct Id<T>(T);

fn id_sample() -> Id<u8> {
    Id(Id(1)? + Id(2)? + Id(3)?)
}

#[derive(Debug, Try)]
struct NewResult<T, E>(Result<T, E>);

impl<T, E> NewResult<T, E> {
    fn new(value: T) -> Self {
        Self(Ok(value))
    }
}

fn new_result_sample() -> NewResult<u8, String> {
    NewResult::new(NewResult::new(1)? + NewResult::new(2)? + NewResult::new(3)?)
}

#[derive(Debug, Try)]
struct New<T: ops::Try>(T);

fn new_sample() -> New<Option<u8>> {
    New(Some(New(Some(1))? + New(Some(2))? + New(Some(3))?))
}

fn main() {
    println!("id: {:?}", id_sample());
    println!("new: {:?}", new_sample());
    println!("new_result: {:?}", new_result_sample());
}
