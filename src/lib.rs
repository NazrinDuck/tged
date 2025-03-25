extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use std::cmp::Ordering;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, Attribute, Data, DeriveInput, Error, Expr, ExprArray,
    ExprLit, Fields, Ident, Meta, UnOp,
};

///
///
/// pos: (u16, u16),
/// height: u16,
/// width: u16,
#[proc_macro_attribute]
pub fn view(attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!(&item);
    let input = parse_macro_input!(item as DeriveInput);
    let pos_fields = match parse_attrs(input.attrs) {
        Ok(init) => init,
        Err(err) => return Error::into_compile_error(err).into(),
    };
    let vis = input.vis;
    let name = input.ident;

    let old_fields;
    let default_fields;
    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let (name, ty) = (&f.ident, &f.ty);
                    quote! {#name: #ty,}
                });

                let default_recurse = fields.named.iter().map(|f| {
                    let (name, ty) = (&f.ident, &f.ty);
                    quote! {#name: #ty::default(),}
                });

                old_fields = quote! {
                    #(#recurse)*
                };
                default_fields = quote! {
                    #(#default_recurse)*
                }
            }
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let get_start = quote! {
        #[inline]
        fn get_start(&self, term: &Term) -> (u16, u16) {
            let (height, width) = (term.height, term.width);
            (self.start.0.unwrap(width), self.start.1.unwrap(height))
        }
    };

    let new = quote! {
        pub fn new() -> Self {
            #name {
                id: 0,
                #pos_fields
                #default_fields
            }
        }
    };

    let imple = quote! {
        impl #name {
            #new

            #get_start
        }
    };

    let expanded = quote! {
        use crate::view::ViewID;

        #[derive(Debug)]
        #vis struct #name {
            id: ViewID,
            start: (Pos, Pos),
            end: (Pos, Pos),
            #old_fields
        }

        #imple
    };
    TokenStream::from(expanded)
    //output
}

fn parse_attrs(attrs: Vec<Attribute>) -> syn::Result<TokenStream2> {
    let mut result: Vec<TokenStream2> = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("start") || attr.path().is_ident("end") {
            let ident = attr.path().get_ident().unwrap();
            let name = ident.to_string();
            match &attr.meta {
                Meta::NameValue(val) => {
                    let (x, y) = parse_tuple(&val.value)?;
                    let (x_pos, y_pos): (u16, u16) = (x.unsigned_abs() - 1, y.unsigned_abs() - 1);

                    let x = match x.cmp(&0) {
                        Ordering::Greater => quote! { Pos::Fixed( #x_pos ) },
                        Ordering::Equal => unreachable!(),
                        Ordering::Less => quote! { Pos::Opposite( #x_pos ) },
                    };

                    let y = match y.cmp(&0) {
                        Ordering::Greater => quote! { Pos::Fixed( #y_pos ) },
                        Ordering::Equal => unreachable!(),
                        Ordering::Less => quote! { Pos::Opposite( #y_pos ) },
                    };

                    result.push(quote! { #ident: (#x, #y), });
                }
                other => {
                    return Err(Error::new_spanned(
                        other,
                        format!("attribute `{name}` format wrong"),
                    ))
                }
            }
            continue;
        }

        return Err(Error::new_spanned(attr, "unrecognized ident"));
    }
    //let attr = parse_macro_input!(attr as Attribute);
    /*

    if attr.elems.len() != 4 {}

    dbg!(attr.elems.len());
    for ele in attr.elems {
        let a = if let Expr::Lit(lit) = ele {
            get_abs_val(lit)
        } else if let Expr::Unary(unary) = ele {
            if let Expr::Lit(lit) = *unary.expr {
                get_abs_val(lit)
            } else {
                panic!("not")
            }
        } else {
            panic!("not")
        };
        dbg!(a);
    }
    */

    Ok(quote! { #(#result)* })
}

fn parse_tuple(expr: &Expr) -> syn::Result<(i16, i16)> {
    if let Expr::Tuple(tuple) = expr {
        let elems = &tuple.elems;
        if elems.len() != 2 {
            Err(Error::new_spanned(
                elems,
                "attribute tuple must have 2 fields",
            ))
        } else {
            let mut iter = elems.iter().map(get_val);
            let x = iter.next().unwrap()?;
            let y = iter.next().unwrap()?;
            dbg!(x);
            dbg!(y);
            Ok((x, y))
        }
    } else {
        Err(Error::new_spanned(expr, "attribute value must be tuple"))
    }
}

fn get_val(expr: &Expr) -> syn::Result<i16> {
    if let Expr::Unary(unary) = expr {
        if let UnOp::Neg(_) = unary.op {
            let val = get_val(&unary.expr)?;
            Ok(-val)
        } else {
            Err(Error::new_spanned(
                unary.op,
                "unary op `Neg` is only allowed",
            ))
        }
    } else if let Expr::Lit(lit) = expr {
        match &lit.lit {
            syn::Lit::Int(val) => {
                let val = val.base10_parse::<i16>().unwrap();
                if val == 0 {
                    Err(Error::new_spanned(lit, "num can't be zero"))
                } else {
                    Ok(val)
                }
            }
            _ => Err(Error::new_spanned(lit, "must be literal num")),
        }
    } else {
        Err(Error::new_spanned(expr, "must be literal num"))
    }
}
