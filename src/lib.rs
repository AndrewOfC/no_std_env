use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use std::path::Path;
use std::collections::HashMap;
use std::{fs, path};

#[proc_macro]
pub fn no_std_env(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as LitStr);
    let value = input.value();
    let cwd = match std::env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("<unknown>")
    };
    /*
     * check .env file
     */
    let p = Path::new(".env") ;
    if p.exists() {
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
    else {
        let err = format!("failed to find .env file in {:?}", path::absolute(p).unwrap() );
        return quote! { compile_error!(#err)}.into()
    }

    /*
     * check environment variable
     */
    let env_value = match std::env::var(&value) {
        Ok(val) => val,
        Err(_) => {
            let s = format!("environment variable {} not set in {}", value, cwd);
            return quote! { compile_error!(#s)}.into() ;
        }
    } ;

    quote! {
        #env_value
    }.into()
}