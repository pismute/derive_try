use crate::error::Error;
use crate::helper::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident, Type};

fn quote_id_try(ident: &Ident, field_type: &Type, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let from_residual = quote! {
        impl #impl_generics ::std::ops::FromResidual<<Self as ::std::ops::Try>::Residual> for #ident #ty_generics #where_clause {

            #[inline]
            #[track_caller]
            fn from_residual(residual: <Self as ::std::ops::Try>::Residual) -> Self {
                match residual {}
            }
        }

    };

    let trei = quote! {
        impl #impl_generics ::std::ops::Try for #ident #ty_generics #where_clause {
            type Output = #field_type;
            type Residual = ::std::convert::Infallible;

            #[inline]
            fn from_output(output: Self::Output) -> Self {
                Self(output)
            }

            #[inline]
            fn branch(self) -> ::std::ops::ControlFlow<Self::Residual, Self::Output> {
                ::std::ops::ControlFlow::Continue(self.0)
            }
        }
    };

    let toks = quote! {
        #from_residual
        #trei
    };

    toks.into()
}

pub(crate) fn id_try_derive(ast: DeriveInput) -> Result<TokenStream, Error> {
    let data = get_data_struct(&ast)?;

    let field = get_field(data)?;

    Ok(quote_id_try(&ast.ident, &field.ty, &ast.generics))
}

fn quote_try(ident: &Ident, field_type: &Type, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let from_residual = quote! {
        impl #impl_generics ::std::ops::FromResidual for #ident #ty_generics #where_clause {
            #[inline]
            #[track_caller]
            fn from_residual(residual: <#field_type as ::std::ops::Try>::Residual) -> Self {
                Self(<#field_type as ::std::ops::FromResidual>::from_residual(residual))
            }
        }
    };

    let trei = quote! {
        impl #impl_generics ::std::ops::Try for #ident #ty_generics #where_clause {
            type Output = <#field_type as ::std::ops::Try>::Output;
            type Residual = <#field_type as ::std::ops::Try>::Residual;

            #[inline]
            fn from_output(output: Self::Output) -> Self {
                Self(<#field_type as ::std::ops::Try>::from_output(output))
            }

            #[inline]
            fn branch(self) -> ::std::ops::ControlFlow<Self::Residual, Self::Output> {
                self.0.branch()
            }
        }
    };

    let toks = quote! {
        #from_residual
        #trei
    };

    toks.into()
}

pub(crate) fn try_derive(ast: DeriveInput) -> Result<TokenStream, Error> {
    let data = get_data_struct(&ast)?;

    let field = get_field(data)?;

    let token_stream = quote_try(&ast.ident, &field.ty, &ast.generics);

    Ok(token_stream)
}
