extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::Neg;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, Attribute, Data, DeriveInput, Error, Expr, ExprArray,
    ExprGroup, ExprLit, ExprTuple, Fields, Ident, Lit, Meta, Stmt, Type, UnOp,
};

///
///
/// pos: (u16, u16),
/// height: u16,
/// width: u16,
#[proc_macro_attribute]
pub fn view(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    dbg!(&attr);
    let attr = parse_macro_input!(attr as Lit);

    let name_field = match parse_view_attr(attr) {
        Ok(init) => init,
        Err(err) => return Error::into_compile_error(err).into(),
    };

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
                    let (vis, name, ty) = (&f.vis, &f.ident, &f.ty);
                    quote! {#vis #name: #ty,}
                });

                let default_recurse = fields.named.iter().map(|f| {
                    let (name, ty) = (&f.ident, &f.ty);
                    if let Type::Path(path) = ty {
                        if path.path.segments.len() == 1 {
                            let ident = &path.path.segments[0].ident;
                            quote! {#name: #ident::default(),}
                        } else {
                            Error::into_compile_error(Error::new_spanned(path, "something wrong"))
                        }
                    } else {
                        quote! {#name: #ty::default(),}
                    }
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

    let is_show = quote! {
        #[inline]
        fn is_show(&self) -> bool {
            self.show
        }
    };

    let get_name = quote! {
        #[inline]
        fn get_name(&self) -> &String {
            &self.name
        }
    };

    let is_lock = quote! {
        #[inline]
        fn is_lock(&self) -> bool {
            self.lock
        }
    };

    let is_silent = quote! {
        #[inline]
        fn is_silent(&self) -> bool {
            self.silent
        }
    };

    let get_start = quote! {
        #[inline]
        fn get_start(&self, term: &Term) -> (u16, u16) {
            let (height, width) = (term.height, term.width);
            (self.start.0.unwrap(width), self.start.1.unwrap(height))
        }
    };

    let get_end = quote! {
        #[inline]
        fn get_end(&self, term: &Term) -> (u16, u16) {
            let (height, width) = (term.height, term.width);
            (self.end.0.unwrap(width), self.end.1.unwrap(height))
        }
    };

    let refresh = quote! {
        #[inline]
        fn refresh(&self, term: &Term) {
            let (bclr, fclr) = (&self.bcolor, &self.fcolor);
            let (x_s, y_s) = self.get_start(term);
            let (x_e, y_e) = self.get_end(term);

            let width = (x_e - x_s) as usize;

            let space = " ".repeat(width).color(bclr, fclr);
            for i in y_s..y_e {
                print!("\x1b[{i};{x_s}H{}", space);
            }
        }
    };

    let resize = quote! {
        #[inline]
        fn resize(&mut self, dx_s: i16, dy_s: i16, dx_e: i16, dy_e: i16) {
            let (x_s, y_s) = self.start.clone();
            let (x_e, y_e) = self.end.clone();
            self.start.0 = x_s + dx_s;
            self.start.1 = y_s + dy_s;
            self.end.0 = x_e + dx_e;
            self.end.1 = y_e + dy_e;
        }
    };

    let new = quote! {
        #[inline]
        pub fn new() -> Self {
            #name {
                #name_field
                id: 0,
                lock: false,
                show: true,
                #pos_fields
                #default_fields
            }
        }
    };

    let impl_position = quote! {
        #[automatically_derived]
        impl Position for #name {
            #get_start

            #get_end

            #get_name

            #is_silent

            #is_lock

            #resize
        }
    };

    let implement = quote! {
        impl #name {
            #new

            #refresh
        }
    };

    let expanded = quote! {
        #[derive(Debug)]
        #vis struct #name {
            name: String,
            id: ViewID,
            start: (Pos, Pos),
            end: (Pos, Pos),
            silent: bool,
            lock: bool,
            show: bool,
            fcolor: Color,
            bcolor: Color,
            #old_fields
        }

        #implement

        #impl_position
    };
    TokenStream::from(expanded)
    //output
}

