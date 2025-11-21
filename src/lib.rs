use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use std::path::Path;
use std::collections::HashMap;
use std::fs;

#[proc_macro]
pub fn no_std_env(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as LitStr);
    let value = input.value();

    /*
     * check .env file
     */
    if Path::new(".env").exists() {
        let mut env_map = HashMap::new();
        let contents  = match fs::read_to_string(".env") {
            Ok(contents) => contents,
            Err(_) => return quote! { compile_error!("failed to read .env file")}.into()
        } ;

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                env_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        if let Some(env_value) = env_map.get(&value) {
            return quote! { #env_value }.into();
        }
    }

    /*
     * check environment variable
     */
    let env_value = match std::env::var(&value) {
        Ok(val) => val,
        Err(_) => {
            return quote! { compile_error!("environment variable {} not set", stringify!(#value))}.into() ;
        }
    } ;

    quote! {
        #env_value
    }.into()
}