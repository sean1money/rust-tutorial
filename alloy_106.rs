#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use alloy_sol_types::{sol, SolCall, SolError, SolEvent, SolStruct, SolType};
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
///Module containing a contract's types and functions.
/**

```solidity
interface Token {
    function transfer(address recipient, uint amount) external;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Token {
    use super::*;
    use ::alloy_sol_types as alloy_sol_types;
    /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address recipient, uint amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct transferCall {
        pub recipient: ::alloy_sol_types::private::Address,
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for transferCall {
        #[inline]
        fn clone(&self) -> transferCall {
            transferCall {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for transferCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "transferCall",
                "recipient",
                &self.recipient,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for transferCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for transferCall {
        #[inline]
        fn eq(&self, other: &transferCall) -> bool {
            self.recipient == other.recipient && self.amount == other.amount
        }
    }
    ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct transferReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for transferReturn {
        #[inline]
        fn clone(&self) -> transferReturn {
            transferReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for transferReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "transferReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for transferReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for transferReturn {
        #[inline]
        fn eq(&self, other: &transferReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::Address,
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.recipient, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`Token`](self) function calls.
    pub enum TokenCalls {
        transfer(transferCall),
    }
    #[automatically_derived]
    impl TokenCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[169u8, 5u8, 156u8, 187u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TokenCalls {
        const NAME: &'static str = "TokenCalls";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<TokenCalls>] = &[
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenCalls::transfer)
                    }
                    transfer
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
}
/**```solidity
struct MetaDataKeyValuePair { string key; string value; }
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct MetaDataKeyValuePair {
    pub key: ::alloy_sol_types::private::String,
    pub value: ::alloy_sol_types::private::String,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for MetaDataKeyValuePair {
    #[inline]
    fn clone(&self) -> MetaDataKeyValuePair {
        MetaDataKeyValuePair {
            key: ::core::clone::Clone::clone(&self.key),
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for MetaDataKeyValuePair {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MetaDataKeyValuePair",
            "key",
            &self.key,
            "value",
            &&self.value,
        )
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for MetaDataKeyValuePair {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for MetaDataKeyValuePair {
    #[inline]
    fn eq(&self, other: &MetaDataKeyValuePair) -> bool {
        self.key == other.key && self.value == other.value
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
const _: () = {
    use ::alloy_sol_types as alloy_sol_types;
    #[doc(hidden)]
    type UnderlyingSolTuple<'a> = (
        ::alloy_sol_types::sol_data::String,
        ::alloy_sol_types::sol_data::String,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = (
        ::alloy_sol_types::private::String,
        ::alloy_sol_types::private::String,
    );
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<MetaDataKeyValuePair> for UnderlyingRustTuple<'_> {
        fn from(value: MetaDataKeyValuePair) -> Self {
            (value.key, value.value)
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<UnderlyingRustTuple<'_>> for MetaDataKeyValuePair {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            Self {
                key: tuple.0,
                value: tuple.1,
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolValue for MetaDataKeyValuePair {
        type SolType = Self;
    }
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<Self> for MetaDataKeyValuePair {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            (
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.key,
                ),
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.value,
                ),
            )
        }
        #[inline]
        fn stv_abi_encoded_size(&self) -> usize {
            if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                return size;
            }
            let tuple = <UnderlyingRustTuple<
                '_,
            > as ::core::convert::From<Self>>::from(self.clone());
            <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
        }
        #[inline]
        fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
            <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
        }
        #[inline]
        fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            let tuple = <UnderlyingRustTuple<
                '_,
            > as ::core::convert::From<Self>>::from(self.clone());
            <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
        }
        #[inline]
        fn stv_abi_packed_encoded_size(&self) -> usize {
            if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                return size;
            }
            let tuple = <UnderlyingRustTuple<
                '_,
            > as ::core::convert::From<Self>>::from(self.clone());
            <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolType for MetaDataKeyValuePair {
        type RustType = Self;
        type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
        const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
        const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
            '_,
        > as alloy_sol_types::SolType>::ENCODED_SIZE;
        const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
            '_,
        > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
        #[inline]
        fn valid_token(token: &Self::Token<'_>) -> bool {
            <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
        }
        #[inline]
        fn detokenize(token: Self::Token<'_>) -> Self::RustType {
            let tuple = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::detokenize(token);
            <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolStruct for MetaDataKeyValuePair {
        const NAME: &'static str = "MetaDataKeyValuePair";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed(
                "MetaDataKeyValuePair(string key,string value)",
            )
        }
        #[inline]
        fn eip712_components() -> alloy_sol_types::private::Vec<
            alloy_sol_types::private::Cow<'static, str>,
        > {
            alloy_sol_types::private::Vec::new()
        }
        #[inline]
        fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
            <Self as alloy_sol_types::SolStruct>::eip712_root_type()
        }
        #[inline]
        fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
            [
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                        &self.key,
                    )
                    .0,
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                        &self.value,
                    )
                    .0,
            ]
                .concat()
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for MetaDataKeyValuePair {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            0usize
                + <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.key,
                )
                + <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.value,
                )
        }
        #[inline]
        fn encode_topic_preimage(
            rust: &Self::RustType,
            out: &mut alloy_sol_types::private::Vec<u8>,
        ) {
            out.reserve(
                <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
            );
            <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.key,
                out,
            );
            <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.value,
                out,
            );
        }
        #[inline]
        fn encode_topic(
            rust: &Self::RustType,
        ) -> alloy_sol_types::abi::token::WordToken {
            let mut out = alloy_sol_types::private::Vec::new();
            <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
            alloy_sol_types::abi::token::WordToken(
                alloy_sol_types::private::keccak256(out),
            )
        }
    }
};
/**```solidity
enum AuthorityType { Master, Mint, Burn, Pause, Blacklist, UpdateMetadata }
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
#[repr(u8)]
pub enum AuthorityType {
    Master,
    Mint,
    Burn,
    Pause,
    Blacklist,
    UpdateMetadata,
    /// Invalid variant.
    ///
    /// This is only used when decoding an out-of-range `u8` value.
    #[doc(hidden)]
    __Invalid = u8::MAX,
}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
impl ::core::clone::Clone for AuthorityType {
    #[inline]
    fn clone(&self) -> AuthorityType {
        *self
    }
}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
impl ::core::marker::Copy for AuthorityType {}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
impl ::core::fmt::Debug for AuthorityType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                AuthorityType::Master => "Master",
                AuthorityType::Mint => "Mint",
                AuthorityType::Burn => "Burn",
                AuthorityType::Pause => "Pause",
                AuthorityType::Blacklist => "Blacklist",
                AuthorityType::UpdateMetadata => "UpdateMetadata",
                AuthorityType::__Invalid => "__Invalid",
            },
        )
    }
}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
impl ::core::marker::StructuralPartialEq for AuthorityType {}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
impl ::core::cmp::PartialEq for AuthorityType {
    #[inline]
    fn eq(&self, other: &AuthorityType) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
const _: () = {
    use ::alloy_sol_types as alloy_sol_types;
    #[automatically_derived]
    impl ::core::convert::From<AuthorityType> for u8 {
        #[inline]
        fn from(v: AuthorityType) -> Self {
            v as u8
        }
    }
    #[automatically_derived]
    impl ::core::convert::TryFrom<u8> for AuthorityType {
        type Error = alloy_sol_types::Error;
        #[inline]
        fn try_from(value: u8) -> alloy_sol_types::Result<Self> {
            match value {
                0u8 => ::core::result::Result::Ok(Self::Master),
                1u8 => ::core::result::Result::Ok(Self::Mint),
                2u8 => ::core::result::Result::Ok(Self::Burn),
                3u8 => ::core::result::Result::Ok(Self::Pause),
                4u8 => ::core::result::Result::Ok(Self::Blacklist),
                5u8 => ::core::result::Result::Ok(Self::UpdateMetadata),
                value => {
                    ::core::result::Result::Err(alloy_sol_types::Error::InvalidEnumValue {
                        name: "AuthorityType",
                        value,
                        max: 5u8,
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolValue for AuthorityType {
        type SolType = Self;
    }
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<AuthorityType> for AuthorityType {
        #[inline]
        fn stv_to_tokens(
            &self,
        ) -> <alloy_sol_types::sol_data::Uint<
            8,
        > as alloy_sol_types::SolType>::Token<'_> {
            alloy_sol_types::Word::with_last_byte(*self as u8).into()
        }
        #[inline]
        fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
            <alloy_sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::eip712_data_word(&(*self as u8))
        }
        #[inline]
        fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            out.push(*self as u8);
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolType for AuthorityType {
        type RustType = AuthorityType;
        type Token<'a> = <alloy_sol_types::sol_data::Uint<
            8,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SOL_NAME: &'static str = <alloy_sol_types::sol_data::Uint<
            8,
        > as alloy_sol_types::SolType>::SOL_NAME;
        const ENCODED_SIZE: ::core::option::Option<usize> = <alloy_sol_types::sol_data::Uint<
            8,
        > as alloy_sol_types::SolType>::ENCODED_SIZE;
        const PACKED_ENCODED_SIZE: ::core::option::Option<usize> = <alloy_sol_types::sol_data::Uint<
            8,
        > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
        #[inline]
        fn valid_token(token: &Self::Token<'_>) -> bool {
            Self::type_check(token).is_ok()
        }
        #[inline]
        fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
            <alloy_sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::type_check(token)?;
            <Self as ::core::convert::TryFrom<
                u8,
            >>::try_from(
                    <alloy_sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::detokenize(*token),
                )
                .map(::core::mem::drop)
        }
        #[inline]
        fn detokenize(token: Self::Token<'_>) -> Self::RustType {
            <Self as ::core::convert::TryFrom<
                u8,
            >>::try_from(
                    <alloy_sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::detokenize(token),
                )
                .unwrap_or(Self::__Invalid)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for AuthorityType {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            <alloy_sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::EventTopic>::topic_preimage_length(&(*rust as u8))
        }
        #[inline]
        fn encode_topic_preimage(
            rust: &Self::RustType,
            out: &mut alloy_sol_types::private::Vec<u8>,
        ) {
            <alloy_sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &(*rust as u8),
                out,
            );
        }
        #[inline]
        fn encode_topic(
            rust: &Self::RustType,
        ) -> alloy_sol_types::abi::token::WordToken {
            <alloy_sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::EventTopic>::encode_topic(&(*rust as u8))
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEnum for AuthorityType {
        const COUNT: usize = 6usize;
    }
};
///Module containing a contract's types and functions.
/**

```solidity
interface TokenMint {
    function createNewToken(string symbol, uint8 decimals, address master_authority) external;
    function grantAuthority(AuthorityType authority_type, address new_authority, uint amount) external;
    function revokeAuthority(AuthorityType authority_type, address new_authority) external;
    function blacklistAccount(address account) external;
    function whitelistAccount(address account) external;
    function mintTo(uint amount, address account) external;
    function burnFromAccount(uint amount, address account) external;
    function closeAccount() external;
    function pause() external;
    function unpause() external;
    function updateMetadata(string name, string uri, MetaDataKeyValuePair[] additional_metadata) external;
    function transfer(address recipient, uint amount) external;
    function name() external returns (string);
    function symbol() external returns (string);
    function decimals() external returns (uint8);
    function totalSupply() external returns (uint256);
    function balanceOf(address owner) external returns (uint256 balance);
    event Transfer(address _from, address _to, uint256 _value);
    event Failure(address _from, address _to, string message);
    event Success(address _from, address _to, string message);
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod TokenMint {
    use super::*;
    use ::alloy_sol_types as alloy_sol_types;
    /**Function with signature `createNewToken(string,uint8,address)` and selector `0x8d83b1d4`.
```solidity
function createNewToken(string symbol, uint8 decimals, address master_authority) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct createNewTokenCall {
        pub symbol: ::alloy_sol_types::private::String,
        pub decimals: u8,
        pub master_authority: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for createNewTokenCall {
        #[inline]
        fn clone(&self) -> createNewTokenCall {
            createNewTokenCall {
                symbol: ::core::clone::Clone::clone(&self.symbol),
                decimals: ::core::clone::Clone::clone(&self.decimals),
                master_authority: ::core::clone::Clone::clone(&self.master_authority),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for createNewTokenCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "createNewTokenCall",
                "symbol",
                &self.symbol,
                "decimals",
                &self.decimals,
                "master_authority",
                &&self.master_authority,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for createNewTokenCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for createNewTokenCall {
        #[inline]
        fn eq(&self, other: &createNewTokenCall) -> bool {
            self.symbol == other.symbol && self.decimals == other.decimals
                && self.master_authority == other.master_authority
        }
    }
    ///Container type for the return parameters of the [`createNewToken(string,uint8,address)`](createNewTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct createNewTokenReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for createNewTokenReturn {
        #[inline]
        fn clone(&self) -> createNewTokenReturn {
            createNewTokenReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for createNewTokenReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "createNewTokenReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for createNewTokenReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for createNewTokenReturn {
        #[inline]
        fn eq(&self, other: &createNewTokenReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ::alloy_sol_types::sol_data::String,
                ::alloy_sol_types::sol_data::Uint<8>,
                ::alloy_sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::String,
                u8,
                ::alloy_sol_types::private::Address,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createNewTokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTokenCall) -> Self {
                    (value.symbol, value.decimals, value.master_authority)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        symbol: tuple.0,
                        decimals: tuple.1,
                        master_authority: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createNewTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createNewTokenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createNewTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTokenCall {
            type Parameters<'a> = (
                ::alloy_sol_types::sol_data::String,
                ::alloy_sol_types::sol_data::Uint<8>,
                ::alloy_sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTokenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewToken(string,uint8,address)";
            const SELECTOR: [u8; 4] = [141u8, 131u8, 177u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.symbol,
                    ),
                    <::alloy_sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.master_authority,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `grantAuthority(uint8,address,uint256)` and selector `0x629792a1`.
```solidity
function grantAuthority(AuthorityType authority_type, address new_authority, uint amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct grantAuthorityCall {
        pub authority_type: <AuthorityType as ::alloy_sol_types::SolType>::RustType,
        pub new_authority: ::alloy_sol_types::private::Address,
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for grantAuthorityCall {
        #[inline]
        fn clone(&self) -> grantAuthorityCall {
            grantAuthorityCall {
                authority_type: ::core::clone::Clone::clone(&self.authority_type),
                new_authority: ::core::clone::Clone::clone(&self.new_authority),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for grantAuthorityCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "grantAuthorityCall",
                "authority_type",
                &self.authority_type,
                "new_authority",
                &self.new_authority,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for grantAuthorityCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for grantAuthorityCall {
        #[inline]
        fn eq(&self, other: &grantAuthorityCall) -> bool {
            self.authority_type == other.authority_type
                && self.new_authority == other.new_authority
                && self.amount == other.amount
        }
    }
    ///Container type for the return parameters of the [`grantAuthority(uint8,address,uint256)`](grantAuthorityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct grantAuthorityReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for grantAuthorityReturn {
        #[inline]
        fn clone(&self) -> grantAuthorityReturn {
            grantAuthorityReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for grantAuthorityReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "grantAuthorityReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for grantAuthorityReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for grantAuthorityReturn {
        #[inline]
        fn eq(&self, other: &grantAuthorityReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                AuthorityType,
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <AuthorityType as ::alloy_sol_types::SolType>::RustType,
                ::alloy_sol_types::private::Address,
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantAuthorityCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantAuthorityCall) -> Self {
                    (value.authority_type, value.new_authority, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantAuthorityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        authority_type: tuple.0,
                        new_authority: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantAuthorityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: grantAuthorityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for grantAuthorityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantAuthorityCall {
            type Parameters<'a> = (
                AuthorityType,
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantAuthorityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantAuthority(uint8,address,uint256)";
            const SELECTOR: [u8; 4] = [98u8, 151u8, 146u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <AuthorityType as alloy_sol_types::SolType>::tokenize(
                        &self.authority_type,
                    ),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.new_authority,
                    ),
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `revokeAuthority(uint8,address)` and selector `0xb7a6e424`.
```solidity
function revokeAuthority(AuthorityType authority_type, address new_authority) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct revokeAuthorityCall {
        pub authority_type: <AuthorityType as ::alloy_sol_types::SolType>::RustType,
        pub new_authority: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for revokeAuthorityCall {
        #[inline]
        fn clone(&self) -> revokeAuthorityCall {
            revokeAuthorityCall {
                authority_type: ::core::clone::Clone::clone(&self.authority_type),
                new_authority: ::core::clone::Clone::clone(&self.new_authority),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for revokeAuthorityCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "revokeAuthorityCall",
                "authority_type",
                &self.authority_type,
                "new_authority",
                &&self.new_authority,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for revokeAuthorityCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for revokeAuthorityCall {
        #[inline]
        fn eq(&self, other: &revokeAuthorityCall) -> bool {
            self.authority_type == other.authority_type
                && self.new_authority == other.new_authority
        }
    }
    ///Container type for the return parameters of the [`revokeAuthority(uint8,address)`](revokeAuthorityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct revokeAuthorityReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for revokeAuthorityReturn {
        #[inline]
        fn clone(&self) -> revokeAuthorityReturn {
            revokeAuthorityReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for revokeAuthorityReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "revokeAuthorityReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for revokeAuthorityReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for revokeAuthorityReturn {
        #[inline]
        fn eq(&self, other: &revokeAuthorityReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                AuthorityType,
                ::alloy_sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <AuthorityType as ::alloy_sol_types::SolType>::RustType,
                ::alloy_sol_types::private::Address,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeAuthorityCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeAuthorityCall) -> Self {
                    (value.authority_type, value.new_authority)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeAuthorityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        authority_type: tuple.0,
                        new_authority: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeAuthorityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: revokeAuthorityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for revokeAuthorityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeAuthorityCall {
            type Parameters<'a> = (AuthorityType, ::alloy_sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeAuthorityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeAuthority(uint8,address)";
            const SELECTOR: [u8; 4] = [183u8, 166u8, 228u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <AuthorityType as alloy_sol_types::SolType>::tokenize(
                        &self.authority_type,
                    ),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.new_authority,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `blacklistAccount(address)` and selector `0xd37b34d7`.
```solidity
function blacklistAccount(address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct blacklistAccountCall {
        pub account: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for blacklistAccountCall {
        #[inline]
        fn clone(&self) -> blacklistAccountCall {
            blacklistAccountCall {
                account: ::core::clone::Clone::clone(&self.account),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for blacklistAccountCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "blacklistAccountCall",
                "account",
                &&self.account,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for blacklistAccountCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for blacklistAccountCall {
        #[inline]
        fn eq(&self, other: &blacklistAccountCall) -> bool {
            self.account == other.account
        }
    }
    ///Container type for the return parameters of the [`blacklistAccount(address)`](blacklistAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct blacklistAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for blacklistAccountReturn {
        #[inline]
        fn clone(&self) -> blacklistAccountReturn {
            blacklistAccountReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for blacklistAccountReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "blacklistAccountReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for blacklistAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for blacklistAccountReturn {
        #[inline]
        fn eq(&self, other: &blacklistAccountReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::Address,);
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blacklistAccountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: blacklistAccountCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blacklistAccountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blacklistAccountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blacklistAccountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blacklistAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blacklistAccountCall {
            type Parameters<'a> = (::alloy_sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = blacklistAccountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blacklistAccount(address)";
            const SELECTOR: [u8; 4] = [211u8, 123u8, 52u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `whitelistAccount(address)` and selector `0x63e0c2f8`.
```solidity
function whitelistAccount(address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct whitelistAccountCall {
        pub account: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for whitelistAccountCall {
        #[inline]
        fn clone(&self) -> whitelistAccountCall {
            whitelistAccountCall {
                account: ::core::clone::Clone::clone(&self.account),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for whitelistAccountCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "whitelistAccountCall",
                "account",
                &&self.account,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for whitelistAccountCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for whitelistAccountCall {
        #[inline]
        fn eq(&self, other: &whitelistAccountCall) -> bool {
            self.account == other.account
        }
    }
    ///Container type for the return parameters of the [`whitelistAccount(address)`](whitelistAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct whitelistAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for whitelistAccountReturn {
        #[inline]
        fn clone(&self) -> whitelistAccountReturn {
            whitelistAccountReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for whitelistAccountReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "whitelistAccountReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for whitelistAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for whitelistAccountReturn {
        #[inline]
        fn eq(&self, other: &whitelistAccountReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::Address,);
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<whitelistAccountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: whitelistAccountCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for whitelistAccountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<whitelistAccountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: whitelistAccountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for whitelistAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for whitelistAccountCall {
            type Parameters<'a> = (::alloy_sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = whitelistAccountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "whitelistAccount(address)";
            const SELECTOR: [u8; 4] = [99u8, 224u8, 194u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mintTo(uint256,address)` and selector `0xb723b34e`.
```solidity
function mintTo(uint amount, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct mintToCall {
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
        pub account: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for mintToCall {
        #[inline]
        fn clone(&self) -> mintToCall {
            mintToCall {
                amount: ::core::clone::Clone::clone(&self.amount),
                account: ::core::clone::Clone::clone(&self.account),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for mintToCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "mintToCall",
                "amount",
                &self.amount,
                "account",
                &&self.account,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for mintToCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for mintToCall {
        #[inline]
        fn eq(&self, other: &mintToCall) -> bool {
            self.amount == other.amount && self.account == other.account
        }
    }
    ///Container type for the return parameters of the [`mintTo(uint256,address)`](mintToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct mintToReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for mintToReturn {
        #[inline]
        fn clone(&self) -> mintToReturn {
            mintToReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for mintToReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "mintToReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for mintToReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for mintToReturn {
        #[inline]
        fn eq(&self, other: &mintToReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ::alloy_sol_types::sol_data::Uint<256>,
                ::alloy_sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
                ::alloy_sol_types::private::Address,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintToCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintToCall) -> Self {
                    (value.amount, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amount: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintToCall {
            type Parameters<'a> = (
                ::alloy_sol_types::sol_data::Uint<256>,
                ::alloy_sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mintTo(uint256,address)";
            const SELECTOR: [u8; 4] = [183u8, 35u8, 179u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `burnFromAccount(uint256,address)` and selector `0xdf708e5c`.
```solidity
function burnFromAccount(uint amount, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct burnFromAccountCall {
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
        pub account: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for burnFromAccountCall {
        #[inline]
        fn clone(&self) -> burnFromAccountCall {
            burnFromAccountCall {
                amount: ::core::clone::Clone::clone(&self.amount),
                account: ::core::clone::Clone::clone(&self.account),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for burnFromAccountCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "burnFromAccountCall",
                "amount",
                &self.amount,
                "account",
                &&self.account,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for burnFromAccountCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for burnFromAccountCall {
        #[inline]
        fn eq(&self, other: &burnFromAccountCall) -> bool {
            self.amount == other.amount && self.account == other.account
        }
    }
    ///Container type for the return parameters of the [`burnFromAccount(uint256,address)`](burnFromAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct burnFromAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for burnFromAccountReturn {
        #[inline]
        fn clone(&self) -> burnFromAccountReturn {
            burnFromAccountReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for burnFromAccountReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "burnFromAccountReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for burnFromAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for burnFromAccountReturn {
        #[inline]
        fn eq(&self, other: &burnFromAccountReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ::alloy_sol_types::sol_data::Uint<256>,
                ::alloy_sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
                ::alloy_sol_types::private::Address,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnFromAccountCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnFromAccountCall) -> Self {
                    (value.amount, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnFromAccountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amount: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnFromAccountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: burnFromAccountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for burnFromAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnFromAccountCall {
            type Parameters<'a> = (
                ::alloy_sol_types::sol_data::Uint<256>,
                ::alloy_sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnFromAccountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burnFromAccount(uint256,address)";
            const SELECTOR: [u8; 4] = [223u8, 112u8, 142u8, 92u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `closeAccount()` and selector `0x6dd22fc6`.
```solidity
function closeAccount() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct closeAccountCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for closeAccountCall {
        #[inline]
        fn clone(&self) -> closeAccountCall {
            closeAccountCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for closeAccountCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "closeAccountCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for closeAccountCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for closeAccountCall {
        #[inline]
        fn eq(&self, other: &closeAccountCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`closeAccount()`](closeAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct closeAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for closeAccountReturn {
        #[inline]
        fn clone(&self) -> closeAccountReturn {
            closeAccountReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for closeAccountReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "closeAccountReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for closeAccountReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for closeAccountReturn {
        #[inline]
        fn eq(&self, other: &closeAccountReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeAccountCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeAccountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeAccountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeAccountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeAccountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeAccountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeAccountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeAccount()";
            const SELECTOR: [u8; 4] = [109u8, 210u8, 47u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct pauseCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for pauseCall {
        #[inline]
        fn clone(&self) -> pauseCall {
            pauseCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for pauseCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "pauseCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for pauseCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for pauseCall {
        #[inline]
        fn eq(&self, other: &pauseCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct pauseReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for pauseReturn {
        #[inline]
        fn clone(&self) -> pauseReturn {
            pauseReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for pauseReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "pauseReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for pauseReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for pauseReturn {
        #[inline]
        fn eq(&self, other: &pauseReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct unpauseCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for unpauseCall {
        #[inline]
        fn clone(&self) -> unpauseCall {
            unpauseCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for unpauseCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "unpauseCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for unpauseCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for unpauseCall {
        #[inline]
        fn eq(&self, other: &unpauseCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct unpauseReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for unpauseReturn {
        #[inline]
        fn clone(&self) -> unpauseReturn {
            unpauseReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for unpauseReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "unpauseReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for unpauseReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for unpauseReturn {
        #[inline]
        fn eq(&self, other: &unpauseReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateMetadata(string,string,(string,string)[])` and selector `0xd96a9cfe`.
```solidity
function updateMetadata(string name, string uri, MetaDataKeyValuePair[] additional_metadata) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct updateMetadataCall {
        pub name: ::alloy_sol_types::private::String,
        pub uri: ::alloy_sol_types::private::String,
        pub additional_metadata: ::alloy_sol_types::private::Vec<
            <MetaDataKeyValuePair as ::alloy_sol_types::SolType>::RustType,
        >,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for updateMetadataCall {
        #[inline]
        fn clone(&self) -> updateMetadataCall {
            updateMetadataCall {
                name: ::core::clone::Clone::clone(&self.name),
                uri: ::core::clone::Clone::clone(&self.uri),
                additional_metadata: ::core::clone::Clone::clone(
                    &self.additional_metadata,
                ),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for updateMetadataCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "updateMetadataCall",
                "name",
                &self.name,
                "uri",
                &self.uri,
                "additional_metadata",
                &&self.additional_metadata,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for updateMetadataCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for updateMetadataCall {
        #[inline]
        fn eq(&self, other: &updateMetadataCall) -> bool {
            self.name == other.name && self.uri == other.uri
                && self.additional_metadata == other.additional_metadata
        }
    }
    ///Container type for the return parameters of the [`updateMetadata(string,string,(string,string)[])`](updateMetadataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct updateMetadataReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for updateMetadataReturn {
        #[inline]
        fn clone(&self) -> updateMetadataReturn {
            updateMetadataReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for updateMetadataReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "updateMetadataReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for updateMetadataReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for updateMetadataReturn {
        #[inline]
        fn eq(&self, other: &updateMetadataReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ::alloy_sol_types::sol_data::String,
                ::alloy_sol_types::sol_data::String,
                ::alloy_sol_types::sol_data::Array<MetaDataKeyValuePair>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::String,
                ::alloy_sol_types::private::String,
                ::alloy_sol_types::private::Vec<
                    <MetaDataKeyValuePair as ::alloy_sol_types::SolType>::RustType,
                >,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMetadataCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateMetadataCall) -> Self {
                    (value.name, value.uri, value.additional_metadata)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMetadataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        name: tuple.0,
                        uri: tuple.1,
                        additional_metadata: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMetadataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMetadataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMetadataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMetadataCall {
            type Parameters<'a> = (
                ::alloy_sol_types::sol_data::String,
                ::alloy_sol_types::sol_data::String,
                ::alloy_sol_types::sol_data::Array<MetaDataKeyValuePair>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMetadataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMetadata(string,string,(string,string)[])";
            const SELECTOR: [u8; 4] = [217u8, 106u8, 156u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.uri,
                    ),
                    <::alloy_sol_types::sol_data::Array<
                        MetaDataKeyValuePair,
                    > as alloy_sol_types::SolType>::tokenize(&self.additional_metadata),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address recipient, uint amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct transferCall {
        pub recipient: ::alloy_sol_types::private::Address,
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for transferCall {
        #[inline]
        fn clone(&self) -> transferCall {
            transferCall {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for transferCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "transferCall",
                "recipient",
                &self.recipient,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for transferCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for transferCall {
        #[inline]
        fn eq(&self, other: &transferCall) -> bool {
            self.recipient == other.recipient && self.amount == other.amount
        }
    }
    ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct transferReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for transferReturn {
        #[inline]
        fn clone(&self) -> transferReturn {
            transferReturn {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for transferReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "transferReturn")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for transferReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for transferReturn {
        #[inline]
        fn eq(&self, other: &transferReturn) -> bool {
            true
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::Address,
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.recipient, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external returns (string);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct nameCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for nameCall {
        #[inline]
        fn clone(&self) -> nameCall {
            nameCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for nameCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "nameCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for nameCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for nameCall {
        #[inline]
        fn eq(&self, other: &nameCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct nameReturn {
        pub _0: ::alloy_sol_types::private::String,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for nameReturn {
        #[inline]
        fn clone(&self) -> nameReturn {
            nameReturn {
                _0: ::core::clone::Clone::clone(&self._0),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for nameReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "nameReturn",
                "_0",
                &&self._0,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for nameReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for nameReturn {
        #[inline]
        fn eq(&self, other: &nameReturn) -> bool {
            self._0 == other._0
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::String,);
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nameReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external returns (string);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct symbolCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for symbolCall {
        #[inline]
        fn clone(&self) -> symbolCall {
            symbolCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for symbolCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "symbolCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for symbolCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for symbolCall {
        #[inline]
        fn eq(&self, other: &symbolCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct symbolReturn {
        pub _0: ::alloy_sol_types::private::String,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for symbolReturn {
        #[inline]
        fn clone(&self) -> symbolReturn {
            symbolReturn {
                _0: ::core::clone::Clone::clone(&self._0),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for symbolReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "symbolReturn",
                "_0",
                &&self._0,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for symbolReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for symbolReturn {
        #[inline]
        fn eq(&self, other: &symbolReturn) -> bool {
            self._0 == other._0
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                fn from(value: symbolCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::String,);
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: symbolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for symbolCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = symbolReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "symbol()";
            const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct decimalsCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for decimalsCall {
        #[inline]
        fn clone(&self) -> decimalsCall {
            decimalsCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for decimalsCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "decimalsCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for decimalsCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for decimalsCall {
        #[inline]
        fn eq(&self, other: &decimalsCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`decimals()`](decimalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct decimalsReturn {
        pub _0: u8,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for decimalsReturn {
        #[inline]
        fn clone(&self) -> decimalsReturn {
            decimalsReturn {
                _0: ::core::clone::Clone::clone(&self._0),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for decimalsReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "decimalsReturn",
                "_0",
                &&self._0,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for decimalsReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for decimalsReturn {
        #[inline]
        fn eq(&self, other: &decimalsReturn) -> bool {
            self._0 == other._0
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decimalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decimalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decimalsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decimalsReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decimals()";
            const SELECTOR: [u8; 4] = [49u8, 60u8, 229u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct totalSupplyCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for totalSupplyCall {
        #[inline]
        fn clone(&self) -> totalSupplyCall {
            totalSupplyCall {}
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for totalSupplyCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "totalSupplyCall")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for totalSupplyCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for totalSupplyCall {
        #[inline]
        fn eq(&self, other: &totalSupplyCall) -> bool {
            true
        }
    }
    ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct totalSupplyReturn {
        pub _0: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for totalSupplyReturn {
        #[inline]
        fn clone(&self) -> totalSupplyReturn {
            totalSupplyReturn {
                _0: ::core::clone::Clone::clone(&self._0),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for totalSupplyReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "totalSupplyReturn",
                "_0",
                &&self._0,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for totalSupplyReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for totalSupplyReturn {
        #[inline]
        fn eq(&self, other: &totalSupplyReturn) -> bool {
            self._0 == other._0
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalSupplyReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalSupply()";
            const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address owner) external returns (uint256 balance);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct balanceOfCall {
        pub owner: ::alloy_sol_types::private::Address,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for balanceOfCall {
        #[inline]
        fn clone(&self) -> balanceOfCall {
            balanceOfCall {
                owner: ::core::clone::Clone::clone(&self.owner),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for balanceOfCall {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "balanceOfCall",
                "owner",
                &&self.owner,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for balanceOfCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for balanceOfCall {
        #[inline]
        fn eq(&self, other: &balanceOfCall) -> bool {
            self.owner == other.owner
        }
    }
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct balanceOfReturn {
        pub balance: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for balanceOfReturn {
        #[inline]
        fn clone(&self) -> balanceOfReturn {
            balanceOfReturn {
                balance: ::core::clone::Clone::clone(&self.balance),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::fmt::Debug for balanceOfReturn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "balanceOfReturn",
                "balance",
                &&self.balance,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::marker::StructuralPartialEq for balanceOfReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::cmp::PartialEq for balanceOfReturn {
        #[inline]
        fn eq(&self, other: &balanceOfReturn) -> bool {
            self.balance == other.balance
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::Address,);
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value.balance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { balance: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (::alloy_sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address _from, address _to, uint256 _value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub _from: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub _to: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub _value: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::clone::Clone for Transfer {
        #[inline]
        fn clone(&self) -> Transfer {
            Transfer {
                _from: ::core::clone::Clone::clone(&self._from),
                _to: ::core::clone::Clone::clone(&self._to),
                _value: ::core::clone::Clone::clone(&self._value),
            }
        }
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::fmt::Debug for Transfer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Transfer",
                "_from",
                &self._from,
                "_to",
                &self._to,
                "_value",
                &&self._value,
            )
        }
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::marker::StructuralPartialEq for Transfer {}
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::cmp::PartialEq for Transfer {
        #[inline]
        fn eq(&self, other: &Transfer) -> bool {
            self._from == other._from && self._to == other._to
                && self._value == other._value
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _from: data.0,
                    _to: data.1,
                    _value: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._from,
                    ),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._to,
                    ),
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Failure(address,address,string)` and selector `0xac2b023b39b7342d071b56fe3ea3e809a12ae9f14e5b387fd7a2f63d32cbe14d`.
```solidity
event Failure(address _from, address _to, string message);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    pub struct Failure {
        #[allow(missing_docs)]
        pub _from: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub _to: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub message: ::alloy_sol_types::private::String,
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::clone::Clone for Failure {
        #[inline]
        fn clone(&self) -> Failure {
            Failure {
                _from: ::core::clone::Clone::clone(&self._from),
                _to: ::core::clone::Clone::clone(&self._to),
                message: ::core::clone::Clone::clone(&self.message),
            }
        }
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::fmt::Debug for Failure {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Failure",
                "_from",
                &self._from,
                "_to",
                &self._to,
                "message",
                &&self.message,
            )
        }
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::marker::StructuralPartialEq for Failure {}
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::cmp::PartialEq for Failure {
        #[inline]
        fn eq(&self, other: &Failure) -> bool {
            self._from == other._from && self._to == other._to
                && self.message == other.message
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Failure {
            type DataTuple<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Failure(address,address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                172u8,
                43u8,
                2u8,
                59u8,
                57u8,
                183u8,
                52u8,
                45u8,
                7u8,
                27u8,
                86u8,
                254u8,
                62u8,
                163u8,
                232u8,
                9u8,
                161u8,
                42u8,
                233u8,
                241u8,
                78u8,
                91u8,
                56u8,
                127u8,
                215u8,
                162u8,
                246u8,
                61u8,
                50u8,
                203u8,
                225u8,
                77u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _from: data.0,
                    _to: data.1,
                    message: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._from,
                    ),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._to,
                    ),
                    <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Failure {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Failure> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Failure) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Success(address,address,string)` and selector `0x250bf4b2488bf4c0d40c5698ffbdd9538361bf0c1d6a7ac473e87b55dd8030fd`.
```solidity
event Success(address _from, address _to, string message);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    pub struct Success {
        #[allow(missing_docs)]
        pub _from: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub _to: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub message: ::alloy_sol_types::private::String,
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::clone::Clone for Success {
        #[inline]
        fn clone(&self) -> Success {
            Success {
                _from: ::core::clone::Clone::clone(&self._from),
                _to: ::core::clone::Clone::clone(&self._to),
                message: ::core::clone::Clone::clone(&self.message),
            }
        }
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::fmt::Debug for Success {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Success",
                "_from",
                &self._from,
                "_to",
                &self._to,
                "message",
                &&self.message,
            )
        }
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::marker::StructuralPartialEq for Success {}
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::cmp::PartialEq for Success {
        #[inline]
        fn eq(&self, other: &Success) -> bool {
            self._from == other._from && self._to == other._to
                && self.message == other.message
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Success {
            type DataTuple<'a> = (
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Success(address,address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                37u8,
                11u8,
                244u8,
                178u8,
                72u8,
                139u8,
                244u8,
                192u8,
                212u8,
                12u8,
                86u8,
                152u8,
                255u8,
                189u8,
                217u8,
                83u8,
                131u8,
                97u8,
                191u8,
                12u8,
                29u8,
                106u8,
                122u8,
                196u8,
                115u8,
                232u8,
                123u8,
                85u8,
                221u8,
                128u8,
                48u8,
                253u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _from: data.0,
                    _to: data.1,
                    message: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._from,
                    ),
                    <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._to,
                    ),
                    <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Success {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Success> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Success) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    ///Container for all the [`TokenMint`](self) function calls.
    pub enum TokenMintCalls {
        createNewToken(createNewTokenCall),
        grantAuthority(grantAuthorityCall),
        revokeAuthority(revokeAuthorityCall),
        blacklistAccount(blacklistAccountCall),
        whitelistAccount(whitelistAccountCall),
        mintTo(mintToCall),
        burnFromAccount(burnFromAccountCall),
        closeAccount(closeAccountCall),
        pause(pauseCall),
        unpause(unpauseCall),
        updateMetadata(updateMetadataCall),
        transfer(transferCall),
        name(nameCall),
        symbol(symbolCall),
        decimals(decimalsCall),
        totalSupply(totalSupplyCall),
        balanceOf(balanceOfCall),
    }
    #[automatically_derived]
    impl TokenMintCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 253u8, 222u8, 3u8],
            [24u8, 22u8, 13u8, 221u8],
            [49u8, 60u8, 229u8, 103u8],
            [63u8, 75u8, 168u8, 58u8],
            [98u8, 151u8, 146u8, 161u8],
            [99u8, 224u8, 194u8, 248u8],
            [109u8, 210u8, 47u8, 198u8],
            [112u8, 160u8, 130u8, 49u8],
            [132u8, 86u8, 203u8, 89u8],
            [141u8, 131u8, 177u8, 212u8],
            [149u8, 216u8, 155u8, 65u8],
            [169u8, 5u8, 156u8, 187u8],
            [183u8, 35u8, 179u8, 78u8],
            [183u8, 166u8, 228u8, 36u8],
            [211u8, 123u8, 52u8, 215u8],
            [217u8, 106u8, 156u8, 254u8],
            [223u8, 112u8, 142u8, 92u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TokenMintCalls {
        const NAME: &'static str = "TokenMintCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::createNewToken(_) => {
                    <createNewTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantAuthority(_) => {
                    <grantAuthorityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeAuthority(_) => {
                    <revokeAuthorityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blacklistAccount(_) => {
                    <blacklistAccountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::whitelistAccount(_) => {
                    <whitelistAccountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mintTo(_) => <mintToCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::burnFromAccount(_) => {
                    <burnFromAccountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::closeAccount(_) => {
                    <closeAccountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateMetadata(_) => {
                    <updateMetadataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::decimals(_) => <decimalsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::totalSupply(_) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<TokenMintCalls>] = &[
                {
                    fn name(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::name)
                    }
                    name
                },
                {
                    fn totalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn decimals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::decimals)
                    }
                    decimals
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::unpause)
                    }
                    unpause
                },
                {
                    fn grantAuthority(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <grantAuthorityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::grantAuthority)
                    }
                    grantAuthority
                },
                {
                    fn whitelistAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <whitelistAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::whitelistAccount)
                    }
                    whitelistAccount
                },
                {
                    fn closeAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <closeAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::closeAccount)
                    }
                    closeAccount
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::pause)
                    }
                    pause
                },
                {
                    fn createNewToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <createNewTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::createNewToken)
                    }
                    createNewToken
                },
                {
                    fn symbol(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::symbol)
                    }
                    symbol
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::transfer)
                    }
                    transfer
                },
                {
                    fn mintTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <mintToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::mintTo)
                    }
                    mintTo
                },
                {
                    fn revokeAuthority(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <revokeAuthorityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::revokeAuthority)
                    }
                    revokeAuthority
                },
                {
                    fn blacklistAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <blacklistAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::blacklistAccount)
                    }
                    blacklistAccount
                },
                {
                    fn updateMetadata(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <updateMetadataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::updateMetadata)
                    }
                    updateMetadata
                },
                {
                    fn burnFromAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TokenMintCalls> {
                        <burnFromAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TokenMintCalls::burnFromAccount)
                    }
                    burnFromAccount
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::createNewToken(inner) => {
                    <createNewTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantAuthority(inner) => {
                    <grantAuthorityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeAuthority(inner) => {
                    <revokeAuthorityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blacklistAccount(inner) => {
                    <blacklistAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::whitelistAccount(inner) => {
                    <whitelistAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mintTo(inner) => {
                    <mintToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burnFromAccount(inner) => {
                    <burnFromAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeAccount(inner) => {
                    <closeAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateMetadata(inner) => {
                    <updateMetadataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::createNewToken(inner) => {
                    <createNewTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantAuthority(inner) => {
                    <grantAuthorityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeAuthority(inner) => {
                    <revokeAuthorityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blacklistAccount(inner) => {
                    <blacklistAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::whitelistAccount(inner) => {
                    <whitelistAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mintTo(inner) => {
                    <mintToCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::burnFromAccount(inner) => {
                    <burnFromAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::closeAccount(inner) => {
                    <closeAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateMetadata(inner) => {
                    <updateMetadataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`TokenMint`](self) events.
    pub enum TokenMintEvents {
        Transfer(Transfer),
        Failure(Failure),
        Success(Success),
    }
    #[automatically_derived]
    impl TokenMintEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                37u8,
                11u8,
                244u8,
                178u8,
                72u8,
                139u8,
                244u8,
                192u8,
                212u8,
                12u8,
                86u8,
                152u8,
                255u8,
                189u8,
                217u8,
                83u8,
                131u8,
                97u8,
                191u8,
                12u8,
                29u8,
                106u8,
                122u8,
                196u8,
                115u8,
                232u8,
                123u8,
                85u8,
                221u8,
                128u8,
                48u8,
                253u8,
            ],
            [
                172u8,
                43u8,
                2u8,
                59u8,
                57u8,
                183u8,
                52u8,
                45u8,
                7u8,
                27u8,
                86u8,
                254u8,
                62u8,
                163u8,
                232u8,
                9u8,
                161u8,
                42u8,
                233u8,
                241u8,
                78u8,
                91u8,
                56u8,
                127u8,
                215u8,
                162u8,
                246u8,
                61u8,
                50u8,
                203u8,
                225u8,
                77u8,
            ],
            [
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for TokenMintEvents {
        const NAME: &'static str = "TokenMintEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
                }
                Some(<Failure as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Failure as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Failure)
                }
                Some(<Success as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Success as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Success)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for TokenMintEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Failure(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Success(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Failure(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Success(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
}
