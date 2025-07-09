use darling::{
    FromDeriveInput, FromField, FromVariant,
    ast::{self, Fields},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, Ident, parse_macro_input};

const DEFAULT_WEIGHT: f64 = 1.0;

#[derive(Debug, FromField)]
#[darling(attributes(diff_score))]
struct FieldOpts {
    ident: Option<Ident>,
    #[darling(default)]
    weight: Option<f64>,
    #[darling(default)]
    use_eq: bool,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(diff_score))]
struct VariantOpts {
    ident: Ident,
    fields: Fields<FieldOpts>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(diff_score))]
struct DiffScoreReceiver {
    ident: Ident,
    data: ast::Data<VariantOpts, FieldOpts>,

    #[darling(default)]
    default_penalty: Option<f64>,
}

impl ToTokens for DiffScoreReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DiffScoreReceiver {
            ident,
            data,
            default_penalty,
        } = self;

        *tokens = match data {
            ast::Data::Enum(variants) => {
                let default_penalty = default_penalty.unwrap_or(DEFAULT_WEIGHT);
                let score_exprs: Vec<_> = variants
                    .iter()
                    .map(|variant| {
                        let variant_ident = &variant.ident;
                        match variant.fields.style {
                            ast::Style::Tuple => {
                                let mut self_fields = Vec::new();
                                let mut other_fields = Vec::new();
                                let mut exprs = Vec::new();

                                for (num, field) in variant.fields.iter().enumerate() {
                                    let self_ident = format_ident!("sf{num}");
                                    let other_ident = format_ident!("of{num}");
                                    let weight = field.weight.unwrap_or(DEFAULT_WEIGHT);
                                    exprs.push(if field.use_eq {
                                        quote! {
                                            score += #weight * if #self_ident != #other_ident { 1.0 } else { 0.0 }
                                        }
                                    } else {
                                        quote! {
                                            score += #weight * #self_ident.diff_score(&#other_ident)
                                        }
                                    });
                                    self_fields.push(self_ident);
                                    other_fields.push(other_ident);
                                }

                                quote! {
                                    (#ident::#variant_ident(#(#self_fields,)*), #ident::#variant_ident(#(#other_fields,)*)) => {
                                        let mut score = 0.0;
                                        #(#exprs;)*
                                        score
                                    }
                                }
                            }
                            ast::Style::Struct => {
                                let mut self_fields = Vec::new();
                                let mut other_fields = Vec::new();
                                let mut exprs = Vec::new();

                                for  field in variant.fields.iter() {
                                    let field_ident = field.ident.as_ref().unwrap();
                                    let other_ident = format_ident!("other_{field_ident}");

                                    self_fields.push(field_ident);
                                    other_fields.push(quote! { #field_ident: #other_ident });

                                    let weight = field.weight.unwrap_or(DEFAULT_WEIGHT);
                                    exprs.push(if field.use_eq {
                                        quote! {
                                            score += #weight * if #field_ident != #other_ident { 1.0 } else { 0.0 }
                                        }
                                    } else {
                                        quote! {
                                            score += #weight * #field_ident.diff_score(&#other_ident)
                                        }
                                    });
                                }

                                quote! {
                                    (#ident::#variant_ident { #(#self_fields,)* }, #ident::#variant_ident { #(#other_fields,)* }) => {
                                        let mut score = 0.0;
                                        #(#exprs;)*
                                        score
                                    }
                                }
                            },
                            ast::Style::Unit => quote! {
                                (#ident::#variant_ident, #ident::#variant_ident) => 0.0
                            },
                        }
                    })
                    .collect();

                quote! {
                    impl ::diff_score::DiffScore for #ident {
                        fn diff_score(&self, other: &Self) -> f64 {
                            match (self, other) {
                                #(#score_exprs,)*
                                (_, _) => #default_penalty,
                            }
                        }
                    }
                }
            }
            ast::Data::Struct(fields) => {
                let score_exprs: Vec<_> = fields
                    .iter()
                    .map(|field| {
                        let field_ident = field.ident.as_ref().unwrap();
                        let weight = field.weight.unwrap_or(DEFAULT_WEIGHT);

                        if field.use_eq {
                            quote! {
                                score += #weight * if self.#field_ident != other.#field_ident { 1.0 } else { 0.0 }
                            }
                        } else {
                            quote! {
                                score += #weight * self.#field_ident.diff_score(&other.#field_ident)
                            }
                        }
                    })
                    .collect();
                quote! {
                    impl ::diff_score::DiffScore for #ident {
                        fn diff_score(&self, other: &Self) -> f64 {
                            let mut score = 0.0;
                            #(#score_exprs;)*
                            score
                        }
                    }
                }
            }
        };
    }
}

#[proc_macro_derive(DiffScore, attributes(diff_score))]
pub fn diff_score(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let receiver = DiffScoreReceiver::from_derive_input(&input).unwrap();

    quote!(#receiver).into()
}
