use quote::quote;
use syn::Local;

pub fn rust_code(rust_code: &Local) -> String {
    #[cfg(not(nightly))]
    {
        quote!(#rust_code).to_string()
    }

    #[cfg(nightly)]
    {
        let stream = quote!(#rust_code);
        let mut tokens = stream.clone().into_iter();
        let mut span = tokens.next().unwrap().span().unwrap();

        while let Some(token) = tokens.next() {
            span = span.join(token.span().unwrap()).unwrap();
        }

        span.source_text().unwrap_or_else(|| stream.to_string())
    }
}
