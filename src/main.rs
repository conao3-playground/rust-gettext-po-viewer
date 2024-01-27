use std::ffi::CString;

extern "C" {
    fn double_input(input: libc::c_int) -> libc::c_int;
    fn c_po_file_read(filename: *const libc::c_char) -> *mut libc::c_void;
    fn c_po_message_iterator(file: *mut libc::c_void) -> *mut libc::c_void;
    fn c_po_next_message(iterator: *mut libc::c_void) -> *mut libc::c_void;
    fn c_po_message_msgid(message: *mut libc::c_void) -> *mut libc::c_char;
    fn c_po_message_msgstr(message: *mut libc::c_void) -> *mut libc::c_char;
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

    let iterator = unsafe { c_po_message_iterator(file) };
    println!("{:?}", iterator);

    let message = unsafe { c_po_next_message(iterator) };
    println!("{:?}", message);

    let msgid = unsafe { c_po_message_msgid(message) };
    let msgid = unsafe { CString::from_raw(msgid) };
    let msgid = msgid.to_str().unwrap();
    println!("msgid: {:?}", msgid);

    let msgstr = unsafe { c_po_message_msgstr(message) };
    let msgstr = unsafe { CString::from_raw(msgstr) };
    let msgstr = msgstr.to_str().unwrap();
    println!("msgstr: {:?}", msgstr);
}
