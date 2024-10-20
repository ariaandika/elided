use super::*;

pub struct ManualDispatch(TokenStream);

impl Parse for ManualDispatch {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut file = input.parse::<File>()?;

        let Some(en) = file.items.iter().find_map(|e|match e {
            Item::Enum(en) => Some(en as *const ItemEnum),
            _ => None
        }) else {
            return Err(input.error("expected enum declaration"));
        };
        let en = unsafe { &*en };

        let Some(im) = file.items.iter_mut().find_map(|e|match e {
            Item::Impl(im) => Some(im as *mut ItemImpl),
            _ => None
        }) else {
            return Err(input.error("expected trait implementation"));
        };
        let im = unsafe { &mut *im };

        en.variants.iter().map(|e|{
            match &e.fields {
                Fields::Unnamed(f) if f.unnamed.len() == 1 => Ok(()),
                _ => Err(input.error("expect single unamed type"))
            }
        }).collect::<Result<Vec<_>>>()?;

        let Some((_, _tr, _)) = im.trait_.as_ref() else {
            return Err(input.error("expected trait implementation"))
        };

        let trq = _tr.clone();

        for item in &mut im.items {
            match item {
                ImplItem::Fn(item_fn)
                    if matches!(item_fn.sig.inputs.first(),Some(FnArg::Receiver(_))) =>
                {
                    let sig = &item_fn.sig;
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

                    item_fn.block = parse_quote!({
                        match self {
                            #(#mt),*
                        }
                    });
                }
                _ => {},
            }
        };

        Ok(Self(file.to_token_stream()))
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
