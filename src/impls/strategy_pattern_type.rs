use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Ident, ItemStruct, Type};

use crate::util::macro_utils::{get_attributes, get_params_bare_fn, get_return_type_result};

#[derive(PartialEq, Eq)]
enum SearchType
{
    Exact,
    RegExp,
    IgnoreCase
}

#[derive(FromMeta)]
struct StrategyTypeParams
{
    pub search : Option<String>
}

pub fn strategy_pattern_type_impl( args : TokenStream, item : TokenStream ) -> TokenStream
{
    let args = match get_attributes::<StrategyTypeParams>( args )
    {
        Ok( v ) => v,
        Err( e ) => return e
    };

    let item = parse_macro_input!( item as ItemStruct );

    let search : SearchType = match args.search {
        Some( v ) => match v.as_str()
        {
            "RegExp" => SearchType::RegExp,
            "IgnoreCase" => SearchType::IgnoreCase,
            "Exact" => SearchType::Exact,
            _ => return quote_spanned! {
                item.span() =>
                compile_error!( "search not valid. Expected Exact, IgnoreCase or RegExp" );
            }.into()
        },
        _ => SearchType::Exact
    };

    let attrs = &item.attrs;
    let ident = &item.ident;

    let fn_type: Type = get_fn_type( &item );

    let struct_impl = get_struct_impl( &item, &fn_type, search );
    
    quote! {
        #(#attrs)*
        pub struct #ident
        {
            pub fnc : #fn_type,
            pub key : &'static str
        }

        #struct_impl
        
        ::inventory::collect!( #ident );

    }.into()
}

fn initialize_map( search : &SearchType, struct_ident : &Ident, fn_type : Type ) -> proc_macro2::TokenStream
{
    if search == &SearchType::RegExp
    {
        return quote! {};
    }

    let key = if search == &SearchType::Exact
    {
        quote! { let key = fnc.key.to_string(); }
    }
    else
    {
        quote! { let key = fnc.key.to_uppercase(); }
    };

    quote! {
        fn map_fncs() -> &'static ::std::collections::HashMap<::std::string::String, #fn_type>
        {
            static __MAP_FNCS: ::std::sync::OnceLock<::std::collections::HashMap<::std::string::String, #fn_type>> = ::std::sync::OnceLock::new();
            __MAP_FNCS.get_or_init(|| #struct_ident::initialize_map_fncs() )
        }

        fn initialize_map_fncs() -> ::std::collections::HashMap<::std::string::String, #fn_type>
        {
            let mut map : ::std::collections::HashMap<::std::string::String, #fn_type> = ::std::collections::HashMap::new();

            for fnc in ::inventory::iter::<#struct_ident>
            {
                #key

                map.insert( key, fnc.fnc );
            }

            map
        }
    }
}

fn get_struct_impl( 
    item : &ItemStruct, 
    fn_type : &Type, 
    search : SearchType 
) -> proc_macro2::TokenStream
{
    if let syn::Type::BareFn( bare_fn_type) = fn_type
    {
        let ( _, names, full ) = get_params_bare_fn( bare_fn_type, Some( item.span() ) );

        let output = get_return_type_result( bare_fn_type.output.clone() );

        let ident = item.ident.clone();

        let body_fn = match search {
            SearchType::RegExp => get_body_fn_reg_exp( &names, &ident ),
            SearchType::IgnoreCase => get_body_fn_exact_or_ignore_case( &names, &ident, true ),
            _ => get_body_fn_exact_or_ignore_case( &names, &ident, false )
        };

        let map_impl = initialize_map(
            &search,
            &ident,
            fn_type.clone()
        );

        return quote! {
            
            impl #ident
            {
                pub fn exec( key : &str, #full ) #output
                {
                    #body_fn

                    ::std::result::Result::Err( format!( "Key '{}' not found", key ) )
                }

                #map_impl
            }
        }
    }

    quote_spanned! {
        item.span() =>
        compile_error!( "Expected TypeBareFn" );
    }
}

fn get_body_fn_exact_or_ignore_case( 
    names : &proc_macro2::TokenStream, 
    ident : &Ident, 
    ignore_case : bool 
) -> proc_macro2::TokenStream
{
    let upper_key = if ignore_case
    {
        quote! { &key.to_uppercase() }
    }
    else
    {
        quote! { &key.to_string() }
    };

    quote! {

        if #ident::map_fncs().contains_key( #upper_key )
        {
            return ::std::result::Result::Ok( ( #ident::map_fncs().get( #upper_key ).unwrap() )( #names ) )
        }
    }

    
}

fn get_body_fn_reg_exp( names : &proc_macro2::TokenStream, ident : &Ident ) -> proc_macro2::TokenStream
{
    quote! {

        for obj in ::inventory::iter::<#ident>
        {
            let re = format!( r"{}", obj.key );
            let re = ::regex::Regex::new( re.as_str() );

            if re.is_err()
            {
                return ::std::result::Result::Err( format!( "Invalid regex expression: {}", obj.key ) )
            }

            let re = re.unwrap();

            if re.is_match( key )
            {
                return ::std::result::Result::Ok( ( obj.fnc )( #names ) )
            }
        }
    }
}

fn get_fn_type( item : &ItemStruct ) -> Type
{
    item.fields.iter().map( | f | f.ty.clone() ).next().unwrap()
}