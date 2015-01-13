#[lang="fail_bounds_check"]
fn fail_bounds_check(_: &(&'static str, usize),
                     _: usize, _: usize) -> ! {
    loop{}
}

//override system abort to avoid pulling in memory allocation dependencies
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop{}
}
