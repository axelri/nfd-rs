use libc::{c_char, size_t};

#[repr(C)]
pub enum nfdresult_t {
    NFD_ERROR,
    NFD_OKAY,
    NFD_CANCEL,
}

#[repr(C)]
pub struct nfdpathset_t {
    pub buf: *mut c_char,
    pub indices: *mut size_t,
    pub count: size_t,
}

#[link(name = "nfd")]
extern "C" {
    pub fn NFD_OpenDialog(filter_list: *const c_char,
                          default_path: *const c_char,
                          outPath: *mut *mut c_char)
                          -> nfdresult_t;

    pub fn NFD_SaveDialog(filter_list: *const c_char,
                          default_path: *const c_char,
                          outPath: *mut *mut c_char)
                          -> nfdresult_t;

    pub fn NFD_OpenDialogMultiple(filter_list: *const c_char,
                                  default_path: *const c_char,
                                  outPaths: *mut nfdpathset_t)
                                  -> nfdresult_t;

    pub fn NFD_GetError() -> *const c_char;

    pub fn NFD_PathSet_GetCount(path_set: *const nfdpathset_t) -> size_t;

    pub fn NFD_PathSet_GetPath(path_set: *const nfdpathset_t, index: size_t) -> *mut c_char;

    pub fn NFD_PathSet_Free(path_set: *mut nfdpathset_t);
}
