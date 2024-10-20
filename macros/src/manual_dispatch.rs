use super::*;

pub struct ManualDispatch(TokenStream);

impl Parse for ManualDispatch {
    fn parse(input: ParseStream) -> Result<Self> {
        let file = input.parse::<File>()?;

        let Some(tr) = file.items.iter().find_map(|e|match e {
            Item::Trait(tr) => Some(tr),
            _ => None
        }) else {
            return Err(input.error("trait declaration required"));
        };

        let Some(en) = file.items.iter().find_map(|e|match e {
            Item::Enum(en) => Some(en),
            _ => None
        }) else {
            return Err(input.error("enum declaration required"));
        };

        en.variants.iter().map(|e|{
            match &e.fields {
                Fields::Unnamed(f) if f.unnamed.len() == 1 => Ok(()),
                _ => Err(input.error("expect single unamed type"))
            }
        }).collect::<Result<Vec<_>>>()?;

        let trq = &tr.ident;
        let slq = &en.ident;

        let fnq = tr.items.iter().map(|e|{
            match e {
                TraitItem::Fn(tfn) => {
                    let sig = &tfn.sig;
                    let args = sig.inputs.iter().skip(1).map(|e|{
                        match e {
                            FnArg::Receiver(_) => Err(input.error("invalid self")),
                            FnArg::Typed(id) => match &*id.pat {
                                Pat::Ident(id) => Ok(&id.ident),
                                _ => Err(input.error("expect identifier"))
                            }
                        }
                    }).collect::<Result<Vec<_>>>()?;
                    let fn_name = &sig.ident;
                    let mt = en.variants.iter().map(|vr|{
                        let vr = &vr.ident;
                        quote! {
                            Self::#vr(vr) => #trq::#fn_name(vr,#(#args),*)
                        }
                    });
                    Ok(quote! { #sig { match self { #(#mt),* } } })
                }
                _ => Err(input.error("expect function trait definition"))
            }
        }).collect::<Result<Vec<_>>>()?;

        let im = quote! {
            impl #trq for #slq {
                #(#fnq)*
            }
        };

        Ok(Self(quote! {
            #tr
            #en
            #im
        }))
    }
}

impl ToTokens for ManualDispatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        ToTokens::to_tokens(&self.0, tokens);
    }
    fn to_token_stream(&self) -> TokenStream {
        ToTokens::to_token_stream(&self.0)
    }
}
