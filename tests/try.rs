#![feature(try_trait_v2)]
#![feature(try_blocks)]

use std::convert;

use derive_try::{IdTry, Try};

#[derive(Debug, IdTry)]
struct Id<T>(T);

#[test]
fn id_try_test() {
    let ret: Id<u8> = try { Id(1)? + Id(2)? + Id(3)? };

    assert_eq!(ret.0, 6);
}

#[derive(Debug, IdTry)]
struct IdU8(u8);

#[test]
fn id_try_non_generic_test() {
    let ret: IdU8 = try { IdU8(1)? + IdU8(2)? + IdU8(3)? };

    assert_eq!(ret.0, 6);
}

#[derive(Debug, Try)]
struct NTry<T: std::ops::Try>(T);

#[test]
fn try_test() {
    let ret: NTry<Option<u8>> = try { NTry(Some(1))? + NTry(Some(2))? + NTry(Some(3))? };

    assert_eq!(ret.0, Some(6));
}

#[derive(Debug, Try)]
struct TryRes<T, E>(Result<T, E>);

#[test]
fn try_res_test() {
    let ret: TryRes<u8, convert::Infallible> =
        try { TryRes(Ok(1))? + TryRes(Ok(2))? + TryRes(Ok(3))? };

    assert_eq!(ret.0, Ok(6));
}
