use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItemFn};

///Generate a function "boxed" that will return a Box of the return value of the function passed to the macro.
///
/// # Example
/// impl Value {
///   #[boxed]
///   fn new(x: i32) -> Value {
///       //implementation
///   }
/// }
///
/// ## Output
/// impl Value {
///   fn new(x: i32) -> Value {
///       //implementation
///   }
///   fn boxed(x: i32) -> Box<Value> {
///      Box::new(Self::new(x))
///   }
/// }
#[proc_macro_attribute]
pub fn boxed(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplItemFn);
    let name = &input.sig.ident;
    if name != "new" {
        return syn::Error::new_spanned(input, "Function must be named 'new'")
            .to_compile_error()
            .into();
    }
    let pub_vis = &input.vis;
    let inputs = &input.sig.inputs;
    let arguments_items = {
        let mut arguments = Vec::new();
        for input in inputs {
            match input {
                syn::FnArg::Receiver(_) => {}
                syn::FnArg::Typed(pat) => {
                    arguments.push(pat.pat.clone());
                }
            }
        }
        arguments
    };
    let res = quote! {
        #input
        #pub_vis fn boxed(#inputs) -> Box<Self> {
            Box::new(Self::new(#(#arguments_items),*))
        }
    };
    res.into()
}
