//? Default handler
pub extern "C" fn default_handler() {
    panic!("Unhandled interrupt");
    loop {}
}

//? Page fault handler
pub extern "C" fn page_fault() {
    panic!("Page fault");
    loop {}
}

//? Double fault handler
pub extern "C" fn double_fault() {
    panic!("Double fault");
    loop {}
}

//? Breakpoint handler
pub extern "C" fn breakpoint() {
    panic!("Breakpoint");
    loop {}
}