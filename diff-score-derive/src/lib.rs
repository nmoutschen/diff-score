use darling::{
    FromDeriveInput, FromField, FromVariant,
    ast::{self, Fields},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, Ident, Path, parse_macro_input};

const DEFAULT_WEIGHT: f64 = 1.0;

#[derive(Debug, FromField)]
#[darling(attributes(diff_score))]
struct FieldOpts {
    ident: Option<Ident>,
    /// Provide a weight for the field
    #[darling(default)]
    weight: Option<f64>,
    /// Provide a custom function to compute the score
    #[darling(default)]
    with: Option<Path>,
    #[darling(default)]
    skip: bool,
}

impl FieldOpts {
    fn weight(&self) -> f64 {
        if self.skip {
            0.0
        } else {
            self.weight.unwrap_or(DEFAULT_WEIGHT)
        }
    }

    fn as_expr(&self, a: &TokenStream, b: &TokenStream) -> TokenStream {
        let weight = self.weight();
        if self.skip {
            return quote! {};
        }

        match self.with.as_ref() {
            // Some(Path) => quote! {
            //     score += if #a == #b { 0.0 } else { #weight }
            // },
            Some(f) => {
                quote! {
                    score += #weight * #f(&#a, &#b);
                }
            }
            None => quote! {
                score += #weight * #a.diff_score(&#b);
            },
        }
    }
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
}

impl ToTokens for DiffScoreReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DiffScoreReceiver { ident, data } = self;

        *tokens = match data {
            ast::Data::Enum(variants) => {
                let score_exprs: Vec<_> = variants
                    .iter()
                    .map(|variant| {
                        let variant_ident = &variant.ident;
                        match variant.fields.style {
                            ast::Style::Tuple => {
                                let mut self_fields = Vec::new();
                                let mut other_fields = Vec::new();
                                let mut exprs = Vec::new();

                                let mut total_weight = 0.0;

                                for (num, field) in variant.fields.iter().enumerate() {
                                    let self_ident = format_ident!("sf{num}");
                                    let other_ident = format_ident!("of{num}");
                                    let weight = field.weight();
                                    total_weight += weight;
                                    exprs.push(field.as_expr(&quote!{#self_ident}, &quote!{#other_ident}));
                                    self_fields.push(self_ident);
                                    other_fields.push(other_ident);
                                }

                                quote! {
                                    (#ident::#variant_ident(#(#self_fields,)*), #ident::#variant_ident(#(#other_fields,)*)) => {
                                        let mut score = 0.0;
                                        #(#exprs)*
                                        score / #total_weight
                                    }
                                }
                            }
                            ast::Style::Struct => {
                                let mut self_fields = Vec::new();
                                let mut other_fields = Vec::new();
                                let mut exprs = Vec::new();
                                let mut total_weight = 0.0;

                                for  field in variant.fields.iter() {
                                    let self_ident = field.ident.as_ref().unwrap();
                                    let other_ident = format_ident!("other_{self_ident}");

                                    self_fields.push(self_ident);
                                    other_fields.push(quote! { #self_ident: #other_ident });

                                    let weight = field.weight();
                                    total_weight += weight;
                                    exprs.push(field.as_expr(&quote!{#self_ident}, &quote!{#other_ident}));
                                }

                                quote! {
                                    (#ident::#variant_ident { #(#self_fields,)* }, #ident::#variant_ident { #(#other_fields,)* }) => {
                                        let mut score = 0.0;
                                        #(#exprs)*
                                        score / #total_weight
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
                                (_, _) => #DEFAULT_WEIGHT,
                            }
                        }
                    }
                }
            }
            ast::Data::Struct(fields) => {
                let mut total_weight = 0.0;
                let score_exprs: Vec<_> = fields
                    .iter()
                    .map(|field| {
                        let field_ident = field.ident.as_ref().unwrap();
                        let self_ident = quote! { self.#field_ident };
                        let other_ident = quote! { other.#field_ident };
                        total_weight += field.weight();

                        field.as_expr(&self_ident, &other_ident)
                    })
                    .collect();
                quote! {
                    impl ::diff_score::DiffScore for #ident {
                        fn diff_score(&self, other: &Self) -> f64 {
                            let mut score = 0.0;
                            #(#score_exprs)*
                            score / #total_weight
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
