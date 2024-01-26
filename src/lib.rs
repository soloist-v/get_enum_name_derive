use syn::{parse_macro_input, DeriveInput, Data};
use quote::{quote};

#[proc_macro_derive(GetName)]
pub fn get_enum_name_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let mut names = vec![];
    let struct_name = input.ident.clone();
    match &mut input.data {
        Data::Enum(data) => {
            for i in data.variants.iter() {
                let name = &i.ident;
                let name_str = name.to_string();
                let code = quote! {
                    Self::#name(_) => #name_str.to_owned()
                };
                names.push(code);
            }
        }
        _ => panic!("GetName can only be derived for enums"),
    };
    let res = quote! {
        impl #struct_name {
            pub fn get_name(&self) -> String{
                match self{
                    #(#names, )*
                }
            }
        }
    };
    res.into()
}
