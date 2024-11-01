use super::*;

pub struct StdError {
    this: Ident,
    units: Vec<(Ident,Type)>
}

impl Parse for StdError {
    fn parse(input: ParseStream) -> Result<Self> {
        let en = input.parse::<ItemEnum>()?;
        let mut units = vec![];

        for variant in en.variants {
            match variant.fields {
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    units.push((
                        variant.ident.clone(),
                        unnamed.unnamed.first().unwrap().ty.clone(),
                    ));
                }
                _ => return Err(input.error("expect single unamed variants")),
            }
        }

        Ok(Self {
            this: en.ident.clone(),
            units
        })
    }
}

impl ToTokens for StdError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StdError { this, units } = self;

        let mut debug = TokenStream::new();
        let mut display = TokenStream::new();

        for (variant, target) in units {
            tokens.extend(quote! {
                impl From<#target> for #this {
                    fn from(value: #target) -> Self {
                        #this::#variant(value)
                    }
                }
            });

            let variant_name = variant.to_string();

            debug.extend(quote! {
                Self::#variant(err) => f.debug_tuple(#variant_name)
                    .field(err)
                    .finish(),
            });

            display.extend(quote! {
                Self::#variant(err) => {
                    write!(f, "{}: ", #variant_name)?;
                    std::fmt::Display::fmt(err, f)
                },
            });
        }

        tokens.extend(quote! {
            impl std::error::Error for #this { }

            impl std::fmt::Debug for #this {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self { #debug }
                }
            }
            impl std::fmt::Display for #this {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self { #display }
                }
            }
        });

    }
}

