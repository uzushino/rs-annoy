use std::ffi::CString;
use std::path::{Path, PathBuf};

use libc::{c_float, c_int, c_uint, c_void};

pub enum AnnoyIndexInterface {}

pub mod ffi {
    use super::*;

    #[link(name = "binding", kind = "static")]
    extern "C" {
        pub fn annoy_index_angular(f: c_int) -> *mut AnnoyIndexInterface;
        pub fn annoy_delete_index(index: *mut AnnoyIndexInterface);
        pub fn annoy_add_item(index: *mut AnnoyIndexInterface, item: c_int, w: *const c_float);
        pub fn annoy_build(index: *mut AnnoyIndexInterface, q: c_int);

        pub fn annoy_load(index: *mut AnnoyIndexInterface, p: *const c_void);
        pub fn annoy_save(index: *mut AnnoyIndexInterface, p: *const c_void);

        pub fn annoy_get_item(index: *mut AnnoyIndexInterface, item: c_int, result: *mut c_float);

        pub fn annoy_get_nns_by_item(
            index: *mut AnnoyIndexInterface,
            item: c_int,
            n: c_int,
            search_k: c_int,
            result: *mut c_int,
            distances: *mut c_float,
        );

        pub fn annoy_get_nns_by_vector(
            index: *mut AnnoyIndexInterface,
            w: *const c_float,
            n: c_int,
            search_k: c_int,
            result: *mut c_int,
            distances: *mut c_float,
        );

        pub fn annoy_set_seed(index: *mut AnnoyIndexInterface, q: c_uint);
    }
}

pub struct Rannoy(usize, *mut AnnoyIndexInterface);

/// SAFETY:
/// - `Rannoy` is `Send` because we have exclusive access to `*mut AnnoyIndexInterface`
/// and there is no shared mutable state.
unsafe impl Send for Rannoy {}

impl Rannoy {
    pub fn new(n: usize) -> Self {
        let index = unsafe { ffi::annoy_index_angular(n as i32) };

        Rannoy(n, index)
    }

    pub fn add_item(&self, item: i32, w: &[f32]) {
        unsafe {
            ffi::annoy_add_item(self.1, item, w.as_ptr());
        }
    }

    pub fn build(&self, n: i32) {
        unsafe {
            ffi::annoy_build(self.1, n);
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        unsafe {
            if let Some(f) = path.as_ref().as_os_str().to_str() {
                let path_str_c = CString::new(f).unwrap();
                ffi::annoy_save(self.1, path_str_c.as_ptr() as *const c_void);
            }
        }
    }

    pub fn load(&self, path: PathBuf) {
        unsafe {
            if let Some(f) = path.to_str() {
                ffi::annoy_load(self.1, f.as_ptr() as *const c_void);
            }
        }
    }

    pub fn get_nns_by_item(&self, item: i32, n: i32, search_k: i32) -> (Vec<i32>, Vec<f32>) {
        let mut result = Vec::with_capacity(self.0);
        let result_ptr = result.as_mut_ptr();

        let mut distance = Vec::with_capacity(self.0);
        let distance_ptr = distance.as_mut_ptr();

        unsafe {
            ffi::annoy_get_nns_by_item(self.1, item, n, search_k, result_ptr, distance_ptr);

            let a = std::slice::from_raw_parts_mut(result_ptr, n as usize);
            let b = std::slice::from_raw_parts_mut(distance_ptr, n as usize);

            (a.to_vec(), b.to_vec())
        }
    }

    pub fn get_nns_by_vector(&self, w: Vec<f32>, n: i32, search_k: i32) -> (Vec<i32>, Vec<f32>) {
        let mut result = Vec::with_capacity(self.0);
        let result_ptr = result.as_mut_ptr();

        let mut distance = Vec::with_capacity(self.0);
        let distance_ptr = distance.as_mut_ptr();

        unsafe {
            ffi::annoy_get_nns_by_vector(self.1, w.as_ptr(), n, search_k, result_ptr, distance_ptr);

            let a = std::slice::from_raw_parts_mut(result_ptr, n as usize);
            let b = std::slice::from_raw_parts_mut(distance_ptr, n as usize);

            (a.to_vec(), b.to_vec())
        }
    }

    pub fn set_seed(&self, q: u32) {
        unsafe {
            ffi::annoy_set_seed(self.1, q);
        }
    }
}

impl Drop for Rannoy {
    fn drop(&mut self) {
        unsafe {
            ffi::annoy_delete_index(self.1);
        }
    }
}
