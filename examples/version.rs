use bitwuzla_sys::bitwuzla_version;
use std::ffi::CStr;

fn main() {
    unsafe {
        let version_cstr = bitwuzla_version();
        let version = CStr::from_ptr(version_cstr);
        println!("bitwuzla_version: {version:?}");
    }
}
