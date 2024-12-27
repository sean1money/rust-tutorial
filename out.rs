#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use alloy_primitives::{keccak256, TxHash, U256};
use alloy_sol_types::{sol, SolCall};
use anyhow::Result;
fn main() -> Result<()> {
    {
        ::std::io::_print(format_args!("Hello, alloy sol types!\n"));
    };
    let call = builtinCancellationCall {
        hash: TxHash::ZERO,
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", hex::encode(call.abi_encode())));
    };
    {
        ::std::io::_print(
            format_args!("{0:?}\n", builtinAccountRecoveryCall::SIGNATURE),
        );
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", builtinAccountRecoveryCall::SELECTOR));
    };
    {
        ::std::io::_print(
            format_args!("{0:?}\n", keccak256(builtinAccountRecoveryCall::SIGNATURE)),
        );
    };
    {
        ::std::io::_print(
            format_args!("0x{0}\n", hex::encode(builtinAccountRecoveryCall::SELECTOR)),
        );
    };
    let half_tx1 = HalfTx {
        hash: TxHash::ZERO,
        signatures: ::alloc::vec::Vec::new(),
    };
    let call: builtinAccountRecoveryCall = builtinAccountRecoveryCall {
        halfTxs: <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([half_tx1])),
    };
    let encoded = call.abi_encode();
    {
        ::std::io::_print(format_args!("{0:?}\n", encoded));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", encoded.len()));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", hex::encode(encoded)));
    };
    let abi_string = "625234b1000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000";
    let decoded = builtinAccountRecoveryCall::abi_decode(
        &hex::decode(abi_string)?,
        true,
    );
    {
        ::std::io::_print(format_args!("{0:?}\n", decoded));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", systemGovernanceCall::SIGNATURE));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", systemGovernanceCall::SELECTOR));
    };
    {
        ::std::io::_print(
            format_args!("0x{0}\n", hex::encode(systemGovernanceCall::SELECTOR)),
        );
    };
    let balance = balanceOfRet(U256::from(1000));
    {
        ::std::io::_print(format_args!("{0:?}\n", hex::encode(balance.abi_encode())));
    };
    {
        ::std::io::_print(
            format_args!("{0:?}\n", hex::encode(balance.abi_encode_packed())),
        );
    };
    Ok(())
}
#[allow(missing_docs)]
/**Function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`.
```solidity
function getRoundData(uint80 _roundId) external view returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct getRoundDataCall {
    pub _roundId: ::alloy_sol_types::private::primitives::aliases::U80,
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for getRoundDataCall {
    #[inline]
    fn clone(&self) -> getRoundDataCall {
        getRoundDataCall {
            _roundId: ::core::clone::Clone::clone(&self._roundId),
        }
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for getRoundDataCall {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "getRoundDataCall",
            "_roundId",
            &&self._roundId,
        )
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for getRoundDataCall {}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for getRoundDataCall {
    #[inline]
    fn eq(&self, other: &getRoundDataCall) -> bool {
        self._roundId == other._roundId
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for getRoundDataCall {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::primitives::aliases::U80,
        >;
    }
}
#[allow(missing_docs)]
///Container type for the return parameters of the [`getRoundData(uint80)`](getRoundDataCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct getRoundDataReturn {
    pub roundId: ::alloy_sol_types::private::primitives::aliases::U80,
    pub answer: ::alloy_sol_types::private::primitives::aliases::I256,
    pub startedAt: ::alloy_sol_types::private::primitives::aliases::U256,
    pub updatedAt: ::alloy_sol_types::private::primitives::aliases::U256,
    pub answeredInRound: ::alloy_sol_types::private::primitives::aliases::U80,
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for getRoundDataReturn {
    #[inline]
    fn clone(&self) -> getRoundDataReturn {
        getRoundDataReturn {
            roundId: ::core::clone::Clone::clone(&self.roundId),
            answer: ::core::clone::Clone::clone(&self.answer),
            startedAt: ::core::clone::Clone::clone(&self.startedAt),
            updatedAt: ::core::clone::Clone::clone(&self.updatedAt),
            answeredInRound: ::core::clone::Clone::clone(&self.answeredInRound),
        }
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for getRoundDataReturn {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "getRoundDataReturn",
            "roundId",
            &self.roundId,
            "answer",
            &self.answer,
            "startedAt",
            &self.startedAt,
            "updatedAt",
            &self.updatedAt,
            "answeredInRound",
            &&self.answeredInRound,
        )
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for getRoundDataReturn {}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for getRoundDataReturn {
    #[inline]
    fn eq(&self, other: &getRoundDataReturn) -> bool {
        self.roundId == other.roundId && self.answer == other.answer
            && self.startedAt == other.startedAt && self.updatedAt == other.updatedAt
            && self.answeredInRound == other.answeredInRound
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for getRoundDataReturn {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::primitives::aliases::U80,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::primitives::aliases::I256,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::primitives::aliases::U256,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::primitives::aliases::U256,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::primitives::aliases::U80,
        >;
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
        type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<80>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            ::alloy_sol_types::private::primitives::aliases::U80,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<getRoundDataCall> for UnderlyingRustTuple<'_> {
            fn from(value: getRoundDataCall) -> Self {
                (value._roundId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoundDataCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _roundId: tuple.0 }
            }
        }
    }
    {
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            ::alloy_sol_types::sol_data::Uint<80>,
            ::alloy_sol_types::sol_data::Int<256>,
            ::alloy_sol_types::sol_data::Uint<256>,
            ::alloy_sol_types::sol_data::Uint<256>,
            ::alloy_sol_types::sol_data::Uint<80>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            ::alloy_sol_types::private::primitives::aliases::U80,
            ::alloy_sol_types::private::primitives::aliases::I256,
            ::alloy_sol_types::private::primitives::aliases::U256,
            ::alloy_sol_types::private::primitives::aliases::U256,
            ::alloy_sol_types::private::primitives::aliases::U80,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<getRoundDataReturn> for UnderlyingRustTuple<'_> {
            fn from(value: getRoundDataReturn) -> Self {
                (
                    value.roundId,
                    value.answer,
                    value.startedAt,
                    value.updatedAt,
                    value.answeredInRound,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoundDataReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    roundId: tuple.0,
                    answer: tuple.1,
                    startedAt: tuple.2,
                    updatedAt: tuple.3,
                    answeredInRound: tuple.4,
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for getRoundDataCall {
        type Parameters<'a> = (::alloy_sol_types::sol_data::Uint<80>,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = getRoundDataReturn;
        type ReturnTuple<'a> = (
            ::alloy_sol_types::sol_data::Uint<80>,
            ::alloy_sol_types::sol_data::Int<256>,
            ::alloy_sol_types::sol_data::Uint<256>,
            ::alloy_sol_types::sol_data::Uint<256>,
            ::alloy_sol_types::sol_data::Uint<80>,
        );
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "getRoundData(uint80)";
        const SELECTOR: [u8; 4] = [154u8, 111u8, 200u8, 245u8];
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
                    80,
                > as alloy_sol_types::SolType>::tokenize(&self._roundId),
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
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct balanceOfRet(::alloy_sol_types::private::primitives::aliases::U256);
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for balanceOfRet {
    #[inline]
    fn clone(&self) -> balanceOfRet {
        balanceOfRet(::core::clone::Clone::clone(&self.0))
    }
}
const _: () = {
    use ::alloy_sol_types as alloy_sol_types;
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<balanceOfRet>
    for ::alloy_sol_types::private::primitives::aliases::U256 {
        #[inline]
        fn stv_to_tokens(
            &self,
        ) -> <::alloy_sol_types::sol_data::Uint<
            256,
        > as alloy_sol_types::SolType>::Token<'_> {
            alloy_sol_types::private::SolTypeValue::<
                ::alloy_sol_types::sol_data::Uint<256>,
            >::stv_to_tokens(self)
        }
        #[inline]
        fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::tokenize(self)
                .0
        }
        #[inline]
        fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
        }
        #[inline]
        fn stv_abi_packed_encoded_size(&self) -> usize {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::abi_encoded_size(self)
        }
    }
    #[automatically_derived]
    impl balanceOfRet {
        /// The Solidity type name.
        pub const NAME: &'static str = "@ name";
        /// Convert from the underlying value type.
        #[inline]
        pub const fn from(
            value: ::alloy_sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self(value)
        }
        /// Return the underlying value.
        #[inline]
        pub const fn into(
            self,
        ) -> ::alloy_sol_types::private::primitives::aliases::U256 {
            self.0
        }
        /// Return the single encoding of this value, delegating to the
        /// underlying type.
        #[inline]
        pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
            <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
        }
        /// Return the packed encoding of this value, delegating to the
        /// underlying type.
        #[inline]
        pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
            <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolType for balanceOfRet {
        type RustType = ::alloy_sol_types::private::primitives::aliases::U256;
        type Token<'a> = <::alloy_sol_types::sol_data::Uint<
            256,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SOL_NAME: &'static str = Self::NAME;
        const ENCODED_SIZE: Option<usize> = <::alloy_sol_types::sol_data::Uint<
            256,
        > as alloy_sol_types::SolType>::ENCODED_SIZE;
        const PACKED_ENCODED_SIZE: Option<usize> = <::alloy_sol_types::sol_data::Uint<
            256,
        > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
        #[inline]
        fn valid_token(token: &Self::Token<'_>) -> bool {
            Self::type_check(token).is_ok()
        }
        #[inline]
        fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::type_check(token)
        }
        #[inline]
        fn detokenize(token: Self::Token<'_>) -> Self::RustType {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::detokenize(token)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for balanceOfRet {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
        }
        #[inline]
        fn encode_topic_preimage(
            rust: &Self::RustType,
            out: &mut alloy_sol_types::private::Vec<u8>,
        ) {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
        }
        #[inline]
        fn encode_topic(
            rust: &Self::RustType,
        ) -> alloy_sol_types::abi::token::WordToken {
            <::alloy_sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::EventTopic>::encode_topic(rust)
        }
    }
};
/**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address account) external view returns (balanceOfRet);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct balanceOfCall {
    pub account: ::alloy_sol_types::private::Address,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for balanceOfCall {
    #[inline]
    fn clone(&self) -> balanceOfCall {
        balanceOfCall {
            account: ::core::clone::Clone::clone(&self.account),
        }
    }
}
///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct balanceOfReturn {
    pub _0: <balanceOfRet as ::alloy_sol_types::SolType>::RustType,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for balanceOfReturn {
    #[inline]
    fn clone(&self) -> balanceOfReturn {
        balanceOfReturn {
            _0: ::core::clone::Clone::clone(&self._0),
        }
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
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
    }
    {
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (balanceOfRet,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <balanceOfRet as ::alloy_sol_types::SolType>::RustType,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
            fn from(value: balanceOfReturn) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for balanceOfCall {
        type Parameters<'a> = (::alloy_sol_types::sol_data::Address,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = balanceOfReturn;
        type ReturnTuple<'a> = (balanceOfRet,);
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
/**Function with signature `builtinCancellation(bytes32)` and selector `0x097ff27a`.
```solidity
function builtinCancellation(bytes32 hash);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct builtinCancellationCall {
    pub hash: ::alloy_sol_types::private::FixedBytes<32>,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for builtinCancellationCall {
    #[inline]
    fn clone(&self) -> builtinCancellationCall {
        builtinCancellationCall {
            hash: ::core::clone::Clone::clone(&self.hash),
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for builtinCancellationCall {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "builtinCancellationCall",
            "hash",
            &&self.hash,
        )
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for builtinCancellationCall {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for builtinCancellationCall {
    #[inline]
    fn eq(&self, other: &builtinCancellationCall) -> bool {
        self.hash == other.hash
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for builtinCancellationCall {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::FixedBytes<32>>;
    }
}
///Container type for the return parameters of the [`builtinCancellation(bytes32)`](builtinCancellationCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct builtinCancellationReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for builtinCancellationReturn {
    #[inline]
    fn clone(&self) -> builtinCancellationReturn {
        builtinCancellationReturn {}
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for builtinCancellationReturn {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "builtinCancellationReturn")
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for builtinCancellationReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for builtinCancellationReturn {
    #[inline]
    fn eq(&self, other: &builtinCancellationReturn) -> bool {
        true
    }
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for builtinCancellationReturn {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
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
        type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::FixedBytes<32>,);
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<builtinCancellationCall> for UnderlyingRustTuple<'_> {
            fn from(value: builtinCancellationCall) -> Self {
                (value.hash,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for builtinCancellationCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { hash: tuple.0 }
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
        impl ::core::convert::From<builtinCancellationReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: builtinCancellationReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for builtinCancellationReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for builtinCancellationCall {
        type Parameters<'a> = (::alloy_sol_types::sol_data::FixedBytes<32>,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = builtinCancellationReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "builtinCancellation(bytes32)";
        const SELECTOR: [u8; 4] = [9u8, 127u8, 242u8, 122u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (
                <::alloy_sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(&self.hash),
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
#[allow(missing_docs)]
/**```solidity
struct HalfTx { bytes32 hash; bytes[] signatures; }
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct HalfTx {
    pub hash: ::alloy_sol_types::private::FixedBytes<32>,
    pub signatures: ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Bytes>,
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for HalfTx {
    #[inline]
    fn clone(&self) -> HalfTx {
        HalfTx {
            hash: ::core::clone::Clone::clone(&self.hash),
            signatures: ::core::clone::Clone::clone(&self.signatures),
        }
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for HalfTx {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "HalfTx",
            "hash",
            &self.hash,
            "signatures",
            &&self.signatures,
        )
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for HalfTx {}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for HalfTx {
    #[inline]
    fn eq(&self, other: &HalfTx) -> bool {
        self.hash == other.hash && self.signatures == other.signatures
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for HalfTx {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::FixedBytes<32>>;
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Bytes>,
        >;
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
        ::alloy_sol_types::sol_data::FixedBytes<32>,
        ::alloy_sol_types::sol_data::Array<::alloy_sol_types::sol_data::Bytes>,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = (
        ::alloy_sol_types::private::FixedBytes<32>,
        ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Bytes>,
    );
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<HalfTx> for UnderlyingRustTuple<'_> {
        fn from(value: HalfTx) -> Self {
            (value.hash, value.signatures)
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<UnderlyingRustTuple<'_>> for HalfTx {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            Self {
                hash: tuple.0,
                signatures: tuple.1,
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolValue for HalfTx {
        type SolType = Self;
    }
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<Self> for HalfTx {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            (
                <::alloy_sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(&self.hash),
                <::alloy_sol_types::sol_data::Array<
                    ::alloy_sol_types::sol_data::Bytes,
                > as alloy_sol_types::SolType>::tokenize(&self.signatures),
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
    impl alloy_sol_types::SolType for HalfTx {
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
    impl alloy_sol_types::SolStruct for HalfTx {
        const NAME: &'static str = "HalfTx";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed(
                "HalfTx(bytes32 hash,bytes[] signatures)",
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
                <::alloy_sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.hash)
                    .0,
                <::alloy_sol_types::sol_data::Array<
                    ::alloy_sol_types::sol_data::Bytes,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.signatures)
                    .0,
            ]
                .concat()
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for HalfTx {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            0usize
                + <::alloy_sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.hash)
                + <::alloy_sol_types::sol_data::Array<
                    ::alloy_sol_types::sol_data::Bytes,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.signatures,
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
            <::alloy_sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.hash, out);
            <::alloy_sol_types::sol_data::Array<
                ::alloy_sol_types::sol_data::Bytes,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.signatures,
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
#[allow(missing_docs)]
/**Function with signature `builtinAccountRecovery((bytes32,bytes[])[])` and selector `0xc2c44d97`.
```solidity
function builtinAccountRecovery(HalfTx[] halfTxs);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct builtinAccountRecoveryCall {
    pub halfTxs: ::alloy_sol_types::private::Vec<
        <HalfTx as ::alloy_sol_types::SolType>::RustType,
    >,
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for builtinAccountRecoveryCall {
    #[inline]
    fn clone(&self) -> builtinAccountRecoveryCall {
        builtinAccountRecoveryCall {
            halfTxs: ::core::clone::Clone::clone(&self.halfTxs),
        }
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for builtinAccountRecoveryCall {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "builtinAccountRecoveryCall",
            "halfTxs",
            &&self.halfTxs,
        )
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for builtinAccountRecoveryCall {}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for builtinAccountRecoveryCall {
    #[inline]
    fn eq(&self, other: &builtinAccountRecoveryCall) -> bool {
        self.halfTxs == other.halfTxs
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for builtinAccountRecoveryCall {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            ::alloy_sol_types::private::Vec<
                <HalfTx as ::alloy_sol_types::SolType>::RustType,
            >,
        >;
    }
}
#[allow(missing_docs)]
///Container type for the return parameters of the [`builtinAccountRecovery((bytes32,bytes[])[])`](builtinAccountRecoveryCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct builtinAccountRecoveryReturn {}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for builtinAccountRecoveryReturn {
    #[inline]
    fn clone(&self) -> builtinAccountRecoveryReturn {
        builtinAccountRecoveryReturn {}
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::fmt::Debug for builtinAccountRecoveryReturn {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "builtinAccountRecoveryReturn")
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::marker::StructuralPartialEq for builtinAccountRecoveryReturn {}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::PartialEq for builtinAccountRecoveryReturn {
    #[inline]
    fn eq(&self, other: &builtinAccountRecoveryReturn) -> bool {
        true
    }
}
#[automatically_derived]
#[allow(missing_docs)]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::cmp::Eq for builtinAccountRecoveryReturn {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
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
        type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Array<HalfTx>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            ::alloy_sol_types::private::Vec<
                <HalfTx as ::alloy_sol_types::SolType>::RustType,
            >,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<builtinAccountRecoveryCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: builtinAccountRecoveryCall) -> Self {
                (value.halfTxs,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for builtinAccountRecoveryCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { halfTxs: tuple.0 }
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
        impl ::core::convert::From<builtinAccountRecoveryReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: builtinAccountRecoveryReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for builtinAccountRecoveryReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for builtinAccountRecoveryCall {
        type Parameters<'a> = (::alloy_sol_types::sol_data::Array<HalfTx>,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = builtinAccountRecoveryReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "builtinAccountRecovery((bytes32,bytes[])[])";
        const SELECTOR: [u8; 4] = [194u8, 196u8, 77u8, 151u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (
                <::alloy_sol_types::sol_data::Array<
                    HalfTx,
                > as alloy_sol_types::SolType>::tokenize(&self.halfTxs),
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
    #[automatically_derived]
    impl alloy_sol_types::JsonAbiExt for builtinAccountRecoveryCall {
        type Abi = alloy_sol_types::private::alloy_json_abi::Function;
        #[inline]
        fn abi() -> Self::Abi {
            ::alloy_sol_types::private::alloy_json_abi::Function {
                name: ::alloy_sol_types::private::ToOwned::to_owned(
                    "builtinAccountRecovery",
                ),
                inputs: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ::alloy_sol_types::private::alloy_json_abi::Param {
                            ty: ::alloy_sol_types::private::ToOwned::to_owned("tuple[]"),
                            name: ::alloy_sol_types::private::ToOwned::to_owned(
                                "halfTxs",
                            ),
                            components: <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    ::alloy_sol_types::private::alloy_json_abi::Param {
                                        ty: ::alloy_sol_types::private::ToOwned::to_owned(
                                            "bytes32",
                                        ),
                                        name: ::alloy_sol_types::private::ToOwned::to_owned("hash"),
                                        components: ::alloy_sol_types::private::Vec::new(),
                                        internal_type: ::core::option::Option::None,
                                    },
                                    ::alloy_sol_types::private::alloy_json_abi::Param {
                                        ty: ::alloy_sol_types::private::ToOwned::to_owned(
                                            "bytes[]",
                                        ),
                                        name: ::alloy_sol_types::private::ToOwned::to_owned(
                                            "signatures",
                                        ),
                                        components: ::alloy_sol_types::private::Vec::new(),
                                        internal_type: ::core::option::Option::None,
                                    },
                                ]),
                            ),
                            internal_type: ::core::option::Option::None,
                        },
                    ]),
                ),
                outputs: ::alloy_sol_types::private::Vec::new(),
                state_mutability: ::alloy_sol_types::private::alloy_json_abi::StateMutability::NonPayable,
            }
        }
    }
};
/**```solidity
struct Governance { uint64 epoch; bytes operator; address mint; Parameters parameters; string version; Validator[] validators; string banned; }
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct Governance {
    pub epoch: u64,
    pub operator: ::alloy_sol_types::private::Bytes,
    pub mint: ::alloy_sol_types::private::Address,
    pub parameters: <Parameters as ::alloy_sol_types::SolType>::RustType,
    pub version: ::alloy_sol_types::private::String,
    pub validators: ::alloy_sol_types::private::Vec<
        <Validator as ::alloy_sol_types::SolType>::RustType,
    >,
    pub banned: ::alloy_sol_types::private::String,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for Governance {
    #[inline]
    fn clone(&self) -> Governance {
        Governance {
            epoch: ::core::clone::Clone::clone(&self.epoch),
            operator: ::core::clone::Clone::clone(&self.operator),
            mint: ::core::clone::Clone::clone(&self.mint),
            parameters: ::core::clone::Clone::clone(&self.parameters),
            version: ::core::clone::Clone::clone(&self.version),
            validators: ::core::clone::Clone::clone(&self.validators),
            banned: ::core::clone::Clone::clone(&self.banned),
        }
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
        ::alloy_sol_types::sol_data::Uint<64>,
        ::alloy_sol_types::sol_data::Bytes,
        ::alloy_sol_types::sol_data::Address,
        Parameters,
        ::alloy_sol_types::sol_data::String,
        ::alloy_sol_types::sol_data::Array<Validator>,
        ::alloy_sol_types::sol_data::String,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = (
        u64,
        ::alloy_sol_types::private::Bytes,
        ::alloy_sol_types::private::Address,
        <Parameters as ::alloy_sol_types::SolType>::RustType,
        ::alloy_sol_types::private::String,
        ::alloy_sol_types::private::Vec<
            <Validator as ::alloy_sol_types::SolType>::RustType,
        >,
        ::alloy_sol_types::private::String,
    );
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<Governance> for UnderlyingRustTuple<'_> {
        fn from(value: Governance) -> Self {
            (
                value.epoch,
                value.operator,
                value.mint,
                value.parameters,
                value.version,
                value.validators,
                value.banned,
            )
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<UnderlyingRustTuple<'_>> for Governance {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            Self {
                epoch: tuple.0,
                operator: tuple.1,
                mint: tuple.2,
                parameters: tuple.3,
                version: tuple.4,
                validators: tuple.5,
                banned: tuple.6,
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolValue for Governance {
        type SolType = Self;
    }
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<Self> for Governance {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            (
                <::alloy_sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                    &self.operator,
                ),
                <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.mint,
                ),
                <Parameters as alloy_sol_types::SolType>::tokenize(&self.parameters),
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.version,
                ),
                <::alloy_sol_types::sol_data::Array<
                    Validator,
                > as alloy_sol_types::SolType>::tokenize(&self.validators),
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.banned,
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
    impl alloy_sol_types::SolType for Governance {
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
    impl alloy_sol_types::SolStruct for Governance {
        const NAME: &'static str = "Governance";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed(
                "Governance(uint64 epoch,bytes operator,address mint,Parameters parameters,string version,Validator[] validators,string banned)",
            )
        }
        #[inline]
        fn eip712_components() -> alloy_sol_types::private::Vec<
            alloy_sol_types::private::Cow<'static, str>,
        > {
            let mut components = alloy_sol_types::private::Vec::with_capacity(2);
            components
                .push(<Parameters as alloy_sol_types::SolStruct>::eip712_root_type());
            components
                .extend(<Parameters as alloy_sol_types::SolStruct>::eip712_components());
            components
                .push(<Validator as alloy_sol_types::SolStruct>::eip712_root_type());
            components
                .extend(<Validator as alloy_sol_types::SolStruct>::eip712_components());
            components
        }
        #[inline]
        fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
            [
                <::alloy_sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.epoch)
                    .0,
                <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                        &self.operator,
                    )
                    .0,
                <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                        &self.mint,
                    )
                    .0,
                <Parameters as alloy_sol_types::SolType>::eip712_data_word(
                        &self.parameters,
                    )
                    .0,
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                        &self.version,
                    )
                    .0,
                <::alloy_sol_types::sol_data::Array<
                    Validator,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.validators)
                    .0,
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                        &self.banned,
                    )
                    .0,
            ]
                .concat()
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for Governance {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            0usize
                + <::alloy_sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.epoch)
                + <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.operator,
                )
                + <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.mint,
                )
                + <Parameters as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.parameters,
                )
                + <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.version,
                )
                + <::alloy_sol_types::sol_data::Array<
                    Validator,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.validators,
                )
                + <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.banned,
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
            <::alloy_sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.epoch, out);
            <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.operator,
                out,
            );
            <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.mint,
                out,
            );
            <Parameters as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.parameters,
                out,
            );
            <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.version,
                out,
            );
            <::alloy_sol_types::sol_data::Array<
                Validator,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.validators,
                out,
            );
            <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.banned,
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
struct Parameters { uint128 txFee; uint128 mintFee; uint128 burnFee; uint32 minTxToCheckpoint; uint32 operatorFeeShare; }
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct Parameters {
    pub txFee: u128,
    pub mintFee: u128,
    pub burnFee: u128,
    pub minTxToCheckpoint: u32,
    pub operatorFeeShare: u32,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for Parameters {
    #[inline]
    fn clone(&self) -> Parameters {
        Parameters {
            txFee: ::core::clone::Clone::clone(&self.txFee),
            mintFee: ::core::clone::Clone::clone(&self.mintFee),
            burnFee: ::core::clone::Clone::clone(&self.burnFee),
            minTxToCheckpoint: ::core::clone::Clone::clone(&self.minTxToCheckpoint),
            operatorFeeShare: ::core::clone::Clone::clone(&self.operatorFeeShare),
        }
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
        ::alloy_sol_types::sol_data::Uint<128>,
        ::alloy_sol_types::sol_data::Uint<128>,
        ::alloy_sol_types::sol_data::Uint<128>,
        ::alloy_sol_types::sol_data::Uint<32>,
        ::alloy_sol_types::sol_data::Uint<32>,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = (u128, u128, u128, u32, u32);
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<Parameters> for UnderlyingRustTuple<'_> {
        fn from(value: Parameters) -> Self {
            (
                value.txFee,
                value.mintFee,
                value.burnFee,
                value.minTxToCheckpoint,
                value.operatorFeeShare,
            )
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<UnderlyingRustTuple<'_>> for Parameters {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            Self {
                txFee: tuple.0,
                mintFee: tuple.1,
                burnFee: tuple.2,
                minTxToCheckpoint: tuple.3,
                operatorFeeShare: tuple.4,
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolValue for Parameters {
        type SolType = Self;
    }
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<Self> for Parameters {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            (
                <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(&self.txFee),
                <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(&self.mintFee),
                <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(&self.burnFee),
                <::alloy_sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::tokenize(&self.minTxToCheckpoint),
                <::alloy_sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::tokenize(&self.operatorFeeShare),
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
    impl alloy_sol_types::SolType for Parameters {
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
    impl alloy_sol_types::SolStruct for Parameters {
        const NAME: &'static str = "Parameters";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed(
                "Parameters(uint128 txFee,uint128 mintFee,uint128 burnFee,uint32 minTxToCheckpoint,uint32 operatorFeeShare)",
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
                <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.txFee)
                    .0,
                <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.mintFee)
                    .0,
                <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.burnFee)
                    .0,
                <::alloy_sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.minTxToCheckpoint)
                    .0,
                <::alloy_sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorFeeShare)
                    .0,
            ]
                .concat()
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for Parameters {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            0usize
                + <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.txFee)
                + <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.mintFee)
                + <::alloy_sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.burnFee)
                + <::alloy_sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.minTxToCheckpoint,
                )
                + <::alloy_sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.operatorFeeShare,
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
            <::alloy_sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.txFee, out);
            <::alloy_sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.mintFee, out);
            <::alloy_sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.burnFee, out);
            <::alloy_sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.minTxToCheckpoint,
                out,
            );
            <::alloy_sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.operatorFeeShare,
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
struct Validator { bytes publicKey; address account; string ip; bool archive; }
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct Validator {
    pub publicKey: ::alloy_sol_types::private::Bytes,
    pub account: ::alloy_sol_types::private::Address,
    pub ip: ::alloy_sol_types::private::String,
    pub archive: bool,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for Validator {
    #[inline]
    fn clone(&self) -> Validator {
        Validator {
            publicKey: ::core::clone::Clone::clone(&self.publicKey),
            account: ::core::clone::Clone::clone(&self.account),
            ip: ::core::clone::Clone::clone(&self.ip),
            archive: ::core::clone::Clone::clone(&self.archive),
        }
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
        ::alloy_sol_types::sol_data::Bytes,
        ::alloy_sol_types::sol_data::Address,
        ::alloy_sol_types::sol_data::String,
        ::alloy_sol_types::sol_data::Bool,
    );
    #[doc(hidden)]
    type UnderlyingRustTuple<'a> = (
        ::alloy_sol_types::private::Bytes,
        ::alloy_sol_types::private::Address,
        ::alloy_sol_types::private::String,
        bool,
    );
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<Validator> for UnderlyingRustTuple<'_> {
        fn from(value: Validator) -> Self {
            (value.publicKey, value.account, value.ip, value.archive)
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl ::core::convert::From<UnderlyingRustTuple<'_>> for Validator {
        fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
            Self {
                publicKey: tuple.0,
                account: tuple.1,
                ip: tuple.2,
                archive: tuple.3,
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolValue for Validator {
        type SolType = Self;
    }
    #[automatically_derived]
    impl alloy_sol_types::private::SolTypeValue<Self> for Validator {
        #[inline]
        fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
            (
                <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                    &self.publicKey,
                ),
                <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.account,
                ),
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.ip,
                ),
                <::alloy_sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                    &self.archive,
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
    impl alloy_sol_types::SolType for Validator {
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
    impl alloy_sol_types::SolStruct for Validator {
        const NAME: &'static str = "Validator";
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed(
                "Validator(bytes publicKey,address account,string ip,bool archive)",
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
                <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                        &self.publicKey,
                    )
                    .0,
                <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                        &self.account,
                    )
                    .0,
                <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                        &self.ip,
                    )
                    .0,
                <::alloy_sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                        &self.archive,
                    )
                    .0,
            ]
                .concat()
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::EventTopic for Validator {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            0usize
                + <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.publicKey,
                )
                + <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.account,
                )
                + <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.ip,
                )
                + <::alloy_sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                    &rust.archive,
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
            <::alloy_sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.publicKey,
                out,
            );
            <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.account,
                out,
            );
            <::alloy_sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.ip,
                out,
            );
            <::alloy_sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                &rust.archive,
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
/**Function with signature `systemGovernance((uint64,bytes,address,(uint128,uint128,uint128,uint32,uint32),string,(bytes,address,string,bool)[],string))` and selector `0xed8fad26`.
```solidity
function systemGovernance(Governance governance);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemGovernanceCall {
    pub governance: <Governance as ::alloy_sol_types::SolType>::RustType,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemGovernanceCall {
    #[inline]
    fn clone(&self) -> systemGovernanceCall {
        systemGovernanceCall {
            governance: ::core::clone::Clone::clone(&self.governance),
        }
    }
}
///Container type for the return parameters of the [`systemGovernance((uint64,bytes,address,(uint128,uint128,uint128,uint32,uint32),string,(bytes,address,string,bool)[],string))`](systemGovernanceCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemGovernanceReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemGovernanceReturn {
    #[inline]
    fn clone(&self) -> systemGovernanceReturn {
        systemGovernanceReturn {}
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
        type UnderlyingSolTuple<'a> = (Governance,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Governance as ::alloy_sol_types::SolType>::RustType,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<systemGovernanceCall> for UnderlyingRustTuple<'_> {
            fn from(value: systemGovernanceCall) -> Self {
                (value.governance,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for systemGovernanceCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { governance: tuple.0 }
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
        impl ::core::convert::From<systemGovernanceReturn> for UnderlyingRustTuple<'_> {
            fn from(value: systemGovernanceReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for systemGovernanceReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for systemGovernanceCall {
        type Parameters<'a> = (Governance,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = systemGovernanceReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "systemGovernance((uint64,bytes,address,(uint128,uint128,uint128,uint32,uint32),string,(bytes,address,string,bool)[],string))";
        const SELECTOR: [u8; 4] = [237u8, 143u8, 173u8, 38u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (<Governance as alloy_sol_types::SolType>::tokenize(&self.governance),)
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
/**Function with signature `systemSetParameters((uint128,uint128,uint128,uint32,uint32))` and selector `0x54c33470`.
```solidity
function systemSetParameters(Parameters parameters);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemSetParametersCall {
    pub parameters: <Parameters as ::alloy_sol_types::SolType>::RustType,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemSetParametersCall {
    #[inline]
    fn clone(&self) -> systemSetParametersCall {
        systemSetParametersCall {
            parameters: ::core::clone::Clone::clone(&self.parameters),
        }
    }
}
///Container type for the return parameters of the [`systemSetParameters((uint128,uint128,uint128,uint32,uint32))`](systemSetParametersCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemSetParametersReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemSetParametersReturn {
    #[inline]
    fn clone(&self) -> systemSetParametersReturn {
        systemSetParametersReturn {}
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
        type UnderlyingSolTuple<'a> = (Parameters,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Parameters as ::alloy_sol_types::SolType>::RustType,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<systemSetParametersCall> for UnderlyingRustTuple<'_> {
            fn from(value: systemSetParametersCall) -> Self {
                (value.parameters,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for systemSetParametersCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { parameters: tuple.0 }
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
        impl ::core::convert::From<systemSetParametersReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemSetParametersReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemSetParametersReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for systemSetParametersCall {
        type Parameters<'a> = (Parameters,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = systemSetParametersReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "systemSetParameters((uint128,uint128,uint128,uint32,uint32))";
        const SELECTOR: [u8; 4] = [84u8, 195u8, 52u8, 112u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (<Parameters as alloy_sol_types::SolType>::tokenize(&self.parameters),)
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
/**Function with signature `systemAppendValidator((bytes,address,string,bool))` and selector `0x81910e3c`.
```solidity
function systemAppendValidator(Validator validator);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemAppendValidatorCall {
    pub validator: <Validator as ::alloy_sol_types::SolType>::RustType,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemAppendValidatorCall {
    #[inline]
    fn clone(&self) -> systemAppendValidatorCall {
        systemAppendValidatorCall {
            validator: ::core::clone::Clone::clone(&self.validator),
        }
    }
}
///Container type for the return parameters of the [`systemAppendValidator((bytes,address,string,bool))`](systemAppendValidatorCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemAppendValidatorReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemAppendValidatorReturn {
    #[inline]
    fn clone(&self) -> systemAppendValidatorReturn {
        systemAppendValidatorReturn {}
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
        type UnderlyingSolTuple<'a> = (Validator,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Validator as ::alloy_sol_types::SolType>::RustType,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<systemAppendValidatorCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemAppendValidatorCall) -> Self {
                (value.validator,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemAppendValidatorCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { validator: tuple.0 }
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
        impl ::core::convert::From<systemAppendValidatorReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemAppendValidatorReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemAppendValidatorReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for systemAppendValidatorCall {
        type Parameters<'a> = (Validator,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = systemAppendValidatorReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "systemAppendValidator((bytes,address,string,bool))";
        const SELECTOR: [u8; 4] = [129u8, 145u8, 14u8, 60u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (<Validator as alloy_sol_types::SolType>::tokenize(&self.validator),)
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
/**Function with signature `systemReplaceAllValidators((bytes,address,string,bool)[])` and selector `0x9f8bb0ee`.
```solidity
function systemReplaceAllValidators(Validator[] validators);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemReplaceAllValidatorsCall {
    pub validators: ::alloy_sol_types::private::Vec<
        <Validator as ::alloy_sol_types::SolType>::RustType,
    >,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemReplaceAllValidatorsCall {
    #[inline]
    fn clone(&self) -> systemReplaceAllValidatorsCall {
        systemReplaceAllValidatorsCall {
            validators: ::core::clone::Clone::clone(&self.validators),
        }
    }
}
///Container type for the return parameters of the [`systemReplaceAllValidators((bytes,address,string,bool)[])`](systemReplaceAllValidatorsCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemReplaceAllValidatorsReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemReplaceAllValidatorsReturn {
    #[inline]
    fn clone(&self) -> systemReplaceAllValidatorsReturn {
        systemReplaceAllValidatorsReturn {
        }
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
        type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Array<Validator>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            ::alloy_sol_types::private::Vec<
                <Validator as ::alloy_sol_types::SolType>::RustType,
            >,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<systemReplaceAllValidatorsCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemReplaceAllValidatorsCall) -> Self {
                (value.validators,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemReplaceAllValidatorsCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { validators: tuple.0 }
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
        impl ::core::convert::From<systemReplaceAllValidatorsReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemReplaceAllValidatorsReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemReplaceAllValidatorsReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for systemReplaceAllValidatorsCall {
        type Parameters<'a> = (::alloy_sol_types::sol_data::Array<Validator>,);
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = systemReplaceAllValidatorsReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "systemReplaceAllValidators((bytes,address,string,bool)[])";
        const SELECTOR: [u8; 4] = [159u8, 139u8, 176u8, 238u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (
                <::alloy_sol_types::sol_data::Array<
                    Validator,
                > as alloy_sol_types::SolType>::tokenize(&self.validators),
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
/**Function with signature `systemUpdateBanned(string,bool)` and selector `0xee313577`.
```solidity
function systemUpdateBanned(string newBanned, bool clearExisting);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemUpdateBannedCall {
    pub newBanned: ::alloy_sol_types::private::String,
    pub clearExisting: bool,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemUpdateBannedCall {
    #[inline]
    fn clone(&self) -> systemUpdateBannedCall {
        systemUpdateBannedCall {
            newBanned: ::core::clone::Clone::clone(&self.newBanned),
            clearExisting: ::core::clone::Clone::clone(&self.clearExisting),
        }
    }
}
///Container type for the return parameters of the [`systemUpdateBanned(string,bool)`](systemUpdateBannedCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemUpdateBannedReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemUpdateBannedReturn {
    #[inline]
    fn clone(&self) -> systemUpdateBannedReturn {
        systemUpdateBannedReturn {}
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
            ::alloy_sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::String, bool);
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<systemUpdateBannedCall> for UnderlyingRustTuple<'_> {
            fn from(value: systemUpdateBannedCall) -> Self {
                (value.newBanned, value.clearExisting)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for systemUpdateBannedCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    newBanned: tuple.0,
                    clearExisting: tuple.1,
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
        impl ::core::convert::From<systemUpdateBannedReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemUpdateBannedReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemUpdateBannedReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for systemUpdateBannedCall {
        type Parameters<'a> = (
            ::alloy_sol_types::sol_data::String,
            ::alloy_sol_types::sol_data::Bool,
        );
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = systemUpdateBannedReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "systemUpdateBanned(string,bool)";
        const SELECTOR: [u8; 4] = [238u8, 49u8, 53u8, 119u8];
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
                    &self.newBanned,
                ),
                <::alloy_sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                    &self.clearExisting,
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
/**Function with signature `systemAddBannedAccounts(address[])` and selector `0x4e431d92`.
```solidity
function systemAddBannedAccounts(address[] accounts);
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemAddBannedAccountsCall {
    pub accounts: ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Address>,
}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemAddBannedAccountsCall {
    #[inline]
    fn clone(&self) -> systemAddBannedAccountsCall {
        systemAddBannedAccountsCall {
            accounts: ::core::clone::Clone::clone(&self.accounts),
        }
    }
}
///Container type for the return parameters of the [`systemAddBannedAccounts(address[])`](systemAddBannedAccountsCall) function.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
pub struct systemAddBannedAccountsReturn {}
#[automatically_derived]
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
impl ::core::clone::Clone for systemAddBannedAccountsReturn {
    #[inline]
    fn clone(&self) -> systemAddBannedAccountsReturn {
        systemAddBannedAccountsReturn {}
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
            ::alloy_sol_types::sol_data::Array<::alloy_sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Address>,
        );
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<systemAddBannedAccountsCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemAddBannedAccountsCall) -> Self {
                (value.accounts,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemAddBannedAccountsCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { accounts: tuple.0 }
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
        impl ::core::convert::From<systemAddBannedAccountsReturn>
        for UnderlyingRustTuple<'_> {
            fn from(value: systemAddBannedAccountsReturn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for systemAddBannedAccountsReturn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolCall for systemAddBannedAccountsCall {
        type Parameters<'a> = (
            ::alloy_sol_types::sol_data::Array<::alloy_sol_types::sol_data::Address>,
        );
        type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
        type Return = systemAddBannedAccountsReturn;
        type ReturnTuple<'a> = ();
        type ReturnToken<'a> = <Self::ReturnTuple<
            'a,
        > as alloy_sol_types::SolType>::Token<'a>;
        const SIGNATURE: &'static str = "systemAddBannedAccounts(address[])";
        const SELECTOR: [u8; 4] = [78u8, 67u8, 29u8, 146u8];
        #[inline]
        fn new<'a>(
            tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
        ) -> Self {
            tuple.into()
        }
        #[inline]
        fn tokenize(&self) -> Self::Token<'_> {
            (
                <::alloy_sol_types::sol_data::Array<
                    ::alloy_sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(&self.accounts),
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
