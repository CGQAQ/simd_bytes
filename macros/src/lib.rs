use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn layout_member(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attrs as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let mut tys: Vec<u32> = vec![];
    for attr in args {
        if let NestedMeta::Lit(Lit::Int(ty)) = attr {
            let token = ty.base10_parse::<u32>().unwrap();
            if token < 8 || token % 8 != 0 || token > 64 {
                panic!("layout_member attrs should be either 8, 16, 32 or 64.");
            }

            tys.push(token);
        } else {
            panic!("layout_member attrs must be integers and could divide by eight.")
        }
    }

    eprintln!("{:?}", tys);
    eprintln!("{:?}", input);
    TokenStream::new()
}
