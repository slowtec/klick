use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result as ParseResult},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Lit, Token,
};

#[proc_macro]
pub fn value_spec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ValueSpecInput);
    let value_type = &input.value_type;
    let ident = &input.ident;

    let variant_quotes = input.variants.iter().map(|variant| {
        let variant_name = &variant.name;

        let is_optional = variant
            .attrs
            .iter()
            .any(|attr| matches!(attr, Attr::Optional));
        let optional = quote! {
            Self::#variant_name => #is_optional,
        };

        let unit: &Ident = variant
            .attrs
            .iter()
            .find_map(|attr| {
                if let Attr::Unit(val) = attr {
                    Some(val)
                } else {
                    None
                }
            })
            .expect("defined unit");

        let snake_case_unit_ident = format_ident!("{}", to_snake_case(unit.to_string().as_str()));

        let unit_value = |lit: Option<&_>| {
            let Some(val) = lit else {
                return quote! { None };
            };
            quote! {
                Some(Value::#snake_case_unit_ident(#val))
            }
        };

        let default_lit: Option<&_> = variant.attrs.iter().find_map(|attr| {
            if let Attr::Default(val) = attr {
                Some(val)
            } else {
                None
            }
        });

        let min_lit = variant.attrs.iter().find_map(|attr| {
            if let Attr::Min(val) = attr {
                Some(val)
            } else {
                None
            }
        });

        let max_lit = variant.attrs.iter().find_map(|attr| {
            if let Attr::Max(val) = attr {
                Some(val)
            } else {
                None
            }
        });

        let variant_quote = |val| {
            quote! {
                Self::#variant_name => {
                    #val
                }
            }
        };

        let default_value = unit_value(default_lit);

        let min = match min_lit {
            Some(val) => quote! {
               Self::#variant_name => Some(#val)
            },
            None => quote! {
                Self::#variant_name => None
            },
        };

        let max = match max_lit {
            Some(val) => quote! {
               Self::#variant_name => Some(#val)
            },
            None => quote! {
                Self::#variant_name => None
            },
        };

        let default = variant_quote(default_value);

        let snake_case_ident = to_snake_case(&variant_name.to_string());

        let value_field = (
            variant_name,
            format_ident!("{}", snake_case_ident),
            unit,
            is_optional,
        );

        (variant_name, optional, default, min, max, value_field)
    });

    let (variants, optionals, defaults, mins, maxs, value_fields): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = itertools::multiunzip(variant_quotes);

    let value_types = value_fields.iter().map(|(variant, _, unit, _)| {
        quote! {
            Self::#variant => #unit::VALUE_TYPE
        }
    });

    let required_extractors = value_fields.iter().map(|(variant, _, unit, _)| {
        let snake_case_unit = to_snake_case(unit.to_string().as_str());
        let getter_name = format_ident!("as_{}_unchecked", snake_case_unit);
        quote! {
            ($enum:ident ::#variant, $values:expr) => {
                $values
                    .get(&$enum::#variant.into())
                    .cloned()
                    .or_else(|| $enum::#variant.default_value())
                    .ok_or(anyhow::anyhow!("required value ({:?}) is missing", $enum::#variant))
                    .map(|value|{
                        debug_assert_eq!($enum::#variant.value_type(), value.value_type());
                        value.#getter_name()
                    })
            };
        }
    });

    let optional_extractors = value_fields
        .iter()
        .filter(|(_, _, _, is_optional)| *is_optional)
        .map(|(variant, _, unit, _)| {
            let getter_name =
                format_ident!("as_{}_unchecked", to_snake_case(unit.to_string().as_str()));
            quote! {
                ($enum:ident ::#variant, $values:expr) => {
                    $values
                        .get(&$enum::#variant.into())
                        .cloned()
                        .or_else(|| $enum::#variant.default_value())
                        .map(|value|{
                            debug_assert_eq!($enum::#variant.value_type(), value.value_type());
                            value.#getter_name()
                        })
                };
            }
        })
        .collect::<Vec<_>>();

    let snake_case_ident = to_snake_case(ident.to_string().as_str());
    let required_macro_name = format_ident!("required_{}", snake_case_ident);
    let optional_macro_name = format_ident!("optional_{}", snake_case_ident);

    let opt_macro = if optional_extractors.is_empty() {
        quote! {}
    } else {
        quote! {
            #[macro_export]
            macro_rules! #optional_macro_name {
                #(#optional_extractors)*
            }
        }
    };

    let expanded = quote! {

        #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, strum::AsRefStr, strum_macros::EnumIter)]
        pub enum #ident {
            #(#variants,)*
        }

        impl #ident {
            #[must_use]
            pub const fn is_optional(&self) -> bool {
                match self {
                    #(#optionals)*
                }
            }

            #[must_use]
            pub fn default_value(&self) -> Option<#value_type> {
                match self {
                    #(#defaults),*
                }
            }

            #[must_use]
            pub const fn min(&self) -> Option<f64> {
                match self {
                    #(#mins),*
                }
            }

            #[must_use]
            pub const fn max(&self) -> Option<f64> {
                match self {
                    #(#maxs),*
                }
            }

            #[must_use]
            pub const fn value_type(&self) -> ValueType {
                match self {
                    #(#value_types),*
                }
            }
        }

        #[macro_export]
        macro_rules! #required_macro_name {
            #(#required_extractors)*
        }

        #opt_macro
    };

    TokenStream::from(expanded)
}

struct ValueSpecInput {
    value_type: Ident,
    ident: Ident,
    variants: Punctuated<Variant, Token![,]>,
}

struct Variant {
    name: Ident,
    attrs: Vec<Attr>,
}

enum Attr {
    Optional,
    Unit(Ident),
    Default(Expr),
    Min(Lit),
    Max(Lit),
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "optional" => Ok(Attr::Optional),
            "unit" => {
                input.parse::<Token![=]>()?;
                let unit = input.parse()?;
                Ok(Attr::Unit(unit))
            }
            "min" | "max" => {
                input.parse::<Token![=]>()?;
                let value = input.parse()?;
                match ident.to_string().as_str() {
                    "min" => Ok(Attr::Min(value)),
                    "max" => Ok(Attr::Max(value)),
                    _ => unreachable!(),
                }
            }
            "default" => {
                input.parse::<Token![=]>()?;
                let value = input.parse()?;
                match ident.to_string().as_str() {
                    "default" => Ok(Attr::Default(value)),
                    _ => unreachable!(),
                }
            }
            _ => Err(input.error("Unknown attribute")),
        }
    }
}

impl Parse for ValueSpecInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let value_type: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ident: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let variants = content.parse_terminated(Variant::parse, Token![,])?;

        Ok(Self {
            value_type,
            ident,
            variants,
        })
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let attrs = content.parse_terminated(Attr::parse, Token![;])?;
        let attrs = attrs.into_iter().collect();
        Ok(Self { name, attrs })
    }
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let chars = s.chars().peekable();
    for c in chars {
        if !c.is_uppercase() {
            snake_case.push(c);
            continue;
        }
        if !snake_case.is_empty() {
            snake_case.push('_');
        }
        snake_case.extend(c.to_lowercase());
    }
    snake_case
}
