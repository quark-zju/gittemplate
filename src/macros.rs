// Generated by macros_gen.py

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_object_state {
    // ObjectProtocol impl block.
    // - pattern: impl ObjectProtocol for $_name:tt { $($impl_object_protocol:tt)* } ...
    // - state: impl_object_protocol += $($impl_object_protocol)*
    (input [impl ObjectProtocol for $_name:tt { $($impl_object_protocol:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)* $($impl_object_protocol)*]) (name [$($_name_rest)*])]);
    };

    // Main impl block.
    // - pattern: $(#[$impl_meta:meta])* impl $name:ident { ... }
    // - state: name += $name
    // - state: impl_meta += $($impl_meta)*
    (input [$(#[$impl_meta:meta])* impl $name:ident { $($_input_rest:tt)* }] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)* $($impl_meta)*]) (impl_object_protocol [$($_impl_object_protocol_rest)*]) (name [$($_name_rest)* $name])]);
    };

    // Special case: is_true (ObjectProtocol).
    // - pattern: fn is_true $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_object_protocol += fn is_true $fn_args -> $fn_res { $($fn_body)* }
    (input [fn is_true $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)* fn is_true $fn_args -> $fn_res { $($fn_body)* }]) (name [$($_name_rest)*])]);
    };

    // Special case: deref_object (ObjectProtocol).
    // - pattern: fn deref_object $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_object_protocol += fn deref_object $fn_args -> $fn_res { $($fn_body)* }
    // - state: impl_deref_object =
    (input [fn deref_object $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [ ]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)* fn deref_object $fn_args -> $fn_res { $($fn_body)* }]) (name [$($_name_rest)*])]);
    };

    // Special case: is_eq (ObjectProtocol).
    // - pattern: fn is_eq $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_object_protocol += fn is_eq $fn_args -> $fn_res { $($fn_body)* }
    (input [fn is_eq $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)* fn is_eq $fn_args -> $fn_res { $($fn_body)* }]) (name [$($_name_rest)*])]);
    };

    // Special case: to_serde_value (ObjectProtocol).
    // - pattern: fn to_serde_value $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_object_protocol += fn to_serde_value $fn_args -> $fn_res { $($fn_body)* }
    (input [fn to_serde_value $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)* fn to_serde_value $fn_args -> $fn_res { $($fn_body)* }]) (name [$($_name_rest)*])]);
    };

    // Special case: to_ast_fmt_string (ObjectProtocol).
    // - pattern: fn to_ast_fmt_string $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_object_protocol += fn to_ast_fmt_string $fn_args -> $fn_res { $($fn_body)* }
    (input [fn to_ast_fmt_string $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)*]) (impl_body [$($_impl_body_rest)*]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)* fn to_ast_fmt_string $fn_args -> $fn_res { $($fn_body)* }]) (name [$($_name_rest)*])]);
    };

    // Special case: r#if function (raw ident).
    // - pattern: $(#[$fn_meta:meta])* $fn_vis:vis fn r#if $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_body += $(#[$fn_meta])* $fn_vis fn r#if $fn_args -> $fn_res { $($fn_body)* }
    // - state: attr_names += (r#if "if")
    (input [$(#[$fn_meta:meta])* $fn_vis:vis fn r#if $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)* (r#if "if")]) (impl_body [$($_impl_body_rest)* $(#[$fn_meta])* $fn_vis fn r#if $fn_args -> $fn_res { $($fn_body)* }]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)*]) (name [$($_name_rest)*])]);
    };

    // Special case: r#try function (raw ident).
    // - pattern: $(#[$fn_meta:meta])* $fn_vis:vis fn r#try $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_body += $(#[$fn_meta])* $fn_vis fn r#try $fn_args -> $fn_res { $($fn_body)* }
    // - state: attr_names += (r#try "try")
    (input [$(#[$fn_meta:meta])* $fn_vis:vis fn r#try $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)* (r#try "try")]) (impl_body [$($_impl_body_rest)* $(#[$fn_meta])* $fn_vis fn r#try $fn_args -> $fn_res { $($fn_body)* }]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)*]) (name [$($_name_rest)*])]);
    };

    // Method or property.
    // - pattern: $(#[$fn_meta:meta])* $fn_vis:vis fn $fn_name:ident $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } ...
    // - state: impl_body += $(#[$fn_meta])* $fn_vis fn $fn_name $fn_args -> $fn_res { $($fn_body)* }
    // - state: attr_names += ($fn_name stringify!($fn_name))
    (input [$(#[$fn_meta:meta])* $fn_vis:vis fn $fn_name:ident $fn_args:tt -> $fn_res:ty { $($fn_body:tt)* } $($_input_rest:tt)*] state [(attr_names [$($_attr_names_rest:tt)*]) (impl_body [$($_impl_body_rest:tt)*]) (impl_deref_object [$($_impl_deref_object_rest:tt)*]) (impl_meta [$($_impl_meta_rest:tt)*]) (impl_object_protocol [$($_impl_object_protocol_rest:tt)*]) (name [$($_name_rest:tt)*])]) => {
        $crate::_impl_object_state!(input [$($_input_rest)*] state [(attr_names [$($_attr_names_rest)* ($fn_name stringify!($fn_name))]) (impl_body [$($_impl_body_rest)* $(#[$fn_meta])* $fn_vis fn $fn_name $fn_args -> $fn_res { $($fn_body)* }]) (impl_deref_object [$($_impl_deref_object_rest)*]) (impl_meta [$($_impl_meta_rest)*]) (impl_object_protocol [$($_impl_object_protocol_rest)*]) (name [$($_name_rest)*])]);
    };

    // No more input. Conclude the final state.
    (input [] state [(attr_names [$(($attr:ident $attr_str:expr))*]) (impl_body [$($impl_body:tt)*]) (impl_deref_object [$($impl_deref_object:tt)*]) (impl_meta [$($impl_meta:meta)*]) (impl_object_protocol [$($impl_object_protocol:tt)*]) (name [$impl_name:ident])]) => {
        $(#[$impl_meta])*
        impl $impl_name {
            $($impl_body)*
        }
        impl crate::objects::protocol::ObjectProtocol for $impl_name {
            fn get_attr(&self, name: &str) -> crate::Result<crate::objects::protocol::Attribute> {
                #[allow(unused_imports)]
                use crate::objects::protocol::{Attribute, ToAttribute};
                match name {
                    $(
                    $attr_str => Ok(ToAttribute::to_attribute(self, &Self::$attr, $attr_str)?),
                    )*
                    #[allow(unreachable_patterns)]
                    _ => Ok(Attribute::Missing)
                }
            }
            fn static_list_attrs() -> &'static [&'static str] {
                &[$( $attr_str, )*]
            }
            fn list_attrs(&self) -> &'static [&'static str] {
                Self::static_list_attrs()
            }
            fn as_any(&self) -> Option<&dyn std::any::Any> {
                Some(self as &dyn std::any::Any)
            }
            fn type_name(&self) -> std::borrow::Cow<'static, str> {
                Self::static_type_name()
            }
            fn static_type_name() -> std::borrow::Cow<'static, str> {
                let name = stringify!($impl_name);
                let name = name.strip_suffix("Object").unwrap_or(name);
                name.into()
            }
            $($impl_object_protocol)*
            $($impl_deref_object)*
        }
        impl crate::objects::protocol::convert::IntoObject for $impl_name {
            fn into_object(self) -> crate::objects::protocol::Object {
                crate::objects::protocol::ObjectProtocol::to_object(self)
            }
        }
    };
}

/// Derive `ObjectProtocol` methods.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_object {
    {$($input:tt)*} => {
        $crate::_impl_object_state!(input [$($input)*] state [(attr_names [])(impl_body [])(impl_deref_object [            fn deref_object(&self, name: &str) -> crate::Result<Option<crate::objects::protocol::Object>> {
    crate::objects::auto_deref::default_deref_object(self, name)
}
])(impl_meta [])(impl_object_protocol [])(name [])]);
    };
}
