#![feature(lang_items,asm,start,compiler_builtins_lib)]
#![no_std]
#![crate_type="staticlib"]

extern crate compiler_builtins;

#[lang="eh_personality"]
extern "C" fn eh_personality() {}

#[lang="panic_fmt"]
pub extern "C" fn panic_fmt(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> () {
    loop {}
}

extern "C" {
    fn _estack();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [Some(_estack), // Stack pointer
                                                               Some(startup), // Reset
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
];


// There's a level of indirection here because of the required signature of lang_start. The reset
// vector jumps to startup(), but Rust requires a lang_start(isize, *u8) function. #[start] in
// this case turns lang_start into something equivalent to #[no_mangle] pub fn main() (which you
// cannot directly define because it thus _becomes_ lang_start). The actual start code ends up
// looking like this:
//
//00000000 <ISRVectors>:
//   0:   1ffff800        ; stack pointer
//   4:   00000045        ; address of startup()
//
// 00000044 <_ZN17bare_minimum_rust7startup17hd6300ffb5a62d3fcE>:
//  44:   b580            push    {r7, lr}
//  46:   af00            add     r7, sp, #0
//  48:   f000 f802       bl      50 <_ZN17bare_minimum_rust4main17h1a49112664de8058E>
//  4c:   e7ff            b.n     4e <_ZN17bare_minimum_rust7startup17hd6300ffb5a62d3fcE+0xa>
//  4e:   bd80            pop     {r7, pc}
//
//00000050 <_ZN17bare_minimum_rust4main17h1a49112664de8058E>:
//  50:   e7ff            b.n     52 <_ZN17bare_minimum_rust4main17h1a49112664de8058E+0x2>
//  52:   e7fe            b.n     52 <_ZN17bare_minimum_rust4main17h1a49112664de8058E+0x2>
//
// I could make the signature of my ISRVector entries match lang_start, but that would make
// invoking them harder (and for no real gain).

#[start]
fn lang_start(_: isize, _: *const *const u8) -> isize {
    main();
    0
}

pub unsafe extern "C" fn startup() {
    main();
}

pub fn main() {
    loop {
    }
}
