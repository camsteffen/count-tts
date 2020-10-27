#![no_std]

use proc_macro::{Literal, TokenStream, TokenTree};

/// Expands to the number of token trees in the macro arguments
///
/// Example:
///
/// ```
/// use count_tts::count_tts;
///
/// assert_eq!(count_tts!(), 0);
/// assert_eq!(count_tts!(a b c), 3);
/// assert_eq!(count_tts!(a (b c)), 2);
/// assert_eq!(count_tts!(a, b, c), 5);
/// ```
#[proc_macro]
pub fn count_tts(input: TokenStream) -> TokenStream {
    let count = input.into_iter().count();
    TokenTree::Literal(Literal::usize_unsuffixed(count)).into()
}
