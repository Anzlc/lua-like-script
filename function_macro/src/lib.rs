use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, FnArg, ItemFn };

#[proc_macro_attribute]
pub fn intepreter_function(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let output = quote! {
        #item
    };
    output.into()
}
