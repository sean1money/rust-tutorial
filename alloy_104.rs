#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use alloy::{
    network::EthereumWallet, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, sol,
};
use anyhow::Result;
fn main() -> Result<()> {
    let body = async {
        let bytes = hex::decode(
                "8178ebd5ed69413f50fda1710254350be874b3020319ce7b9b8e38810f358b1b",
            )
            .unwrap();
        let signer = PrivateKeySigner::from_slice(bytes.as_slice()).unwrap();
        let sender = signer.address();
        {
            ::std::io::_eprint(format_args!("Sender address: {0}\n", sender));
        };
        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http("http://127.0.0.1:18545".parse().unwrap());
        let _counter = Counter::deploy(&provider).await.unwrap();
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
///Module containing a contract's types and functions.
#[allow(missing_docs)]
/**

```solidity
contract Counter {
    uint256 public number;
    function setNumber(uint256 newNumber) public { <stmts> }
    function increment() public { <stmts> }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Counter {
    use super::*;
    use ::alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`\x13W`\xdf\x90\x81`\x19\x829\xf3[`\0\x80\xfd\xfe`\x80\x80`@R`\x046\x10\x15`\x12W`\0\x80\xfd[`\05`\xe0\x1c\x90\x81c?\xb5\xc1\xcb\x14`\x92W\x81c\x83\x81\xf5\x8a\x14`yWPc\xd0\x9d\xe0\x8a\x14`<W`\0\x80\xfd[4`tW`\06`\x03\x19\x01\x12`tW`\0T`\0\x19\x81\x14`^W`\x01\x01`\0U\0[cNH{q`\xe0\x1b`\0R`\x11`\x04R`$`\0\xfd[`\0\x80\xfd[4`tW`\06`\x03\x19\x01\x12`tW` \x90`\0T\x81R\xf3[4`tW` 6`\x03\x19\x01\x12`tW`\x045`\0U\0\xfe\xa2dipfsX\"\x12 \xe9x'\x08\x83\xb7\xba\xed\x10\x81\x0c@y\xc9AQ.\x93\xa7\xba\x1c\xd1\x10\x8cx\x1dK\xc78\xd9\t\x05dsolcC\0\x08\x1a\03",
    );
    #[allow(missing_docs)]
    /**Function with signature `number()` and selector `0x8381f58a`.
```solidity
function number() public view returns (uint256 number);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct numberCall {}
    #[automatically_derived]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for numberCall {
        #[inline]
        fn clone(&self) -> numberCall {
            numberCall {}
        }
    }
    #[allow(missing_docs)]
    ///Container type for the return parameters of the [`number()`](numberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct numberReturn {
        pub number: ::alloy::sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for numberReturn {
        #[inline]
        fn clone(&self) -> numberReturn {
            numberReturn {
                number: ::core::clone::Clone::clone(&self.number),
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
        use ::alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numberCall> for UnderlyingRustTuple<'_> {
                fn from(value: numberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy::sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: numberReturn) -> Self {
                    (value.number,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { number: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = numberReturn;
            type ReturnTuple<'a> = (::alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "number()";
            const SELECTOR: [u8; 4] = [131u8, 129u8, 245u8, 138u8];
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
    #[allow(missing_docs)]
    /**Function with signature `setNumber(uint256)` and selector `0x3fb5c1cb`.
```solidity
function setNumber(uint256 newNumber) public { <stmts> }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct setNumberCall {
        pub newNumber: ::alloy::sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for setNumberCall {
        #[inline]
        fn clone(&self) -> setNumberCall {
            setNumberCall {
                newNumber: ::core::clone::Clone::clone(&self.newNumber),
            }
        }
    }
    #[allow(missing_docs)]
    ///Container type for the return parameters of the [`setNumber(uint256)`](setNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct setNumberReturn {}
    #[automatically_derived]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for setNumberReturn {
        #[inline]
        fn clone(&self) -> setNumberReturn {
            setNumberReturn {}
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy::sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: setNumberCall) -> Self {
                    (value.newNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newNumber: tuple.0 }
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
            impl ::core::convert::From<setNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setNumberReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setNumberCall {
            type Parameters<'a> = (::alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setNumberReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setNumber(uint256)";
            const SELECTOR: [u8; 4] = [63u8, 181u8, 193u8, 203u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newNumber),
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
    /**Function with signature `increment()` and selector `0xd09de08a`.
```solidity
function increment() public { <stmts> }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct incrementCall {}
    #[automatically_derived]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for incrementCall {
        #[inline]
        fn clone(&self) -> incrementCall {
            incrementCall {}
        }
    }
    #[allow(missing_docs)]
    ///Container type for the return parameters of the [`increment()`](incrementCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct incrementReturn {}
    #[automatically_derived]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for incrementReturn {
        #[inline]
        fn clone(&self) -> incrementReturn {
            incrementReturn {}
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incrementCall> for UnderlyingRustTuple<'_> {
                fn from(value: incrementCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incrementCall {
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
            impl ::core::convert::From<incrementReturn> for UnderlyingRustTuple<'_> {
                fn from(value: incrementReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incrementReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for incrementCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = incrementReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increment()";
            const SELECTOR: [u8; 4] = [208u8, 157u8, 224u8, 138u8];
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
    #[allow(missing_docs)]
    ///Container for all the [`Counter`](self) function calls.
    pub enum CounterCalls {
        number(numberCall),
        setNumber(setNumberCall),
        increment(incrementCall),
    }
    #[automatically_derived]
    impl CounterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [63u8, 181u8, 193u8, 203u8],
            [131u8, 129u8, 245u8, 138u8],
            [208u8, 157u8, 224u8, 138u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CounterCalls {
        const NAME: &'static str = "CounterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::number(_) => <numberCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setNumber(_) => {
                    <setNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::increment(_) => {
                    <incrementCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<CounterCalls>] = &[
                {
                    fn setNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <setNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(CounterCalls::setNumber)
                    }
                    setNumber
                },
                {
                    fn number(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <numberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(CounterCalls::number)
                    }
                    number
                },
                {
                    fn increment(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <incrementCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(CounterCalls::increment)
                    }
                    increment
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
                Self::number(inner) => {
                    <numberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setNumber(inner) => {
                    <setNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::increment(inner) => {
                    <incrementCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::number(inner) => {
                    <numberCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setNumber(inner) => {
                    <setNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::increment(inner) => {
                    <incrementCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use ::alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Counter`](self) contract instance.

See the [wrapper's documentation](`CounterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CounterInstance<T, P, N> {
        CounterInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<CounterInstance<T, P, N>>,
    > {
        CounterInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        CounterInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Counter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Counter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    pub struct CounterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<
        T: ::core::clone::Clone,
        P: ::core::clone::Clone,
        N: ::core::clone::Clone,
    > ::core::clone::Clone for CounterInstance<T, P, N> {
        #[inline]
        fn clone(&self) -> CounterInstance<T, P, N> {
            CounterInstance {
                address: ::core::clone::Clone::clone(&self.address),
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::clone::Clone::clone(&self._network_transport),
            }
        }
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for CounterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CounterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CounterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Counter`](self) contract instance.

See the [wrapper's documentation](`CounterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<CounterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> CounterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CounterInstance<T, P, N> {
            CounterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CounterInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`number`] function.
        pub fn number(&self) -> alloy_contract::SolCallBuilder<T, &P, numberCall, N> {
            self.call_builder(&numberCall {})
        }
        ///Creates a new call builder for the [`setNumber`] function.
        pub fn setNumber(
            &self,
            newNumber: ::alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setNumberCall, N> {
            self.call_builder(&setNumberCall { newNumber })
        }
        ///Creates a new call builder for the [`increment`] function.
        pub fn increment(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, incrementCall, N> {
            self.call_builder(&incrementCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CounterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
