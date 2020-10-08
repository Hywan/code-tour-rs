#![cfg_attr(nightly, feature(proc_macro_span))]

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
#[cfg(feature = "interactive")]
use syn::Block;
use syn::{parse, AttrStyle, Ident, ItemFn, Pat, Path, Stmt};

mod colours;
mod format;

#[proc_macro_attribute]
pub fn code_tour(
    _attributes: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if let Ok(mut function) = parse::<ItemFn>(input.clone()) {
        let block_statements = &function.block.stmts;
        let mut statements = Vec::with_capacity(block_statements.len());

        for statement in block_statements.into_iter() {
            // That's a `let` binding.
            if let Stmt::Local(local) = statement {
                // Extract the local pattern.
                if let Pat::Ident(local_pat) = &local.pat {
                    // There is attributes, which are normally `///`
                    // (outer doc) or `/** */` (outer block) comments
                    // before the `let` binding.
                    if !local.attrs.is_empty() {
                        // Write `println!("{}", ident_comment)`.
                        {
                            let formatted_comment = local
                                .attrs
                                .iter()
                                .filter_map(|attribute| match (attribute.style, &attribute.path) {
                                    (
                                        AttrStyle::Outer,
                                        Path {
                                            leading_colon: None,
                                            segments,
                                        },
                                    ) if !segments.is_empty()
                                        && segments[0].ident
                                            == Ident::new("doc", Span::call_site()) =>
                                    {
                                        if let Some(TokenTree::Literal(literal)) =
                                            attribute.tokens.clone().into_iter().nth(1)
                                        {
                                            let literal_string =
                                                literal.to_string().replace("\\\'", "'");

                                            Some(format!(
                                                "▍{:<80}",
                                                &literal_string[1..literal_string.len() - 1]
                                            ))
                                        } else {
                                            None
                                        }
                                    }

                                    _ => None,
                                })
                                .map(colours::comment)
                                .collect::<Vec<String>>()
                                .join("\n ");

                            let empty_line = colours::comment(format!("▍{:<80}", " "));
                            let formatted_comment = format!(
                                "\n {empty}\n {comment}\n {empty}\n",
                                empty = empty_line,
                                comment = formatted_comment
                            );

                            statements.push(println(quote!("{}", #formatted_comment)));
                        }

                        let mut local_without_attrs = local.clone();
                        local_without_attrs.attrs = vec![];

                        // Write `println!("{}", stringify!(<local>))`.
                        {
                            let statement =
                                colours::statement(format::rust_code(&local_without_attrs));

                            statements.push(println(quote!(" {}\n", #statement)));
                        }

                        // Write the original statement, without the documentation.
                        {
                            statements.push(Stmt::Local(local_without_attrs));
                        }

                        // Write `println!("{:?}", <ident>)`.
                        {
                            statements.push(println({
                                let ident = &local_pat.ident;

                                quote!(
                                    " ◀︎    {}\n\n",
                                    format!("{:#?}", #ident).replace("\n", "\n ▐    ")
                                )
                            }));
                        }

                        // Insert “Press Enter to continue…”
                        #[cfg(feature = "interactive")]
                        {
                            let stream = quote!({
                                {
                                    use std::io::BufRead;

                                    let mut line = String::new();
                                    let stdin = ::std::io::stdin();

                                    println!(
                                        "\n(Press Enter to continue, otherwise Ctrl-C to exit).\n\n"
                                    );

                                    stdin
                                        .lock()
                                        .read_line(&mut line)
                                        .expect("Failed to read a line from the user.");
                                }
                            });

                            let block = parse::<Block>(stream.into()).unwrap();

                            for statement in block.stmts {
                                statements.push(statement);
                            }
                        }

                        continue;
                    }
                }
            }

            statements.push(statement.clone());
        }

        function.block.stmts = statements;

        quote!(#function).to_token_stream().into()
    } else {
        let span = TokenStream::from(input).into_iter().nth(0).unwrap().span();
        quote_spanned!(span => compile_error!("`code_tour` works on functions only")).into()
    }
}

fn println(tokens: TokenStream) -> Stmt {
    let stream = quote!(if ::std::env::var("CODE_TOUR_QUIET").is_err() {
        println!(#tokens);
    });
    let semi = parse::<Stmt>(stream.into()).unwrap();

    semi
}
