use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        arch: "dpu".to_string(),
        data_layout: "e-m:e-p:32:32-i1:8:32-i8:8:32-i16:16:32-i32:32:32-i64:64:64-n32".to_string(),
        llvm_target: "dpu-upmem-dpurte".to_string(),
        
        target_os: "dpurte".to_string(),
        target_vendor: "upmem".to_string(),
        target_env: String::new(),
        
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),

        options: TargetOptions {
            executables: true,
            max_atomic_width: Some(64),
            atomic_cas: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: "static".to_string(),
            emit_debug_gdb_scripts: false,

            .. Default::default( )
        }
    })
}
