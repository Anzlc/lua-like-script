use std::any::Any;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, FnArg, ItemFn, Pat };

#[proc_macro_attribute]
pub fn interpreter_function(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let sig = item.sig;
    let name = sig.ident;
    let return_type = sig.output;
    let vis = item.vis;
    let inputs = sig.inputs;

    let mut arg_conversion = Vec::new();
    let mut identifiers = Vec::new();

    for (i, arg) in inputs.clone().into_iter().enumerate() {
        if let FnArg::Typed(pattern) = arg {
            let ident = match *pattern.pat {
                Pat::Ident(i) => i,
                _ => panic!("Expected identifier"),
            };

            let ty = pattern.ty;
            let conv =
                // Ty ... function input type
                quote! {
                    let #ident: #ty;
                

                    #ident = args[#i].clone().into();
                
            };
            identifiers.push(ident);

            arg_conversion.push(conv);
        }
    }

    let body = item.block;
    let inputs_len = inputs.len();
    let output =
        quote! {
        #vis fn #name(gc: &mut GarbageCollector, args: &[Value]) -> Value {
            if args.len() != #inputs_len {
                panic!("Expected {} args got {}", #inputs_len, args.len())
            }

            #(#arg_conversion)*

            fn original(#inputs) #return_type {
                #body
            }

            let out = original(#(#identifiers)*);


            return Value::from(out);

        }
    };

    output.into()
}
