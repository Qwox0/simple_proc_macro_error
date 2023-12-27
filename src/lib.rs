#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{fmt::Arguments, ops::Range};

/// Wrapper around [`Range<Span>`].
pub struct SpanRange(pub Range<Span>);

impl From<Range<Span>> for SpanRange {
    fn from(range: Range<Span>) -> Self {
        Self(range)
    }
}

impl From<Span> for SpanRange {
    fn from(span: Span) -> Self {
        Self(span..span)
    }
}

/// Creates a new error [`TokenStream`] from arguments (created by
/// [`format_args`]) and a [`Range`] of [`Span`]s.
pub fn error_ts(msg: Arguments<'_>, span: SpanRange) -> TokenStream {
    let msg = match msg.as_str() {
        Some(str) => Literal::string(str),
        None => Literal::string(&msg.to_string()),
    };
    let msg = TokenStream::from(respan(msg, Span::call_site()));
    TokenStream::from_iter([
        respan(Ident::new("compile_error", span.0.start), span.0.start),
        respan(Punct::new('!', Spacing::Alone), Span::call_site()),
        respan(Group::new(Delimiter::Brace, msg), span.0.end),
    ])
}

/// see <https://internals.rust-lang.org/t/custom-error-diagnostics-with-procedural-macros-on-almost-stable-rust/8113>
fn respan<T: Into<TokenTree>>(t: T, span: Span) -> TokenTree {
    let mut t: TokenTree = t.into();
    t.set_span(span);
    t
}

/// Creates a new proc macro compile time error as a
/// [`proc_macro::TokenStream`].
///
/// The [`std::fmt`] syntax is used for the massage.
///
/// Optionally a [`Span`] or a [`Range<Span>`] can be provided (Format:
/// `error!(span => "Message")`)
///
/// # Example
///
/// ```rust,ignore
/// # extern crate proc_macro;
/// # use proc_macro::TokenStream;
/// # use simple_proc_macro_error::error;
/// #[proc_macro]
/// pub fn my_macro(ts: TokenStream) -> TokenStream {
///     let Some(tt) = ts.into_iter().next() else {
///         return error!("Macro input is empty")
///     };
///     error!(tt.span() => "This macro is not implemented but here is the first token: {tt}")
/// }
/// ```
#[macro_export]
macro_rules! error {
    ($span:expr => $($arg:tt)*) => {{
        $crate::error_ts(format_args!($($arg)*), $crate::SpanRange::from($span))
    }};
    ($($arg:tt)*) => {{
        $crate::error_ts(format_args!($($arg)*), $crate::SpanRange::from(proc_macro::Span::call_site()))
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
