use proc_macro2::Span as Span2;
use syn::{
    punctuated::Punctuated, token::Colon2, Ident, Path, PathArguments, PathSegment, Token, Type,
    TypePath,
};

use std::sync::atomic::{AtomicU16, Ordering};

macro_rules! parse_func {
    ($name:ident, $func:ident, $tokens:ident) => {
        if $name.is_some() {
            return Err(::syn::Error::new(
                $func.span().unwrap().into(),
                &format!("Method `{}` defined multiple times", stringify!($name)),
            ));
        }
        $name = Some($tokens);
    };
}

pub(crate) fn idents_to_snake_case<'a, I: Iterator<Item = &'a Ident>>(
    idents: I,
    span: Span2,
) -> Ident {
    static COUNTER: AtomicU16 = AtomicU16::new(0);
    let val = COUNTER.fetch_add(1, Ordering::Relaxed);
    let index_str = val.to_string();

    let segements: Vec<String> = idents
        .map(|ident| ident.to_string().to_lowercase())
        .collect();
    let length: usize =
        segements.iter().map(|seg| seg.len() + 1).sum::<usize>() + index_str.len() + 1;
    let mut name: String = String::with_capacity(length);

    for seg in &segements {
        name.push('_');
        name.push_str(seg);
    }
    name.push('_');
    name.push_str(&index_str);

    Ident::new(&name, span)
}

pub(crate) fn default_relm4_path() -> Path {
    let relm4_path_segment = PathSegment {
        ident: Ident::new("relm4", Span2::call_site()),
        arguments: PathArguments::None,
    };

    let mut relm4_segments: Punctuated<PathSegment, Colon2> = Punctuated::new();
    relm4_segments.push(relm4_path_segment);

    Path {
        leading_colon: Some(Token![::](Span2::call_site())),
        segments: relm4_segments,
    }
}

pub(crate) fn self_type() -> Type {
    // Create a `Self` type for the model
    let path_segment = PathSegment {
        ident: Ident::new("Self", Span2::call_site()),
        arguments: PathArguments::default(),
    };
    let mut segments = Punctuated::new();
    segments.push(path_segment);
    let path = Path {
        segments,
        leading_colon: None,
    };
    TypePath { path, qself: None }.into()
}
