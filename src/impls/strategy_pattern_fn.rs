use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

use crate::util::macro_utils::{get_attributes, get_params_fn};

#[derive(FromMeta)]
struct StrategyFnParams
{
    key : String,
    strategy : Ident
}

pub fn strategy_pattern_fn_impl( args : TokenStream, item : TokenStream ) -> TokenStream
{
    let args = match get_attributes::<StrategyFnParams>( args )
    {
        Ok( v ) => v,
        Err( e ) => return e
    };

    let key = &args.key;

    let item_fn = parse_macro_input!( item as ItemFn );

    let ident_fn = item_fn.sig.ident.clone();

    let ( _, param_names, _ ) = get_params_fn( &item_fn, None );

    let strategy = &args.strategy;

    quote! {
        
        #item_fn

        ::inventory::submit! {
            #strategy {
                fnc : | #param_names | { #ident_fn( #param_names ) },
                key : #key
            }
        }
    }.into()
}