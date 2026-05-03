//! lx32_prueba — integration test for the LX32 Rust compiler fork.
//!
//! This crate is a minimal standalone proof-of-concept that exercises the
//! custom LX32 LLVM backend through the forked rustc.  It verifies that:
//!
//! - `#![no_std]` / `#![no_main]` compiles without errors.
//! - Basic arithmetic, pointer stores, loops, and calls lower correctly.
//! - The custom `lx.sensor` / `lx.wait` / `lx.report` instructions assemble.
//!
//! Build (after `make setup-rust`):
//!   RUSTC=.../stage1/bin/rustc cargo build --release
#![no_std]
#![no_main]
#![allow(asm_sub_register)]

// ── Utilities ─────────────────────────────────────────────────────────────────

fn add(a: i32, b: i32) -> i32 { a + b }

fn fib(n: i32) -> i32 {
    if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Basic arithmetic.
    let sum = add(10, 20);          // 30
    let _ = fib(8);                 // 21 — recursive call chain

    // Stack array + pointer store.
    let mut buf = [0u32; 4];
    buf[0] = sum as u32;
    buf[1] = buf[0] * 2;

    // Accumulator loop.
    let mut acc = 0u32;
    for i in 0..8u32 {
        acc = acc.wrapping_add(i);
    }
    let _ = acc;

    // Custom LX32K instructions — only assembled when targeting lx32.
    #[cfg(target_arch = "lx32")]
    unsafe {
        let _v: i32;
        core::arch::asm!(
            "lx.sensor {rd}, {rs1}",
            rd  = out(reg) _v,
            rs1 = in(reg)  buf[0],
            options(nomem, nostack, pure),
        );
        core::arch::asm!(
            "lx.wait {rs1}",
            rs1 = in(reg) 50u32,
            options(nostack),
        );
        let report: [u8; 8] = [0x01, 0x00, 0x04, 0, 0, 0, 0, 0];
        core::arch::asm!(
            "lx.report {rs1}",
            rs1 = in(reg) report.as_ptr(),
            options(nostack),
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
