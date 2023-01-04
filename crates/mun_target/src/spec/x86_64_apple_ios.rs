use crate::spec::{Target, TargetOptions};
use crate::spec::apple_base::{Arch, ios_sim_llvm_target, opts};

pub fn target() -> Target {
    let arch = Arch::X86_64_sim;
    Target {
        llvm_target: ios_sim_llvm_target(arch).into(),
        pointer_width: 64,
        data_layout: "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .into(),
        arch: arch.target_arch(),
        options: TargetOptions {
            ..opts("ios", arch)
        },
    }
}
