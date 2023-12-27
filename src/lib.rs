#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(__never_compiled)]
use crate as _doc;

extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::fmt::Arguments;

/// Creates a new error [`TokenStream`] from arguments (created by
/// [`format_args`]) and start/end [`Span`]s.
pub fn error_ts(msg: Arguments<'_>, start: Span, end: Span) -> TokenStream {
    let msg = match msg.as_str() {
        Some(str) => Literal::string(str),
        None => Literal::string(&msg.to_string()),
    };
    let msg = TokenStream::from(respan(msg, Span::call_site()));
    TokenStream::from_iter([
        respan(Ident::new("compile_error", start), start),
        respan(Punct::new('!', Spacing::Alone), Span::call_site()),
        respan(Group::new(Delimiter::Brace, msg), end),
    ])
}

/// see <https://internals.rust-lang.org/t/custom-error-diagnostics-with-procedural-macros-on-almost-stable-rust/8113>
fn respan<T: Into<TokenTree>>(t: T, span: Span) -> TokenTree {
    let mut t: TokenTree = t.into();
    t.set_span(span);
    t
}

/// Creates a new error [`TokenStream`]. One, two or no [`Span`]s for the
/// message are possible.
#[macro_export]
macro_rules! error {
    ($span:expr => $($arg:tt)*) => {{
        $crate::error_ts(format_args!($($arg)*), $span, $span)
    }};
    ($start:expr ; $end:expr => $($arg:tt)*) => {{
        $crate::error_ts(format_args!($($arg)*), $start, $end)
    }};
    ($($arg:tt)*) => {{
        $crate::error_ts(format_args!($($arg)*), proc_macro::Span::call_site(), proc_macro::Span::call_site())
    }};
}

/// Return early with an error.
///
/// This macro is equivalent to `return Err(error!(...))`.
///
/// The surrounding function’s or closure’s return value is required to be
/// `Result<_, proc_macro::TokenStream>`.
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {{
        return Err($crate::error!($($arg)*));
    }};
}
