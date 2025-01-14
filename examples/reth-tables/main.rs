use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::fmt;

fn main() {
    println!("{:?}", Tables::Headers.table_type());

    let _v = vec2![1, 2, 3];
}

#[macro_export]
macro_rules! vec2 {
    ($($x:expr),*) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

pub trait Key: Ord + Clone + Serialize + for<'a> Deserialize<'a> {}

pub trait Value: Serialize {}

pub trait Table: Send + Sync + fmt::Debug + 'static {
    /// The table's name.
    const NAME: &'static str;

    /// Whether the table is also a `DUPSORT` table.
    const DUPSORT: bool;

    /// Key element of `Table`.
    ///
    /// Sorting should be taken into account when encoding this.
    type Key: Key;

    /// Value element of `Table`.
    type Value: Value;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TableType {
    /// key value table
    Table,
    /// Duplicate key value table
    DupSort,
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

    // /// Operate on the dupsort table in a generic way.
    // ///
    // /// By default, the `view` function is invoked unless overridden.
    // fn view_dupsort<T: DupSort>(&self) -> Result<R, Self::Error> {
    //     self.view::<T>()
    // }
}

/// General trait for defining the set of tables
/// Used to initialize database
pub trait TableSet {
    /// Returns an iterator over the tables
    fn tables() -> Box<dyn Iterator<Item = Box<dyn TableInfo>>>;
}

/// Defines all the tables in the database.
#[macro_export]
macro_rules! tables {
    (@bool) => {
        false
    };
    (@bool $($t:tt)+) => {
        true
    };

    (@view $name:ident $v:ident) => {
        $v.view::<$name>()
    };
    (@view $name:ident $v:ident $_subkey:ty) => {
        $v.view_dupsort::<$name>()
    };

    (@value_doc $key:ty, $value:ty) => {
        concat!("[`", stringify!($value), "`]")
    };

    // Don't generate links if we have generics
    (@value_doc $key:ty, $value:ty, $($generic:ident),*) => {
        concat!("`", stringify!($value), "`")
    };

    ($($(#[$attr:meta])* table $name:ident$(<$($generic:ident $(= $default:ty)?),*>)? { type Key = $key:ty; type Value = $value:ty; $(type SubKey = $subkey:ty;)? } )*) => {
        // Table marker types.
        $(
            $(#[$attr])*
            ///
            #[doc = concat!("Marker type representing a database table mapping [`", stringify!($key), "`] to ", tables!(@value_doc $key, $value, $($($generic),*)?), ".")]
            $(
                #[doc = concat!("\n\nThis table's `DUPSORT` subkey is [`", stringify!($subkey), "`].")]
            )?
            pub struct $name$(<$($generic $( = $default)?),*>)? {
                _private: std::marker::PhantomData<($($($generic,)*)?)>,
            }

            // Ideally this implementation wouldn't exist, but it is necessary to derive `Debug`
            // when a type is generic over `T: Table`. See: https://github.com/rust-lang/rust/issues/26925
            impl$(<$($generic),*>)? fmt::Debug for $name$(<$($generic),*>)? {
                fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
                    unreachable!("this type cannot be instantiated")
                }
            }

            impl$(<$($generic),*>)? Table for $name$(<$($generic),*>)?
            where
                $value: Value + 'static
                $($(,$generic: Send + Sync)*)?
            {
                const NAME: &'static str = table_names::$name;
                const DUPSORT: bool = tables!(@bool $($subkey)?);

                type Key = $key;
                type Value = $value;
            }

            $(
                impl DupSort for $name {
                    type SubKey = $subkey;
                }
            )?
        )*

        // Tables enum.
        // NOTE: the ordering of the enum does not matter, but it is assumed that the discriminants
        // start at 0 and increment by 1 for each variant (the default behavior).
        // See for example `reth_db::implementation::mdbx::tx::Tx::db_handles`.

        /// A table in the database.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Tables {
            $(
                #[doc = concat!("The [`", stringify!($name), "`] database table.")]
                $name,
            )*
        }

        impl Tables {
            /// All the tables in the database.
            pub const ALL: &'static [Self] = &[$(Self::$name,)*];

            /// The number of tables in the database.
            pub const COUNT: usize = Self::ALL.len();

            /// Returns the name of the table as a string.
            pub const fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$name => table_names::$name,
                    )*
                }
            }

            /// Returns `true` if the table is a `DUPSORT` table.
            pub const fn is_dupsort(&self) -> bool {
                match self {
                    $(
                        Self::$name => tables!(@bool $($subkey)?),
                    )*
                }
            }

            /// The type of the given table in database.
            pub const fn table_type(&self) -> TableType {
                if self.is_dupsort() {
                    TableType::DupSort
                } else {
                    TableType::Table
                }
            }

            /// Allows to operate on specific table type
            pub fn view<T, R>(&self, visitor: &T) -> Result<R, T::Error>
            where
                T: ?Sized + TableViewer<R>,
            {
                match self {
                    $(
                        Self::$name => tables!(@view $name visitor $($subkey)?),
                    )*
                }
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
                    $(
                        table_names::$name => Ok(Self::$name),
                    )*
                    s => Err(format!("unknown table: {s:?}")),
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

        // Need constants to match on in the `FromStr` implementation.
        #[allow(non_upper_case_globals)]
        mod table_names {
            $(
                pub(super) const $name: &'static str = stringify!($name);
            )*
        }

        /// Maps a run-time [`Tables`] enum value to its corresponding compile-time [`Table`] type.
        ///
        /// This is a simpler alternative to [`TableViewer`].
        ///
        /// # Examples
        ///
        /// ```
        /// use reth_db::{Tables, tables_to_generic};
        /// use reth_db_api::table::Table;
        ///
        /// let table = Tables::Headers;
        /// let result = tables_to_generic!(table, |GenericTable| <GenericTable as Table>::NAME);
        /// assert_eq!(result, table.name());
        /// ```
        #[macro_export]
        macro_rules! tables_to_generic {
            ($table:expr, |$generic_name:ident| $e:expr) => {
                match $table {
                    $(
                        Tables::$name => {
                            use $crate::tables::$name as $generic_name;
                            $e
                        },
                    )*
                }
            };
        }
    };
}

type BlockNumber = u64;

impl Key for BlockNumber {}

#[derive(Serialize)]
pub struct StoredBlockBody {
    pub first_tx_number: u64,
    pub tx_count: u64,
}

#[derive(Serialize)]
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
impl Value for Header {}
impl Value for StoredBlockBody {}

tables! {
    table Headers {
        type Key = BlockNumber;
        type Value = Header;
    }

    table BlockBodyIndices {
        type Key = BlockNumber;
        type Value = StoredBlockBody;
    }
}
