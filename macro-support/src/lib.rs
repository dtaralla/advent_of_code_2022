use macro_state::{proc_append_state, proc_read_state_vec};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::path::PathBuf;
use std::str::FromStr;
use syn::__private::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitInt, Token};

struct AOCArgs {
    day: u8,
    year: u16,
}

impl Parse for AOCArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ints = Punctuated::<LitInt, Token![,]>::parse_terminated(input)?;
        if ints.len() != 2 {
            Err(input.error("Expected day and year like 1, 2022"))
        } else {
            let day = ints[0].base10_parse::<u8>()?;
            if !(1..=25).contains(&day) {
                return Err(syn::Error::new(
                    ints[0].span(),
                    "Day must be from 1 to 31 included.",
                ));
            }

            let year = ints[1].base10_parse::<u16>()?;
            if year < 1 {
                return Err(syn::Error::new(
                    ints[1].span(),
                    "Year must be greater than 0.",
                ));
            }

            Ok(Self { day, year })
        }
    }
}

#[proc_macro_attribute]
pub fn advent_of_code(args: TokenStream, ast: TokenStream) -> TokenStream {
    let args_clone = args.clone();
    let args = parse_macro_input!(args_clone as AOCArgs);
    let ast = parse_macro_input!(ast as syn::ItemStruct);

    let struct_name = &ast.ident;
    let day = &args.day;
    let year = &args.year;

    if !PathBuf::from(format!("src/aoc_{}_{}.rs", year, day)).exists() {
        let msg = format!(
            "aoc_{}_{} module does not exist; create it and implement:\n\t\
            pub fn run(input: &str) -> anyhow::Result<String> {{}}\n\t\
            pub fn run2(input: &str) -> anyhow::Result<String> {{}}",
            year, day
        );
        return quote_spanned! {
            Span::call_site()=>
            compile_error!(#msg);
        }
        .into();
    }

    proc_append_state(
        "exercises",
        &format!("{}:{}:{}", args.year, args.day, ast.ident),
    )
    .unwrap();

    let struct_name_str = &ast.ident.to_string();
    let struct_vis = &ast.vis;
    let usemod_ts = TokenStream::from_str(&format!("use aoc_{}_{}::*;", year, day)).unwrap();
    let usemod = parse_macro_input!(usemod_ts as syn::ItemUse);
    TokenStream::from(quote! {
        #struct_vis struct #struct_name;

        impl aoc_core::AdventOfCodeRunnable for #struct_name {
            fn matches(&self, day: u8, year: u16) -> bool {
                day == #day && year == #year
            }

            fn get_input(&self, oauth_session_id: &str, is_second: bool) -> anyhow::Result<String> {
                if is_second {
                    aoc_core::get_input(oauth_session_id, #day, #year, false)
                }
                else {
                    aoc_core::get_input(oauth_session_id, #day, #year, false)
                }
            }

            fn run(&self, input: &str) -> anyhow::Result<String> {
                #usemod
                run(input)
            }

            fn run2(&self, input: &str) -> anyhow::Result<String> {
                #usemod
                run2(input)
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Dec {}, {} - {}", #day, #year, #struct_name_str)
            }
        }
    })
}

/// This MUST come AFTER you have defined your implemented exercises.
///
/// Example:
///
/// ```rust,ignore
/// #[advent_of_code(1, 2022)]
/// pub struct CalorieCounting;
///
/// #[advent_of_code(2, 2022)]
/// struct RockPaperScissors;
///
/// declare_exercise_modules!();
///
/// int main() -> anyhow::Result<()> {
///     ...
/// }
/// ```
#[proc_macro]
pub fn declare_exercise_modules(_input: TokenStream) -> TokenStream {
    let mut items = proc_read_state_vec("exercises");
    items.sort();

    // let mut es: Vec<Box<dyn AdventOfCodeRunnable>> = vec![];
    let mut mods: Vec<syn::ItemMod> = vec![];
    for item in items.iter() {
        let desc: Vec<&str> = item.split(':').collect();
        let ts = TokenStream::from_str(&format!("mod aoc_{}_{};", desc[0], desc[1])).unwrap();
        mods.push(parse_macro_input!(ts as syn::ItemMod));
    }
    quote!(
        #(#mods)*
    )
    .into()
}

#[proc_macro]
pub fn get_available_exercises(_input: TokenStream) -> TokenStream {
    let mut items = proc_read_state_vec("exercises");
    items.sort();

    let mut es: Vec<syn::Expr> = vec![];
    for item in items.iter() {
        let desc: Vec<&str> = item.split(':').collect();
        let ts = TokenStream::from_str(desc[2]).unwrap();
        es.push(parse_macro_input!(ts as syn::Expr));
    }

    quote!(
        {
            use aoc_core::AdventOfCodeRunnable;
            let mut exercises: Vec<Box<dyn AdventOfCodeRunnable>> = vec![];
            #(
                exercises.push(Box::new(#es));
            )*
            exercises
        }
    )
    .into()
}
