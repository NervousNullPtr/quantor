#![doc(hidden)]
//! Internal procedural macros for `quantor`.
//!
//! Provides `#[function_name]`, which injects a local `const FUNCTION_NAME: &str`
//! into any function or method, containing the function’s name as a string.
//!
//! Not intended for public use; this crate is private and excluded from docs.
//!
//! ## Example
//! ```ignore
//! #[function_name]
//! fn run() {
//!     println!("{}", FUNCTION_NAME);
//! }
//! ```

use proc_macro::{TokenStream, TokenTree, Group, Delimiter};

#[doc(hidden)]
/// Injects a local `FUNCTION_NAME: &str` constant with the name of the current function.
///
/// Works on both free functions and `impl` methods. Zero runtime cost.  
/// Intended for internal diagnostics, logging, and tracing.
///
/// ## Example
/// ```ignore
/// #[function_name]
/// fn compute() {
///     println!("This is '{}'", FUNCTION_NAME);
/// }
/// ```
///
/// This macro is internal to the `quantor` crate and not publicly exported.
#[proc_macro_attribute]
pub fn function_name(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut output = Vec::new();

    while let Some(token) = tokens.next() {
        match &token {
            TokenTree::Ident(ident) if ident.to_string() == "fn" => {
                output.push(token.clone()); // push 'fn'

                if let Some(TokenTree::Ident(name)) = tokens.next() {
                    let fn_name_str = name.to_string();
                    output.push(TokenTree::Ident(name)); // push function name

                    // Copy everything until `{`
                    for next_token in tokens.by_ref() {
                        match &next_token {
                            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                                let original_body = group.stream();
                                let injected = format!(
                                    "const FUNCTION_NAME: &str = \"{}\"; {}",
                                    fn_name_str,
                                    original_body
                                );
                                let new_body = injected.parse::<TokenStream>().unwrap();
                                let new_group = Group::new(Delimiter::Brace, new_body);
                                output.push(TokenTree::Group(new_group));
                                break;
                            }
                            TokenTree::Group(group) => {
                                output.push(TokenTree::Group(Group::new(
                                    group.delimiter(),
                                    group.stream(),
                                )));
                            }
                            _ => output.push(next_token),
                        }
                    }
                    continue;
                }
            }
            _ => output.push(token),
        }
    }

    TokenStream::from_iter(output)
}