use phf::{phf_map, Map};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Lit, Meta, NestedMeta};

const SUPPORTED_ARCH: [&str; 2] = ["x86", "arm"];

enum SupportedArch {
    X86,
    Arm,
}

const hash: Map<&'static str, Map<u32, &'static str>> = phf_map! {
    "x86" => phf_map!{
        128 => "__m128i"
    },
    "arm" => phf_map!{
        128 => "uint8x16_t"
    },
};

#[proc_macro_attribute]
pub fn layout_member(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attrs as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let mut supported_arch: SupportedArch = SupportedArch::X86;
    if let NestedMeta::Meta(Meta::Path(p)) = &args[0] {
        let id = &p.segments[0].ident;

        supported_arch = match id.to_string().as_str() {
            "x86" => SupportedArch::X86,
            "arm" => SupportedArch::Arm,
            _ => panic!("Unsupported arch."),
        };
    }

    let mut ty_width: u32 = 0;
    for i in 1..args.len() {
        let attr = &args[i];
        if let NestedMeta::Lit(Lit::Int(ty)) = attr {
            let token = ty.base10_parse::<u32>().unwrap();
            if token < 128 || token % 128 != 0 || token > 512 {
                panic!("layout_member attrs should be either 8, 16, 32 or 64.");
            }

            ty_width = token;
        } else {
            panic!("layout_member attrs must be integers and could divide by eight.")
        }
    }

    eprintln!("{:?}", args);
    eprintln!("{:?}", input);

    let name = input.ident;

    (quote! {
        union #name {
            mm:
        };
    })
    .into()
}
