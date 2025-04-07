use super::*;

pub struct FromUnit {
    this: Ident,
    units: Vec<UnitKind>,
}

enum UnitKind {
    Single(Ident,Type),
    Multi(Ident,Vec<Type>),
    SingleNamed(Ident,Ident,Type),
    MultiNamed(Ident,Vec<(Ident,Type)>),
}

impl Parse for FromUnit {
    fn parse(input: ParseStream) -> Result<Self> {
        let en = input.parse::<ItemEnum>()?;
        let mut units = vec![];

        for variant in en.variants {
            match variant.fields {
                Fields::Unnamed(unnamed) => {
                    if unnamed.unnamed.len() == 1 {
                        units.push(UnitKind::Single(
                            variant.ident.clone(),
                            unnamed.unnamed.first().unwrap().ty.clone(),
                        ));
                    } else {
                        units.push(UnitKind::Multi(
                            variant.ident.clone(),
                            unnamed.unnamed.iter().map(|field|{
                                field.ty.clone()
                            }).collect(),
                        ));
                    }
                }
                Fields::Named(named) => {
                    if named.named.len() == 1 {
                        let field = named.named.first().unwrap().clone();
                        units.push(UnitKind::SingleNamed(
                            variant.ident.clone(),
                            field.ident.unwrap(),
                            field.ty.clone(),
                        ));
                    } else {
                        units.push(UnitKind::MultiNamed(
                            variant.ident.clone(),
                            named.named.into_iter().map(|field|{
                                (
                                    field.ident.unwrap(),
                                    field.ty,
                                )
                            }).collect(),
                        ));
                    }
                }
                Fields::Unit => {}
            }
        }

        Ok(Self {
            this: en.ident.clone(),
            units,
        })
    }
}

impl ToTokens for FromUnit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FromUnit { this, units } = self;
        for unit in units {
            match unit {
                UnitKind::Single(vr,ty) => {
                    tokens.extend(quote! {
                        impl From<#ty> for #this {
                            fn from(value: #ty) -> Self {
                                Self::#vr(value)
                            }
                        }
                    });
                }
                UnitKind::Multi(vr,tys) => {
                    let params = (0..tys.len()).map(|i|format_ident!("_{i}"));
                    let params = quote! { #(#params),* };
                    let gens = quote! { #(#tys),* };
                    tokens.extend(quote! {
                        impl From<(#gens)> for #this {
                            fn from((#params): (#gens)) -> Self {
                                Self::#vr(#params)
                            }
                        }
                    });
                }
                UnitKind::SingleNamed(vr, id, ty) => {
                    tokens.extend(quote! {
                        impl From<#ty> for #this {
                            fn from(value: #ty) -> Self {
                                Self::#vr {
                                    #id: value
                                }
                            }
                        }
                    });
                }
                UnitKind::MultiNamed(vr, tys) => {
                    let params = (0..tys.len()).map(|i|format_ident!("_{i}"));
                    let params2 = params.clone();
                    let paramsq = quote! { #(#params),* };
                    let gens = tys.iter().map(|e|&e.1);
                    let gens = quote! { #(#gens),* };
                    let ids = tys.iter().zip(params2).map(|((id,_),val)|{
                        quote! { #id: #val }
                    });
                    let ids = quote! { #(#ids),* };
                    tokens.extend(quote! {
                        impl From<(#gens)> for #this {
                            fn from((#paramsq): (#gens)) -> Self {
                                Self::#vr { #ids }
                            }
                        }
                    });
                }
            }
        }
    }
}

