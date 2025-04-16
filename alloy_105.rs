#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use alloy::rpc::types::Transaction;
use alloy_primitives::B256;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
    let transaction = Transaction::default();
    let full = BlockTransactions::Full(
        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([transaction])),
    );
    let json = serde_json::to_string(&full).unwrap();
    {
        ::std::io::_print(format_args!("{0}\n", json));
    };
    let deserialized: BlockTransactions = serde_json::from_str(&json).unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", deserialized));
    };
}
#[serde(tag = "type", content = "txs")]
pub enum BlockTransactions {
    /// Full transactions
    Full(Vec<Transaction>),
    /// Only hashes
    Hashes(Vec<B256>),
}
#[automatically_derived]
impl ::core::clone::Clone for BlockTransactions {
    #[inline]
    fn clone(&self) -> BlockTransactions {
        match self {
            BlockTransactions::Full(__self_0) => {
                BlockTransactions::Full(::core::clone::Clone::clone(__self_0))
            }
            BlockTransactions::Hashes(__self_0) => {
                BlockTransactions::Hashes(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for BlockTransactions {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            BlockTransactions::Full(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Full", &__self_0)
            }
            BlockTransactions::Hashes(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Hashes", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for BlockTransactions {}
#[automatically_derived]
impl ::core::cmp::PartialEq for BlockTransactions {
    #[inline]
    fn eq(&self, other: &BlockTransactions) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (
                    BlockTransactions::Full(__self_0),
                    BlockTransactions::Full(__arg1_0),
                ) => __self_0 == __arg1_0,
                (
                    BlockTransactions::Hashes(__self_0),
                    BlockTransactions::Hashes(__arg1_0),
                ) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for BlockTransactions {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Vec<Transaction>>;
        let _: ::core::cmp::AssertParamIsEq<Vec<B256>>;
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for BlockTransactions {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                BlockTransactions::Full(ref __field0) => {
                    let mut __struct = _serde::Serializer::serialize_struct(
                        __serializer,
                        "BlockTransactions",
                        2,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "type",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "BlockTransactions",
                            variant_index: 0u32,
                            variant_name: "Full",
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "txs",
                        __field0,
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                BlockTransactions::Hashes(ref __field0) => {
                    let mut __struct = _serde::Serializer::serialize_struct(
                        __serializer,
                        "BlockTransactions",
                        2,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "type",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "BlockTransactions",
                            variant_index: 1u32,
                            variant_name: "Hashes",
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "txs",
                        __field0,
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BlockTransactions {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Full" => _serde::__private::Ok(__Field::__field0),
                        "Hashes" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Full" => _serde::__private::Ok(__Field::__field0),
                        b"Hashes" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["Full", "Hashes"];
            #[doc(hidden)]
            struct __Seed<'de> {
                field: __Field,
                marker: _serde::__private::PhantomData<BlockTransactions>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                type Value = BlockTransactions;
                fn deserialize<__D>(
                    self,
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self::Value, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    match self.field {
                        __Field::__field0 => {
                            _serde::__private::Result::map(
                                <Vec<
                                    Transaction,
                                > as _serde::Deserialize>::deserialize(__deserializer),
                                BlockTransactions::Full,
                            )
                        }
                        __Field::__field1 => {
                            _serde::__private::Result::map(
                                <Vec<
                                    B256,
                                > as _serde::Deserialize>::deserialize(__deserializer),
                                BlockTransactions::Hashes,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BlockTransactions>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BlockTransactions;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "adjacently tagged enum BlockTransactions",
                    )
                }
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    match {
                        let mut __rk: _serde::__private::Option<
                            _serde::__private::de::TagOrContentField,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                            &mut __map,
                            _serde::__private::de::TagContentOtherFieldVisitor {
                                tag: "type",
                                content: "txs",
                            },
                        )? {
                            match __k {
                                _serde::__private::de::TagContentOtherField::Other => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                    continue;
                                }
                                _serde::__private::de::TagContentOtherField::Tag => {
                                    __rk = _serde::__private::Some(
                                        _serde::__private::de::TagOrContentField::Tag,
                                    );
                                    break;
                                }
                                _serde::__private::de::TagContentOtherField::Content => {
                                    __rk = _serde::__private::Some(
                                        _serde::__private::de::TagOrContentField::Content,
                                    );
                                    break;
                                }
                            }
                        }
                        __rk
                    } {
                        _serde::__private::Some(
                            _serde::__private::de::TagOrContentField::Tag,
                        ) => {
                            let __field = _serde::de::MapAccess::next_value_seed(
                                &mut __map,
                                _serde::__private::de::AdjacentlyTaggedEnumVariantSeed::<
                                    __Field,
                                > {
                                    enum_name: "BlockTransactions",
                                    variants: VARIANTS,
                                    fields_enum: _serde::__private::PhantomData,
                                },
                            )?;
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                    &mut __map,
                                    _serde::__private::de::TagContentOtherFieldVisitor {
                                        tag: "type",
                                        content: "txs",
                                    },
                                )? {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => {
                                    _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    )
                                }
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => {
                                    let __ret = _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        __Seed {
                                            field: __field,
                                            marker: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )?;
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "txs",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            )
                                        }
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("txs"),
                                            )
                                        }
                                        _serde::__private::None => _serde::__private::Ok(__ret),
                                    }
                                }
                                _serde::__private::None => {
                                    match __field {
                                        __Field::__field0 => {
                                            _serde::__private::de::missing_field("txs")
                                                .map(BlockTransactions::Full)
                                        }
                                        __Field::__field1 => {
                                            _serde::__private::de::missing_field("txs")
                                                .map(BlockTransactions::Hashes)
                                        }
                                    }
                                }
                            }
                        }
                        _serde::__private::Some(
                            _serde::__private::de::TagOrContentField::Content,
                        ) => {
                            let __content = _serde::de::MapAccess::next_value::<
                                _serde::__private::de::Content,
                            >(&mut __map)?;
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                    &mut __map,
                                    _serde::__private::de::TagContentOtherFieldVisitor {
                                        tag: "type",
                                        content: "txs",
                                    },
                                )? {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => {
                                    let __deserializer = _serde::__private::de::ContentDeserializer::<
                                        __A::Error,
                                    >::new(__content);
                                    let __ret = match _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        _serde::__private::de::AdjacentlyTaggedEnumVariantSeed::<
                                            __Field,
                                        > {
                                            enum_name: "BlockTransactions",
                                            variants: VARIANTS,
                                            fields_enum: _serde::__private::PhantomData,
                                        },
                                    )? {
                                        __Field::__field0 => {
                                            _serde::__private::Result::map(
                                                <Vec<
                                                    Transaction,
                                                > as _serde::Deserialize>::deserialize(__deserializer),
                                                BlockTransactions::Full,
                                            )
                                        }
                                        __Field::__field1 => {
                                            _serde::__private::Result::map(
                                                <Vec<
                                                    B256,
                                                > as _serde::Deserialize>::deserialize(__deserializer),
                                                BlockTransactions::Hashes,
                                            )
                                        }
                                    }?;
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "txs",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            )
                                        }
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("txs"),
                                            )
                                        }
                                        _serde::__private::None => _serde::__private::Ok(__ret),
                                    }
                                }
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => {
                                    _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("txs"),
                                    )
                                }
                                _serde::__private::None => {
                                    _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("type"),
                                    )
                                }
                            }
                        }
                        _serde::__private::None => {
                            _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            )
                        }
                    }
                }
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    match _serde::de::SeqAccess::next_element(&mut __seq)? {
                        _serde::__private::Some(__field) => {
                            match _serde::de::SeqAccess::next_element_seed(
                                &mut __seq,
                                __Seed {
                                    field: __field,
                                    marker: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )? {
                                _serde::__private::Some(__ret) => {
                                    _serde::__private::Ok(__ret)
                                }
                                _serde::__private::None => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_length(1, &self),
                                    )
                                }
                            }
                        }
                        _serde::__private::None => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_length(0, &self),
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["type", "txs"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "BlockTransactions",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BlockTransactions>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
