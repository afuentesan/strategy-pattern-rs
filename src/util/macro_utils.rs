
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, ItemFn, ReturnType, Type, TypeBareFn};
use syn::spanned::Spanned;

pub fn get_attributes<T>( args : TokenStream ) -> Result<T, TokenStream>
where T: FromMeta
{
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            // Write error to output token stream if there is one
            return Err( proc_macro::TokenStream::from(Error::from(e).write_errors()) );
            // return Err( Error::from(e) )
        }
    };
    
    let salida = match T::from_list(&attr_args) {
        Ok(params) => params,
        Err(error) => {
            // Write error to output token stream if there is one
            return Err( proc_macro::TokenStream::from(Error::from(error).write_errors()) );
            // return Err( Error::from(error) )
        }
    };

    Ok( salida )
}

pub fn get_params_fn( item_fn : &ItemFn, span : Option<Span> ) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let types_fn = item_fn.sig.inputs.iter().map(|param|{

        if let syn::FnArg::Typed(argument) = param {
            return Some( *argument.ty.clone() );
        }

        None
    } ).filter( | t | t.is_some() ).map( | t | t.unwrap() ).collect::<Vec<Type>>();

    let span = match span
    {
        Some( v ) => v,
        _ => item_fn.sig.span()
    };

    get_params_from_types( types_fn, span )
}

pub fn get_params_bare_fn( bare_fn : &TypeBareFn, span : Option<Span> ) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let types_fn = bare_fn.inputs.iter().map( | i | i.ty.clone() ).collect();

    let span = match span
    {
        Some( v ) => v,
        _ => bare_fn.span()
    };

    get_params_from_types( types_fn, span )
}

fn get_params_from_types( types_fn : Vec<Type>, span : Span ) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream )
{
    let mut types = quote!{};
    let mut names = quote!{};
    let mut full = quote! {};

    let prefix_name = "__a_";

    for i in 0..types_fn.len()
    {
        let t = types_fn[ i ].clone();
        let n = Ident::new( &format!( "{}{}", prefix_name, i ), span );

        types.extend( quote! { #t });
        names.extend( quote! { #n } );
        full.extend( quote! { #n: #t } );

        if i < ( types_fn.len() - 1 )
        {
            types.extend( quote! { , } );
            names.extend( quote! { , } );
            full.extend( quote! { , } );
        }
    }

    ( 
        types,
        names,
        full
    )
}

pub fn get_fn_return_type_without_token( return_type : ReturnType ) -> Type
{
    match return_type {
        ReturnType::Default => syn::Type::Verbatim( quote! { () } ),
        ReturnType::Type( _, t ) => *t
    }
}

pub fn get_return_type_result( return_type : ReturnType ) -> proc_macro2::TokenStream
{
    let output = get_fn_return_type_without_token( return_type.clone() );

    quote! { -> ::std::result::Result<#output, std::string::String> }
}
