use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[link(name = "swiftDemangle")]
extern "C" {
    fn swift_demangle(s: *const c_char) -> *const c_char;
}

pub fn demangle(mangled: String) -> Option<String> {
    let mangled = CString::new(mangled).expect("CString::new failed");
    let demangled = unsafe{ swift_demangle(mangled.as_ptr()) };
    if demangled == 0x0 as *const i8{
        return None;
    }
    let demangled = unsafe{ CStr::from_ptr(demangled)};
    Some(demangled.to_str().ok()?.to_string())
}

#[test]
fn test_demangle() {
    let out = demangle("foo".into());
    assert!(out.is_none());

    let out = demangle("$sSasSQRzlE2eeoiySbSayxG_ABtFZ".into());
    println!("rust OUT: {out:?}");
    assert!(out.is_some());
    assert_eq!(out, Some("static (extension in Swift):Swift.Array<A where A: Swift.Equatable>.== infix(Swift.Array<A>, Swift.Array<A>) -> Swift.Bool".to_string()));
}
