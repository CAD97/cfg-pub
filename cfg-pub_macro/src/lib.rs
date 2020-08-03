use {
    proc_macro2::Span,
    proc_macro2::TokenStream,
    quote::ToTokens,
    std::{mem, panic::catch_unwind},
    syn::Item,
};

pegcel_macros::pegcel_syn! {
    use syn

    CfgPubChain: {
        head: CfgIf
        mid: { &"else" CfgElseIf }*
        tail: { &"else" CfgElse }?
    }

    CfgIf: {
        "if"
        attrs: OuterAttributes
        vis: syn::Visibility
    }

    CfgElseIf: {
        "else" "if"
        attrs: OuterAttributes
        vis: syn::Visibility
    }

    CfgElse: {
        "else"
        vis: syn::Visibility
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OuterAttributes(Vec<syn::Attribute>);
impl syn::parse::Parse for OuterAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::Attribute::parse_outer(input).map(OuterAttributes)
    }
}
impl ToTokens for OuterAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.iter().for_each(|x| x.to_tokens(tokens))
    }
}
impl IntoIterator for OuterAttributes {
    type Item = syn::Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

macro_rules! take_attrs {
    ($item:expr) => {
        take_attrs! { $item; [
            Const, Enum, ExternCrate, Fn, Macro2, Mod, Static,
            Struct, Trait, TraitAlias, Type, Union, Use,
        ] }
    };
    ($item:expr => $place:expr) => {
        take_attrs! { $item => $place; [
            Const, Enum, ExternCrate, Fn, Macro2, Mod, Static,
            Struct, Trait, TraitAlias, Type, Union, Use,
        ] }
    };
    ($item:expr; [$($variant:ident),* $(,)?]) => {
        match &mut $item {
            $(Item::$variant(item) => mem::take(&mut item.attrs),)*
            item => return Err(syn::Error::new_spanned(item, "unimplemented item type for `#[cfg_pub]`")),
        }
    };
    ($item:expr => $place:expr; [$($variant:ident),* $(,)?]) => {
        match &mut $item {
            $(Item::$variant(item) => $place = mem::take(&mut item.attrs),)*
            item => return Err(syn::Error::new_spanned(item, "unimplemented item type for `#[cfg_pub]`")),
        }
    };
}

macro_rules! place_vis {
    ($vis:expr => $item:expr) => {
        place_vis! { $vis => $item; [
            Const, Enum, ExternCrate, Fn, Macro2, Mod, Static,
            Struct, Trait, TraitAlias, Type, Union, Use,
        ] }
    };
    ($vis:expr => $item:expr; [$($variant:ident),* $(,)?]) => {
        match &mut $item {
            $(Item::$variant(item) => item.vis = $vis,)*
            item => return Err(syn::Error::new_spanned(item, "unimplemented item type for `#[cfg_pub]`")),
        }
    };
}

macro_rules! place_attrs {
    ($attrs:expr => $item:expr) => {
        place_attrs! { $attrs => $item; [
            Const, Enum, ExternCrate, Fn, Macro2, Mod, Static,
            Struct, Trait, TraitAlias, Type, Union, Use,
        ] }
    };
    ($attrs:expr => $item:expr; [$($variant:ident),* $(,)?]) => {
        match &mut $item {
            $(Item::$variant(item) => item.attrs = $attrs,)*
            item => return Err(syn::Error::new_spanned(item, "unimplemented item type for `#[cfg_pub]`")),
        }
    };
}

#[no_mangle]
pub extern "C" fn cfg_pub(attr: TokenStream, item: TokenStream) -> TokenStream {
    fn cfg_pub(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
        let cfg_pub: CfgPubChain = syn::parse2(attr)?;
        let mut item: syn::Item = syn::parse2(item)?;

        let mut out = TokenStream::new();
        let mut attrs = take_attrs!(item);
        let base_attrs_count = attrs.len();

        // if
        attrs.extend(cfg_pub.head.attrs);
        place_vis!(cfg_pub.head.vis => item);
        place_attrs!(mem::take(&mut attrs) => item);
        item.to_tokens(&mut out);
        take_attrs!(item => attrs);
        attrs.truncate(base_attrs_count);

        // elif
        for elif in cfg_pub.mid {
            attrs.extend(elif.attrs);
            place_vis!(elif.vis => item);
            place_attrs!(mem::take(&mut attrs) => item);
            item.to_tokens(&mut out);
            take_attrs!(item => attrs);
            attrs.truncate(base_attrs_count);
        }

        // else
        if let Some(el) = cfg_pub.tail {
            return Err(syn::Error::new_spanned(
                el,
                "`#[cfg_pub]` does not do actual cfg chaining yet",
            ));
        }

        Ok(out)
    }

    match catch_unwind(|| cfg_pub(attr, item)) {
        Ok(tts) => tts.unwrap_or_else(|e| e.to_compile_error()),
        Err(panic) => {
            if let Some(s) = panic.downcast_ref::<String>() {
                syn::Error::new(
                    Span::call_site(),
                    format_args!("proc-macro panicked: {}", s),
                )
                .to_compile_error()
            } else if let Some(s) = panic.downcast_ref::<&'static str>() {
                syn::Error::new(
                    Span::call_site(),
                    format_args!("proc-macro panicked: {}", s),
                )
                .to_compile_error()
            } else {
                syn::Error::new(Span::call_site(), "proc-macro panicked").to_compile_error()
            }
        }
    }
}
