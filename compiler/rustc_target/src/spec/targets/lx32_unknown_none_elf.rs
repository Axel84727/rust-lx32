// Target specification for the PULSAR LX32K keyboard processor.
//
// This is a 32-bit custom RISC-V-derived ISA with six hardware instructions
// for Hall-effect sensor reads, USB HID DMA, and pipeline timing control.
//
// Tier: 3  (custom silicon, no upstream LLVM, hosted in a separate fork)

use crate::spec::{
    Arch, Cc, FramePointer, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetMetadata,
    TargetOptions,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S32".into(),
        llvm_target: "lx32-unknown-none-elf".into(),
        metadata: TargetMetadata {
            description: Some("PULSAR LX32K keyboard processor (bare metal, no std)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 32,
        arch: Arch::Lx32,

        options: TargetOptions {
            // Use lld — it ships with the custom LLVM build.
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("ld.lld".into()),

            cpu: "generic".into(),

            // No hardware atomics on this ISA.
            max_atomic_width: Some(0),
            atomic_cas: false,

            // Firmware always aborts on panic — no unwinding on 4KB SRAM.
            panic_strategy: PanicStrategy::Abort,

            // Static relocation model — firmware lives at a fixed base address.
            relocation_model: RelocModel::Static,

            // Keep frame pointers for reliable stack traces on the simulator.
            frame_pointer: FramePointer::Always,

            // No default libraries or debug scripts — pure bare metal.
            no_default_libraries: true,
            emit_debug_gdb_scripts: false,

            // Produce standalone executables.
            executables: true,

            ..Default::default()
        },
    }
}
