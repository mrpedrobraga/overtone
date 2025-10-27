use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Error, FnArg, ImplItem, Index, ItemImpl, LitInt, LitStr, PatType, Type,
    TypeReference,
};

#[proc_macro_attribute]
pub fn node_impl(attribute: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(input as ItemImpl);

    let Some((_, trait_, _)) = impl_block.trait_.as_ref() else {
        return Error::new_spanned(
            &impl_block,
            "this is supposed to be an implementation of `Node`",
        )
        .to_compile_error()
        .into();
    };

    enum FieldRef {
        Indexed(Index),
        Named(Ident),
    }
    let mut self_fields = Vec::new();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("fields") {
            meta.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    if let Ok(v) = meta.value() {
                        if let Ok(lit_int) = v.parse::<LitInt>() {
                            let idx = syn::Index::from(lit_int.base10_parse::<usize>()?);
                            self_fields.push((ident.clone(), FieldRef::Indexed(idx)));
                        } else if let Ok(lit_str) = v.parse::<LitStr>() {
                            // string literal → map to local ident
                            self_fields.push((
                                ident.clone(),
                                FieldRef::Named(format_ident!("{}", lit_str.value())),
                            ));
                        } else {
                            return Err(meta.error("expected integer or string literal"));
                        }
                    } else {
                        self_fields.push((ident.clone(), FieldRef::Named(ident.clone())));
                    }
                }
                Ok(())
            })
        } else {
            Err(meta.error("unsupported property"))
        }
    });

    parse_macro_input!(attribute with args_parser);

    let func = impl_block.items.iter().find_map(|item| match item {
        ImplItem::Fn(f) if f.sig.ident == "process" => Some(f),
        _ => None,
    });
    let Some(func) = func else {
        return Error::new_spanned(&impl_block, "expected `fn process(...inputs, ...outputs)`")
            .to_compile_error()
            .into();
    };

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for argument in &func.sig.inputs {
        match argument {
            FnArg::Typed(PatType { pat, ty, .. }) => match &**ty {
                Type::Reference(TypeReference {
                    mutability, elem, ..
                }) => {
                    if mutability.is_some() {
                        outputs.push((pat.clone(), elem.clone()));
                    } else {
                        inputs.push((pat.clone(), elem.clone()));
                    }
                }
                &_ => {
                    return Error::new_spanned(
                        &impl_block,
                        "arguments must be `#[field]` (a field of `self`), `&T` (an input) or `&mut T` (an output)",
                    )
                    .to_compile_error()
                    .into();
                }
            },
            FnArg::Receiver(_) => {
                return Error::new_spanned(
                    &impl_block,
                    "can not take self as argument — use `#[field]`",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let input_binds = inputs.iter().map(|(pat, ty)| {
        quote! { let #pat = ::cables_core::as_input::<'pip, #ty>(parameters.next().unwrap()); }
    });
    let output_binds = outputs.iter().map(|(pat, ty)| {
        quote! { let #pat = ::cables_core::as_output::<'pip, #ty>(parameters.next().unwrap()); }
    });
    let field_binds = self_fields.iter().map(|(local, field)| match field {
        FieldRef::Indexed(idx) => quote! { let #local = Clone::clone(&self.#idx); },
        FieldRef::Named(field) => quote! { let #local = Clone::clone(&self.#field); },
    });

    let body = &func.block;

    let fn_bind_parameters = quote! {
        fn bind_parameters<'pip>(
            &self,
            parameters: &mut dyn Iterator<Item = *mut u8>
        ) -> Box<dyn FnMut() + 'pip> {
            #(#field_binds)*
            #(#input_binds)*
            #(#output_binds)*
            Box::new(
                move || #body
            )
        }
    };

    let input_socket_match_arms = inputs.iter().enumerate().map(|(i, (_, ty))| {
        quote! { #i => Some(::cables_core::graph::SocketData::new::<#ty>()), }
    });
    let output_socket_match_arms = outputs.iter().enumerate().map(|(i, (_, ty))| {
        quote! { #i => Some(::cables_core::graph::SocketData::new::<#ty>()), }
    });

    let fn_input_socket = quote! {
        fn input_socket(&self, idx: usize) -> Option<::cables_core::graph::SocketData> {
            match idx {
                #(#input_socket_match_arms)*
                _ => None,
            }
        }
    };

    let fn_output_socket = quote! {
        fn output_socket(&self, idx: usize) -> Option<::cables_core::graph::SocketData> {
            match idx {
                #(#output_socket_match_arms)*
                _ => None,
            }
        }
    };

    let (impl_generics, _, where_clause) = impl_block.generics.split_for_impl();
    let self_ty = &impl_block.self_ty;

    let final_impl = quote! {
        impl #impl_generics #trait_ for #self_ty #where_clause {
            #fn_bind_parameters
            #fn_input_socket
            #fn_output_socket
        }
    };

    final_impl.into()
}
