mod raw {
    #![allow(unused)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let example_cstr = std::ffi::CString::new("Hello world").unwrap();

    let len = unsafe {
        raw::strlen(example_cstr.as_bytes_with_nul() as *const _ as *const std::ffi::c_char)
    };

    println!("Length of string was {}", len);
}
