use cassandra_sys::cass_table_meta_clustering_key;
use cassandra_sys::cass_table_meta_clustering_key_count;
use cassandra_sys::cass_table_meta_column;
use cassandra_sys::cass_table_meta_column_by_name;
use cassandra_sys::cass_table_meta_column_count;
use cassandra_sys::cass_table_meta_field_by_name;
use cassandra_sys::cass_table_meta_name;
use cassandra_sys::cass_table_meta_partition_key;
use cassandra_sys::cass_table_meta_partition_key_count;
use cassandra_sys::CassTableMeta as _CassTableMeta;
use cassandra_sys::cass_iterator_columns_from_table_meta;
use cassandra_sys::cass_iterator_fields_from_table_meta;
use cassandra::iterator;
use cassandra::value;
use cassandra::iterator::FieldIterator;
use cassandra::iterator::ColumnIterator;

use std::str;
use std::slice;
use std::mem;

use cassandra::schema::column_meta;
use cassandra::schema::column_meta::ColumnMeta;
use cassandra::value::Value;

///Table metadata
pub struct TableMeta(*const _CassTableMeta);

pub mod protected {
    use cassandra_sys::CassTableMeta as _CassTableMeta;
    use cassandra::schema::table_meta::TableMeta;
    pub fn build(table: *const _CassTableMeta) -> TableMeta {
        TableMeta(table)
    }
}

impl TableMeta {
    ///returns an iterator over the fields of this table
    pub fn field_iter(&mut self) -> FieldIterator {
        unsafe { iterator::protected::CassIterator::build(cass_iterator_fields_from_table_meta(self.0)) }
    }

    ///An iterator over the columns in this table
    pub fn columns_iter(&self) -> ColumnIterator {
        unsafe { iterator::protected::CassIterator::build(cass_iterator_columns_from_table_meta(self.0)) }
    }

    ///Gets the column metadata for the provided column name.
    pub fn column_by_name(&self, name: &str) -> ColumnMeta {
        unsafe { column_meta::protected::build(cass_table_meta_column_by_name(self.0, name.as_ptr() as *const i8)) }
    }

    ///Gets the name of the table.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_table_meta_name(self.0, &mut name, &mut name_length);
            let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }

    ///Gets the total number of columns for the table.
    pub fn column_count(&self) -> u64 {
        unsafe { cass_table_meta_column_count(self.0) }
    }

    ///Gets the column metadata for the provided index.
    pub fn column(&self, index: u64) -> ColumnMeta {
        unsafe { column_meta::protected::build(cass_table_meta_column(self.0, index)) }
    }

    ///Gets the number of columns for the table's partition key.
    pub fn partition_key_count(&self) -> u64 {
        unsafe { cass_table_meta_partition_key_count(self.0) }
    }

    ///Gets the partition key column metadata for the provided index.
    pub fn partition_key(&self, index: u64) -> Option<ColumnMeta> {
        unsafe {
            let key = cass_table_meta_partition_key(self.0, index);
            if key.is_null() { None } else { Some(column_meta::protected::build(key)) }
        }
    }

    ///Gets the number of columns for the table's clustering key
    pub fn clustering_key_count(&self) -> u64 {
        unsafe { cass_table_meta_clustering_key_count(self.0) }
    }

    ///Gets the clustering key column metadata for the provided index.
    pub fn cluster_key(&self, index: u64) -> Option<ColumnMeta> {
        unsafe {
            let key = cass_table_meta_clustering_key(self.0, index);
            if key.is_null() { None } else { Some(column_meta::protected::build(key)) }
        }
    }


    ///Gets a metadata field for the provided name. Metadata fields allow direct
    ///access to the column data found in the underlying "tables" metadata table.
    pub fn field_by_name(&self, name: &str) -> Option<Value> {
        // fixme replace CassValule with a custom type
        unsafe {
            let value = cass_table_meta_field_by_name(self.0, name.as_ptr() as *const i8);
            if value.is_null() { None } else { Some(value::protected::build(value)) }

        }
    }
}
