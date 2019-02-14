macro_rules! ast_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident #manual_extra_traits $($rest:tt)*
    ) => {
        $(#[$attr])*
        #[cfg_attr(feature = "extra-traits", derive(Debug))]
        #[cfg_attr(feature = "clone-impls", derive(Clone))]
        pub struct $name $($rest)*
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident $($rest:tt)*
    ) => {
        $(#[$attr])*
        #[cfg_attr(feature = "extra-traits", derive(Debug, Eq, PartialEq, Hash))]
        #[cfg_attr(feature = "clone-impls", derive(Clone))]
        pub struct $name $($rest)*
    };
}

macro_rules! ast_enum {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident $(# $tags:ident)* { $($variants:tt)* }
    ) => (
        $(#[$enum_attr])*
        #[cfg_attr(feature = "extra-traits", derive(Debug, Eq, PartialEq, Hash))]
        #[cfg_attr(feature = "clone-impls", derive(Clone))]
        pub enum $name {
            $($variants)*
        }
    )
}

macro_rules! ast_enum_of_structs {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                pub $variant:ident $( ($member:ident $($rest:tt)*) )*,
            )*
        }

        $($remaining:tt)*
    ) => (
        ast_enum! {
            $(#[$enum_attr])*
            pub enum $name {
                $(
                    $(#[$variant_attr])*
                    $variant $( ($member) )*,
                )*
            }
        }

        $(
            maybe_ast_struct! {
                $(#[$variant_attr])*
                $(
                    pub struct $member $($rest)*
                )*
            }

            $(
                impl From<$member> for $name {
                    fn from(e: $member) -> $name {
                        $name::$variant(e)
                    }
                }
            )*
        )*

        generate_to_tokens! {
            $($remaining)*
            ()
            tokens
            $name { $($variant $( [$($rest)*] )*,)* }
        }
    )
}

macro_rules! generate_to_tokens {
    (do_not_generate_to_tokens $($foo:tt)*) => ();

    (($($arms:tt)*) $tokens:ident $name:ident { $variant:ident, $($next:tt)*}) => {
        generate_to_tokens!(
            ($($arms)* $name::$variant => {})
            $tokens $name { $($next)* }
        );
    };

    (($($arms:tt)*) $tokens:ident $name:ident { $variant:ident [$($rest:tt)*], $($next:tt)*}) => {
        generate_to_tokens!(
            ($($arms)* $name::$variant(ref _e) => to_tokens_call!(_e, $tokens, $($rest)*),)
            $tokens $name { $($next)* }
        );
    };

    (($($arms:tt)*) $tokens:ident $name:ident {}) => {
        impl ::quote::ToTokens for $name {
            fn to_tokens(&self, $tokens: &mut ::proc_macro2::TokenStream) {
                match *self {
                    $($arms)*
                }
            }
        }
    };
}

macro_rules! to_tokens_call {
    ($e:ident, $tokens:ident, $($rest:tt)*) => {
        ::quote::ToTokens::to_tokens($e, $tokens)
    };
}

macro_rules! maybe_ast_struct {
    (
        $(#[$attr:meta])*
        $(
            pub struct $name:ident
        )*
    ) => ();

    ($($rest:tt)*) => (ast_struct! { $($rest)* });
}
