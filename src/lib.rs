use impls::{strategy_pattern_fn::strategy_pattern_fn_impl, strategy_pattern_type::strategy_pattern_type_impl};
use proc_macro::TokenStream;

mod impls;
mod util;

#[proc_macro_attribute]
pub fn strategy_pattern_type( args : TokenStream, item : TokenStream ) -> TokenStream
{
    strategy_pattern_type_impl( args, item )
}

#[proc_macro_attribute]
pub fn strategy_pattern_fn( args : TokenStream, item : TokenStream ) -> TokenStream
{
    strategy_pattern_fn_impl( args, item )
}

