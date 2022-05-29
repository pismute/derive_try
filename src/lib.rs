#![feature(try_trait_v2)]

mod error;
mod helper;
mod try2;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

macro_rules! derive(
    ($name:ident, $mod_:ident, $fn_name: ident) => {
        #[proc_macro_derive($name)]
        #[doc(hidden)]
        pub fn $fn_name(input: TokenStream) -> TokenStream {
            let ast = parse_macro_input!(input as DeriveInput);

            let token_stream = $mod_::$fn_name(ast).map_err(|x| x.to_string()).unwrap();

            TokenStream::from(token_stream)
        }
    }
);

derive!(IdTry, try2, id_try_derive);
derive!(Try, try2, try_derive);
