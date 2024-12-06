#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use alloy::primitives::{Bytes, ChainId, TxHash};
use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};
fn main() {
    {
        ::std::io::_print(format_args!("Alloy 102 example!\n"));
    };
}
#[rlp(trailing)]
pub struct RecoveryMessage {
    pub nonce: u64,
    pub incomplete_signatures: Vec<IncompleteSignature>,
    pub chain_id: Option<ChainId>,
}
#[automatically_derived]
impl ::core::fmt::Debug for RecoveryMessage {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "RecoveryMessage",
            "nonce",
            &self.nonce,
            "incomplete_signatures",
            &self.incomplete_signatures,
            "chain_id",
            &&self.chain_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for RecoveryMessage {
    #[inline]
    fn clone(&self) -> RecoveryMessage {
        RecoveryMessage {
            nonce: ::core::clone::Clone::clone(&self.nonce),
            incomplete_signatures: ::core::clone::Clone::clone(
                &self.incomplete_signatures,
            ),
            chain_id: ::core::clone::Clone::clone(&self.chain_id),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RecoveryMessage {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RecoveryMessage {
    #[inline]
    fn eq(&self, other: &RecoveryMessage) -> bool {
        self.nonce == other.nonce
            && self.incomplete_signatures == other.incomplete_signatures
            && self.chain_id == other.chain_id
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RecoveryMessage {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u64>;
        let _: ::core::cmp::AssertParamIsEq<Vec<IncompleteSignature>>;
        let _: ::core::cmp::AssertParamIsEq<Option<ChainId>>;
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for RecoveryMessage {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "RecoveryMessage",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "nonce",
                &self.nonce,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "incomplete_signatures",
                &self.incomplete_signatures,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "chain_id",
                &self.chain_id,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for RecoveryMessage {
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
                __field2,
                __ignore,
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
                        "field identifier",
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
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
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
                        "nonce" => _serde::__private::Ok(__Field::__field0),
                        "incomplete_signatures" => {
                            _serde::__private::Ok(__Field::__field1)
                        }
                        "chain_id" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
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
                        b"nonce" => _serde::__private::Ok(__Field::__field0),
                        b"incomplete_signatures" => {
                            _serde::__private::Ok(__Field::__field1)
                        }
                        b"chain_id" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
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
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<RecoveryMessage>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = RecoveryMessage;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct RecoveryMessage",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        u64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct RecoveryMessage with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Vec<IncompleteSignature>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct RecoveryMessage with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Option<ChainId>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct RecoveryMessage with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(RecoveryMessage {
                        nonce: __field0,
                        incomplete_signatures: __field1,
                        chain_id: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        Vec<IncompleteSignature>,
                    > = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<ChainId>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("nonce"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "incomplete_signatures",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<IncompleteSignature>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chain_id",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<ChainId>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("nonce")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field(
                                "incomplete_signatures",
                            )?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("chain_id")?
                        }
                    };
                    _serde::__private::Ok(RecoveryMessage {
                        nonce: __field0,
                        incomplete_signatures: __field1,
                        chain_id: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "nonce",
                "incomplete_signatures",
                "chain_id",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "RecoveryMessage",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<RecoveryMessage>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    extern crate alloy_rlp;
    impl alloy_rlp::Encodable for RecoveryMessage {
        #[inline]
        fn length(&self) -> usize {
            let payload_length = self._alloy_rlp_payload_length();
            payload_length + alloy_rlp::length_of_length(payload_length)
        }
        #[inline]
        fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
            alloy_rlp::Header {
                list: true,
                payload_length: self._alloy_rlp_payload_length(),
            }
                .encode(out);
            alloy_rlp::Encodable::encode(&self.nonce, out);
            alloy_rlp::Encodable::encode(&self.incomplete_signatures, out);
            if let Some(val) = self.chain_id.as_ref() {
                alloy_rlp::Encodable::encode(val, out)
            }
        }
    }
    impl RecoveryMessage {
        #[allow(unused_parens)]
        #[inline]
        fn _alloy_rlp_payload_length(&self) -> usize {
            0usize + alloy_rlp::Encodable::length(&self.nonce)
                + alloy_rlp::Encodable::length(&self.incomplete_signatures)
                + self
                    .chain_id
                    .as_ref()
                    .map(|val| alloy_rlp::Encodable::length(val))
                    .unwrap_or(0)
        }
    }
};
const _: () = {
    extern crate alloy_rlp;
    impl alloy_rlp::Decodable for RecoveryMessage {
        #[inline]
        fn decode(b: &mut &[u8]) -> alloy_rlp::Result<Self> {
            let alloy_rlp::Header { list, payload_length } = alloy_rlp::Header::decode(
                b,
            )?;
            if !list {
                return Err(alloy_rlp::Error::UnexpectedString);
            }
            let started_len = b.len();
            if started_len < payload_length {
                return Err(alloy_rlp::DecodeError::InputTooShort);
            }
            let this = Self {
                nonce: alloy_rlp::Decodable::decode(b)?,
                incomplete_signatures: alloy_rlp::Decodable::decode(b)?,
                chain_id: if started_len - b.len() < payload_length {
                    if alloy_rlp::private::Option::map_or(
                        b.first(),
                        false,
                        |b| *b == 128u8,
                    ) {
                        alloy_rlp::Buf::advance(b, 1);
                        None
                    } else {
                        Some(alloy_rlp::Decodable::decode(b)?)
                    }
                } else {
                    None
                },
            };
            let consumed = started_len - b.len();
            if consumed != payload_length {
                return Err(alloy_rlp::Error::ListLengthMismatch {
                    expected: payload_length,
                    got: consumed,
                });
            }
            Ok(this)
        }
    }
};
pub struct IncompleteSignature {
    pub tx_hash: TxHash,
    pub signatures: Vec<Bytes>,
}
#[automatically_derived]
impl ::core::fmt::Debug for IncompleteSignature {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "IncompleteSignature",
            "tx_hash",
            &self.tx_hash,
            "signatures",
            &&self.signatures,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for IncompleteSignature {
    #[inline]
    fn clone(&self) -> IncompleteSignature {
        IncompleteSignature {
            tx_hash: ::core::clone::Clone::clone(&self.tx_hash),
            signatures: ::core::clone::Clone::clone(&self.signatures),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for IncompleteSignature {}
#[automatically_derived]
impl ::core::cmp::PartialEq for IncompleteSignature {
    #[inline]
    fn eq(&self, other: &IncompleteSignature) -> bool {
        self.tx_hash == other.tx_hash && self.signatures == other.signatures
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for IncompleteSignature {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<TxHash>;
        let _: ::core::cmp::AssertParamIsEq<Vec<Bytes>>;
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for IncompleteSignature {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "IncompleteSignature",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "tx_hash",
                &self.tx_hash,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "signatures",
                &self.signatures,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for IncompleteSignature {
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
                __ignore,
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
                        "field identifier",
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
                        _ => _serde::__private::Ok(__Field::__ignore),
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
                        "tx_hash" => _serde::__private::Ok(__Field::__field0),
                        "signatures" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
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
                        b"tx_hash" => _serde::__private::Ok(__Field::__field0),
                        b"signatures" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
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
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<IncompleteSignature>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = IncompleteSignature;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct IncompleteSignature",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        TxHash,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct IncompleteSignature with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Vec<Bytes>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct IncompleteSignature with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(IncompleteSignature {
                        tx_hash: __field0,
                        signatures: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<TxHash> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Vec<Bytes>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tx_hash",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<TxHash>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "signatures",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<Vec<Bytes>>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("tx_hash")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("signatures")?
                        }
                    };
                    _serde::__private::Ok(IncompleteSignature {
                        tx_hash: __field0,
                        signatures: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["tx_hash", "signatures"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "IncompleteSignature",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<IncompleteSignature>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    extern crate alloy_rlp;
    impl alloy_rlp::Encodable for IncompleteSignature {
        #[inline]
        fn length(&self) -> usize {
            let payload_length = self._alloy_rlp_payload_length();
            payload_length + alloy_rlp::length_of_length(payload_length)
        }
        #[inline]
        fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
            alloy_rlp::Header {
                list: true,
                payload_length: self._alloy_rlp_payload_length(),
            }
                .encode(out);
            alloy_rlp::Encodable::encode(&self.tx_hash, out);
            alloy_rlp::Encodable::encode(&self.signatures, out);
        }
    }
    impl IncompleteSignature {
        #[allow(unused_parens)]
        #[inline]
        fn _alloy_rlp_payload_length(&self) -> usize {
            0usize + alloy_rlp::Encodable::length(&self.tx_hash)
                + alloy_rlp::Encodable::length(&self.signatures)
        }
    }
};
const _: () = {
    extern crate alloy_rlp;
    impl alloy_rlp::Decodable for IncompleteSignature {
        #[inline]
        fn decode(b: &mut &[u8]) -> alloy_rlp::Result<Self> {
            let alloy_rlp::Header { list, payload_length } = alloy_rlp::Header::decode(
                b,
            )?;
            if !list {
                return Err(alloy_rlp::Error::UnexpectedString);
            }
            let started_len = b.len();
            if started_len < payload_length {
                return Err(alloy_rlp::DecodeError::InputTooShort);
            }
            let this = Self {
                tx_hash: alloy_rlp::Decodable::decode(b)?,
                signatures: alloy_rlp::Decodable::decode(b)?,
            };
            let consumed = started_len - b.len();
            if consumed != payload_length {
                return Err(alloy_rlp::Error::ListLengthMismatch {
                    expected: payload_length,
                    got: consumed,
                });
            }
            Ok(this)
        }
    }
};
