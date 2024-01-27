use std::ffi::CString;

extern "C" {
    fn double_input(input: libc::c_int) -> libc::c_int;
    fn c_po_file_read(filename: *const libc::c_char) -> *mut libc::c_void;
}

fn main() {
    println!("Hello, world!");

    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let filename = CString::new(args[1].as_str()).unwrap();
    let file = unsafe { c_po_file_read(filename.as_ptr()) };
    println!("{:?}", file);
}
