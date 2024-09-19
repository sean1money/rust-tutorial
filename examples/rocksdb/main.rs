use anyhow::Result;
use rocksdb::{ColumnFamilyDescriptor, Error, Options};

fn main() -> Result<()> {
    example1()?;

    table_test()?;

    column_family_test()?;

    Ok(())
}

fn example1() -> Result<()> {
    let db = rocksdb::DB::open_default("target/rocksdb")?;
    db.put(b"my key", b"my value")?;
    match db.get(b"my key")? {
        Some(value) => {
            println!("retrieved value: {:?}", value);
        }
        None => {
            println!("value not found");
        }
    }
    db.delete(b"my key")?;

    Ok(())
}

fn column_family_test() -> Result<()> {
    let path = "target/test_cf";
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);

    let txs_cf = ColumnFamilyDescriptor::new("transactions", Options::default());
    let accounts_cf = ColumnFamilyDescriptor::new("accounts", Options::default());

    let cfs = vec![txs_cf, accounts_cf];
    let db = rocksdb::DB::open_cf_descriptors(&db_opts, path, cfs)?;

    let txs_cf_handle = db
        .cf_handle("transactions")
        .expect("Failed to get cf handle");
    let accounts_cf_handle = db.cf_handle("accounts").expect("Failed to get cf handle");

    db.put_cf(
        txs_cf_handle,
        "tx.1".as_bytes(),
        "{a json string}".as_bytes(),
    )?;

    db.put_cf(
        txs_cf_handle,
        "tx.2".as_bytes(),
        "{a json string}".as_bytes(),
    )?;

    db.put_cf(
        accounts_cf_handle,
        "0xaaa".as_bytes(),
        "{account info}".as_bytes(),
    )?;

    let tx = db.get_cf(txs_cf_handle, "tx.1".as_bytes())?;
    let tx_no_exist = db.get_cf(txs_cf_handle, "tx.3".as_bytes())?;
    let account = db.get_cf(accounts_cf_handle, "0xaaa".as_bytes())?;

    println!("tx: {:?}", tx);
    println!("tx_no_exist: {:?}", tx_no_exist);
    println!("account: {:?}", account);

    Ok(())
}

fn table_test() -> Result<()> {
    let table = Table::new("target/test_table")?;

    table.insert("k1", "v1")?;
    table.insert("k2", "v2")?;

    match table.get("k1").expect("Failed to read data") {
        Some(value) => {
            println!("retrieved value: {:?}", value);
        }
        None => {
            println!("value not found");
        }
    }

    // delete the key: k1
    table.delete("k1")?;
    match table.get("k1").expect("Faield to read data") {
        Some(value) => {
            println!("k1 still exist: {:?}", value);
        }
        None => {
            println!("k1 successfully deleted");
        }
    }

    Ok(())
}

pub struct Table {
    db: rocksdb::DB,
}

impl Table {
    pub fn new(path: &str) -> Result<Self> {
        let mut options = Options::default();
        // if the db is not exist, create it
        options.create_if_missing(true);

        // open the db
        let db = rocksdb::DB::open(&options, path)?;

        Ok(Table { db })
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<(), Error> {
        self.db.put(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Error> {
        match self.db.get(key.as_bytes())? {
            Some(value) => Ok(Some(
                String::from_utf8(value).expect("Invalid UTF-8 sequence"),
            )),
            None => Ok(None),
        }
    }

    pub fn delete(&self, key: &str) -> Result<(), Error> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }

    pub fn close(self) {
        drop(self.db);
    }
}
