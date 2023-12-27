# Simple `proc_macro` error

[![Github](https://img.shields.io/badge/github-Qwox0/simple__proc__macro__error-blue?style=flat&logo=github)](https://github.com/Qwox0/simple_proc_macro_error)

Create simple error messages in Procedural Macros.

# Example

```rust,ignore
#[proc_macro]
pub fn my_macro(ts: TokenStream) -> TokenStream {
    let Some(tt) = ts.into_iter().next() else {
        return error!("Macro input is empty")
    };
    error!(tt.span() => "This macro is not implemented but here is the first token: {tt}")
}
```
