extern crate libc;

mod ffi;

use ffi::*;
use libc::c_char;
use std::ffi::*;

/// Result of opening a file dialog
pub enum NFDResult {
    /// User pressed okay. `String` is the file path selected
    Okay(Vec<String>),
    /// User pressed cancel
    Cancel,
    /// Program error. `String` is the error description
    Error(String),
}

enum DialogType {
    SingleFile,
    MultiFile,
    SaveFile
}

/// Open single file dialog
#[inline(always)]
pub fn open_file_dialog(filter_list: Option<&str>, default_path: Option<&str>) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::SingleFile)
}

pub fn open_multiple_dialog(filter_list: Option<&str>, default_path: Option<&str>) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::MultiFile)
}

/// Open save dialog
#[inline(always)]
pub fn open_save_dialog(filter_list: Option<&str>, default_path: Option<&str>) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::SaveFile)
}

fn open_dialog(filter_list: Option<&str>, default_path: Option<&str>, dialog_type: &DialogType) -> NFDResult {
    let result: nfdresult_t;
    let result_cstring;

    let filter_list_cstring;
    let filter_list_ptr = match filter_list {
        Some(fl_str) => {
            filter_list_cstring = CString::new(fl_str).unwrap();
            filter_list_cstring.as_ptr()
        }
        None => std::ptr::null()
    };

    let default_path_cstring;
    let default_path_ptr = match default_path {
        Some(dp_str) => {
            default_path_cstring = CString::new(dp_str).unwrap();
            default_path_cstring.as_ptr()
        }
        None => std::ptr::null()
    };

    let mut out_path: *mut c_char = std::ptr::null_mut();
    let ptr_out_path = &mut out_path as *mut *mut c_char;

    unsafe {
        result = match dialog_type {
            &DialogType::SingleFile => {
                NFD_OpenDialog(filter_list_ptr, default_path_ptr, ptr_out_path)
            },

            &DialogType::MultiFile => {
                unimplemented!();
            },

            &DialogType::SaveFile => {
                NFD_SaveDialog(filter_list_ptr, default_path_ptr, ptr_out_path)
            },
        };

        result_cstring = match result {
            nfdresult_t::NFD_OKAY => CStr::from_ptr(out_path).to_owned(),
            nfdresult_t::NFD_ERROR => CStr::from_ptr(NFD_GetError()).to_owned(),
            _ => CString::new("").unwrap()
        }
    }

    let result_string = result_cstring.to_str().unwrap().to_string();

    match result {
        nfdresult_t::NFD_OKAY => NFDResult::Okay(vec![result_string]),
        nfdresult_t::NFD_CANCEL => NFDResult::Cancel,
        nfdresult_t::NFD_ERROR => NFDResult::Error(result_string)
    }
}
