// Set the recursion limit to 128 to avoid stack overflows when
#![recursion_limit = "128"]

use proc_macro2::TokenStream;
// Import dependencies
use syn::{parse_macro_input, DataStruct, DeriveInput, MetaNameValue};

// Extern crates are used to import external dependencies
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

// Procedural macros are declared by annotating a function with #[proc_macro_derive] or #[proc_macro_attribute].
#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&input.data);

    // Build the output, possibly using quasi-quotations
    proc_macro::TokenStream::from(quote! {
        impl #ident #generics #where_clause {
            #[allow(unused_variables)]
            pub fn vertex_attrib_pointers(gl: &::gl::Gl) {
                let stride = ::std::mem::size_of::<Self>();
                let offset = 0;

                #(#fields_vertex_attrib_pointer)*
            }
        }
    })
}

// Function which inspecting types with panic calls until we arrive at something reasonable
fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<TokenStream> {
    match data {
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|f| generate_struct_field_vertex_attrib_pointer_call(f))
            .collect(),
        _ => panic!("#[derive(VertexAttribPointers)] is only defined for structs"),
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> TokenStream {
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };

    let location_attr = field
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("location"))
        .next()
        .unwrap_or_else(|| panic!("Field {} is missing #[location = ?] attribute", field_name));

    let location_value: usize = match location_attr.meta {
        syn::Meta::NameValue(MetaNameValue { value: ref val, .. }) => expr_to_usize(val),
        _ => panic!(
            "Field {} location attribute value must be a string literal",
            field_name
        ),
    };

    let field_ty = &field.ty;

    TokenStream::from(quote! {
        let location = #location_value;
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + ::std::mem::size_of::<#field_ty>();
    })
}

// Convert a syn::Expr to usize
fn expr_to_usize(expr: &syn::Expr) -> usize {
    syn::LitInt::new(&expr_to_string(expr), proc_macro2::Span::call_site())
        .base10_parse()
        .unwrap_or_else(|_| panic!("Expected integer literal 2"))
}

// Convert a syn::Expr to a string
fn expr_to_string(expr: &syn::Expr) -> String {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(ref s),
            ..
        }) => s.value(),
        _ => panic!("Unexpected string literal"),
    }
}
