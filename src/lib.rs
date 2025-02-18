use impls::{strategy_pattern_fn::strategy_pattern_fn_impl, strategy_pattern_type::strategy_pattern_type_impl};
use proc_macro::TokenStream;

mod impls;
mod util;

/// Attribute macro to define a strategy type.
/// 
/// This macro should be applied to a struct that will serve as a strategy container.
/// It allows defining a type that will store function pointers and execute them dynamically.
/// 
/// # Parameters
/// - `search`: Defines the search mode for function keys. It can be:
///   - `"Exact"` → Exact match.
///   - `"IgnoreCase"` → Case-insensitive match.
///   - `"RegExp"` → Uses regular expressions.
/// 
/// # Example
/// ```
/// #[strategy_pattern_type(search = "IgnoreCase")]
/// pub struct GreetingStrategy(fn(String) -> String);
/// ```
#[proc_macro_attribute]
pub fn strategy_pattern_type( args : TokenStream, item : TokenStream ) -> TokenStream
{
    strategy_pattern_type_impl( args, item )
}

/// Attribute macro to register a function as part of a strategy pattern.
/// 
/// This macro should be applied to functions that will be executed dynamically based on a key.
/// 
/// # Parameters
/// - `strategy`: The name of the struct that represents the strategy.
/// - `key`: A string used to identify the function within the strategy.
/// 
/// # Example
/// ```
/// #[strategy_pattern_fn(strategy = GreetingStrategy, key = "formal_greeting")]
/// pub fn formal_greeting(name: String) -> String {
///     format!("Good day, {name}")
/// }
/// ```
#[proc_macro_attribute]
pub fn strategy_pattern_fn( args : TokenStream, item : TokenStream ) -> TokenStream
{
    strategy_pattern_fn_impl( args, item )
}

