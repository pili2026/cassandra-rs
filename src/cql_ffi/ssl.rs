#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;

use cql_ffi::error::CassError;
use cql_ffi::string::CassString;
use cql_bindgen::CassSsl as _CassSsl;
use cql_bindgen::cass_ssl_new;
use cql_bindgen::cass_ssl_free;
use cql_bindgen::cass_ssl_add_trusted_cert;
use cql_bindgen::cass_ssl_set_verify_flags;
use cql_bindgen::cass_ssl_set_cert;
use cql_bindgen::cass_ssl_set_private_key;

pub struct CassSsl(pub *mut _CassSsl);

impl Drop for CassSsl {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl CassSsl {
    pub unsafe fn new() -> CassSsl {CassSsl(cass_ssl_new())}
    unsafe fn free(&mut self) {cass_ssl_free(self.0)}
    pub unsafe fn add_trusted_cert<'a>(&'a mut self, cert: CassString) -> Result<&'a Self,CassError> {CassError::build(cass_ssl_add_trusted_cert(self.0, cert.0)).wrap(self)}
    pub unsafe fn set_verify_flags(&mut self, flags: c_int) {cass_ssl_set_verify_flags(self.0,flags)}
    pub unsafe fn set_cert<'a>(&'a mut self, cert: CassString) -> Result<&'a Self,CassError> {CassError::build(cass_ssl_set_cert(self.0,cert.0)).wrap(self)}
    pub unsafe fn set_private_key<'a>(&'a mut self, key: CassString, password: *const c_char) -> Result<&'a Self,CassError> {CassError::build(cass_ssl_set_private_key(self.0,key.0, password)).wrap(self)}
}
