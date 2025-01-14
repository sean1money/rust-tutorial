#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::fmt;
fn main() {
    {
        ::std::io::_print(format_args!("{0:?}\n", Tables::Headers.table_type()));
    };
}
pub trait Key: Ord + Clone + Serialize + for<'a> Deserialize<'a> {}
pub trait Value: Serialize {}
pub trait Table: Send + Sync + fmt::Debug + 'static {
    /// The table's name.
    const NAME: &str;
    const CF: u64;
    const KEY_PREFIX: &str;
    /// Whether the table is also a `DUPSORT` table.
    /// Key element of `Table`.
    ///
    /// Sorting should be taken into account when encoding this.
    type Key: Key;
    /// Value element of `Table`.
    type Value: Value;
}
pub enum TableType {
    /// key value table
    Table,
    /// Duplicate key value table
    DupSort,
}
#[automatically_derived]
impl ::core::fmt::Debug for TableType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TableType::Table => "Table",
                TableType::DupSort => "DupSort",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TableType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TableType {
    #[inline]
    fn eq(&self, other: &TableType) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TableType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::Copy for TableType {}
#[automatically_derived]
impl ::core::clone::Clone for TableType {
    #[inline]
    fn clone(&self) -> TableType {
        *self
    }
}
/// Trait that provides object-safe access to the table's metadata.
pub trait TableInfo: Send + Sync + fmt::Debug + 'static {
    /// The table's name.
    fn name(&self) -> &'static str;
    /// Whether the table is a `DUPSORT` table.
    fn is_dupsort(&self) -> bool;
}
pub trait TableViewer<R> {
    /// The error type returned by the viewer.
    type Error;
    /// Calls `view` with the correct table type.
    fn view_rt(&self, table: Tables) -> Result<R, Self::Error> {
        table.view(self)
    }
    /// Operate on the table in a generic way.
    fn view<T: Table>(&self) -> Result<R, Self::Error>;
}
/// General trait for defining the set of tables
/// Used to initialize database
pub trait TableSet {
    /// Returns an iterator over the tables
    fn tables() -> Box<dyn Iterator<Item = Box<dyn TableInfo>>>;
}
type BlockNumber = u64;
impl Key for BlockNumber {}
pub struct StoredBlockBody {
    pub first_tx_number: u64,
    pub tx_count: u64,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for StoredBlockBody {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "StoredBlockBody",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "first_tx_number",
                &self.first_tx_number,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "tx_count",
                &self.tx_count,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
pub struct Header {
    pub hash: B256,
    pub parent_hash: B256,
    pub number: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub extra_data: B256,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub base_fee_per_gas: u64,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Header {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Header",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "hash",
                &self.hash,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "parent_hash",
                &self.parent_hash,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "number",
                &self.number,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "timestamp",
                &self.timestamp,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "nonce",
                &self.nonce,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "extra_data",
                &self.extra_data,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "gas_limit",
                &self.gas_limit,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "gas_used",
                &self.gas_used,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "base_fee_per_gas",
                &self.base_fee_per_gas,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl Value for Header {}
impl Value for StoredBlockBody {}
///
///Marker type representing a database table mapping [`BlockNumber`] to `Header`.
pub struct Headers {
    _marker: std::marker::PhantomData<()>,
}
#[automatically_derived]
impl ::core::clone::Clone for Headers {
    #[inline]
    fn clone(&self) -> Headers {
        Headers {
            _marker: ::core::clone::Clone::clone(&self._marker),
        }
    }
}
impl fmt::Debug for Headers {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "internal error: entered unreachable code: {0}",
                    format_args!("this type cannot be instantiated"),
                ),
            );
        }
    }
}
impl Table for Headers
where
    Header: Value + 'static,
{
    const NAME: &'static str = table_names::Headers;
    const CF: u64 = 1;
    const KEY_PREFIX: &str = "header";
    type Key = BlockNumber;
    type Value = Header;
}
///
///Marker type representing a database table mapping [`BlockNumber`] to `StoredBlockBody`.
pub struct BlockBodyIndices {
    _marker: std::marker::PhantomData<()>,
}
impl fmt::Debug for BlockBodyIndices {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "internal error: entered unreachable code: {0}",
                    format_args!("this type cannot be instantiated"),
                ),
            );
        }
    }
}
impl Table for BlockBodyIndices
where
    StoredBlockBody: Value + 'static,
{
    const NAME: &'static str = table_names::BlockBodyIndices;
    const CF: u64 = 2;
    const KEY_PREFIX: &str = "";
    type Key = BlockNumber;
    type Value = StoredBlockBody;
}
/// A table in the database.
pub enum Tables {
    ///The [`Headers`] database table.
    Headers,
    ///The [`BlockBodyIndices`] database table.
    BlockBodyIndices,
}
#[automatically_derived]
impl ::core::clone::Clone for Tables {
    #[inline]
    fn clone(&self) -> Tables {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tables {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Tables {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Tables {
    #[inline]
    fn eq(&self, other: &Tables) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Tables {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for Tables {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state)
    }
}
impl Tables {
    /// All the tables in the database.
    pub const ALL: &'static [Self] = &[Self::Headers, Self::BlockBodyIndices];
    /// The number of tables in the database.
    pub const COUNT: usize = Self::ALL.len();
    /// Returns the name of the table as a string.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Headers => table_names::Headers,
            Self::BlockBodyIndices => table_names::BlockBodyIndices,
        }
    }
    /// Returns `true` if the table is a `DUPSORT` table.
    pub const fn is_dupsort(&self) -> bool {
        false
    }
    /// The type of the given table in database.
    pub const fn table_type(&self) -> TableType {
        if self.is_dupsort() { TableType::DupSort } else { TableType::Table }
    }
    /// Allows to operate on specific table type
    pub fn view<T, R>(&self, _visitor: &T) -> Result<R, T::Error>
    where
        T: ?Sized + TableViewer<R>,
    {
        ::core::panicking::panic("not yet implemented")
    }
}
impl fmt::Debug for Tables {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
impl fmt::Display for Tables {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name().fmt(f)
    }
}
impl std::str::FromStr for Tables {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            table_names::Headers => Ok(Self::Headers),
            table_names::BlockBodyIndices => Ok(Self::BlockBodyIndices),
            s => {
                Err(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("unknown table: {0:?}", s),
                        );
                        res
                    }),
                )
            }
        }
    }
}
impl TableInfo for Tables {
    fn name(&self) -> &'static str {
        self.name()
    }
    fn is_dupsort(&self) -> bool {
        self.is_dupsort()
    }
}
impl TableSet for Tables {
    fn tables() -> Box<dyn Iterator<Item = Box<dyn TableInfo>>> {
        Box::new(Self::ALL.iter().map(|table| Box::new(*table) as Box<dyn TableInfo>))
    }
}
#[allow(non_upper_case_globals)]
mod table_names {
    pub(super) const Headers: &'static str = "Headers";
    pub(super) const BlockBodyIndices: &'static str = "BlockBodyIndices";
}