fn parse_view_attr(attr: Lit) -> syn::Result<TokenStream2> {
    match attr {
        Lit::Str(string) => Ok(quote! {name: String::from(#string),}),
        other => Err(Error::new_spanned(other, "expected string literal")),
    }
}

fn parse_attrs(attrs: Vec<Attribute>) -> syn::Result<TokenStream2> {
    let mut result: Vec<TokenStream2> = Vec::new();
    let (mut start, mut end): (bool, bool) = (false, false);
    let (mut bcolor, mut fcolor): (bool, bool) = (false, false);
    let mut silent: bool = false;
    for attr in attrs {
        if attr.path().is_ident("start") || attr.path().is_ident("end") {
            let ident = attr.path().get_ident().unwrap();
            let name = ident.to_string();
            match &name[..] {
                "start" => {
                    if start {
                        return Err(Error::new_spanned(
                            ident,
                            "arttribute `start` can only appear once",
                        ));
                    }
                    start = true;
                }
                "end" => {
                    if end {
                        return Err(Error::new_spanned(
                            ident,
                            "arttribute `end` can only appear once",
                        ));
                    }
                    end = true;
                }
                _ => unreachable!(),
            }
            match &attr.meta {
                Meta::NameValue(val) => {
                    let (x, y) = parse_pos_tuple(&val.value)?;
                    let (x_pos, y_pos): (u16, u16) = (x.unsigned_abs() - 1, y.unsigned_abs() - 1);

                    let x = match x.cmp(&0) {
                        Ordering::Greater => quote! { Pos::Fixed( #x_pos ) },
                        Ordering::Equal => {
                            return Err(Error::new_spanned(val, "num can't be zero"))
                        }
                        Ordering::Less => quote! { Pos::Opposite( #x_pos ) },
                    };

                    let y = match y.cmp(&0) {
                        Ordering::Greater => quote! { Pos::Fixed( #y_pos ) },
                        Ordering::Equal => {
                            return Err(Error::new_spanned(val, "num can't be zero"))
                        }
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

        if attr.path().is_ident("bcolor") || attr.path().is_ident("fcolor") {
            let ident = attr.path().get_ident().unwrap();
            let name = ident.to_string();
            match &name[..] {
                "bcolor" => {
                    if bcolor {
                        return Err(Error::new_spanned(
                            ident,
                            "arttribute `bcolor` can only appear once",
                        ));
                    }
                    bcolor = true;
                }
                "fcolor" => {
                    if fcolor {
                        return Err(Error::new_spanned(
                            ident,
                            "arttribute `fcolor` can only appear once",
                        ));
                    }
                    fcolor = true;
                }
                _ => unreachable!(),
            }

            match &attr.meta {
                Meta::NameValue(val) => {
                    let (r, g, b) = parse_color_tuple(&val.value)?;

                    result.push(quote! { #ident: Color::new(#r, #g, #b), });
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

        if attr.path().is_ident("silent") {
            let ident = attr.path().get_ident().unwrap();

            if silent {
                return Err(Error::new_spanned(
                    ident,
                    "arttribute `silent` can only appear once",
                ));
            }
            silent = true;

            result.push(quote! { #ident: true, });

            continue;
        }

        return Err(Error::new_spanned(attr, "unrecognized ident"));
    }
    // start/end part is necessary
    if !start {
        return Err(Error::new_spanned(0, "expected `start`"));
    }

    if !end {
        return Err(Error::new_spanned(0, "expected `end`"));
    }

    if !silent {
        result.push(quote! { silent: false, });
    }

    // bcolor/fcolor part can be left empty
    if !bcolor {
        result.push(quote! { bcolor: Color::new(0, 0, 0), });
    }

    if !fcolor {
        result.push(quote! { fcolor: Color::new(0xff, 0xff, 0xff), });
    }

    Ok(quote! { #(#result)* })
}

fn parse_pos_tuple(expr: &Expr) -> syn::Result<(i16, i16)> {
    if let Expr::Tuple(tuple) = expr {
        let elems = &tuple.elems;
        if elems.len() != 2 {
            Err(Error::new_spanned(
                elems,
                "attribute `pos` must have only 2 fields",
            ))
        } else {
            let mut iter = elems.iter().map(get_val);
            let x = iter.next().unwrap()?;
            let y = iter.next().unwrap()?;
            Ok((x, y))
        }
    } else {
        Err(Error::new_spanned(
            expr,
            "attribute `pos` value must be tuple",
        ))
    }
}

fn parse_color_tuple(expr: &Expr) -> syn::Result<(u8, u8, u8)> {
    if let Expr::Tuple(tuple) = expr {
        let elems = &tuple.elems;
        if elems.len() != 3 {
            Err(Error::new_spanned(
                elems,
                "attribute `color` must have only 3 fields",
            ))
        } else {
            let mut iter = elems.iter().map(get_unsigned_val);
            let r = iter.next().unwrap()?;
            let g = iter.next().unwrap()?;
            let b = iter.next().unwrap()?;
            Ok((r, g, b))
        }
    } else {
        Err(Error::new_spanned(
            expr,
            "attribute `color` value must be tuple",
        ))
    }
}

fn get_val<T>(expr: &Expr) -> syn::Result<T>
where
    T: FromStr + Neg<Output = T>,
    T::Err: Display,
{
    if let Expr::Unary(unary) = expr {
        if let UnOp::Neg(_) = unary.op {
            let val = get_val::<T>(&unary.expr)?;
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
                let val = val.base10_parse::<T>().unwrap();
                Ok(val)
            }
            _ => Err(Error::new_spanned(lit, "must be literal number")),
        }
    } else {
        Err(Error::new_spanned(expr, "must be literal number"))
    }
}

fn get_unsigned_val<T>(expr: &Expr) -> syn::Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    if let Expr::Lit(lit) = expr {
        match &lit.lit {
            syn::Lit::Int(val) => {
                let val = val.base10_parse::<T>().unwrap();
                Ok(val)
            }
            _ => Err(Error::new_spanned(lit, "must be literal number")),
        }
    } else {
        Err(Error::new_spanned(expr, "`Unary Op` is not allowed"))
    }
}
