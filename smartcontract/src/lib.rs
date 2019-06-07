use std::str;
use std::slice;


extern "C" {
    fn random_number() -> i32;
}

#[no_mangle]
pub extern fn multiply_by_two(num: i32) -> i32 {
   num * 2 
}

#[no_mangle]
pub extern fn multiply_by_random(num: i32) -> i32 {
   num * unsafe { random_number() }
}

// #[no_mangle]
// pub extern fn hello_string_from_rust(ptr: i32, len: i32) {
//     let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
//     let string_from_host = str::from_utf8(&slice).unwrap();
//     let out_str = format!("Hello {}", string_from_host);
//     unsafe {
//       print_str(out_str.as_ptr(), out_str.len());
//     }
// }
