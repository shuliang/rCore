pub unsafe fn set_cpu_id(cpu_id: usize) {
    llvm_asm!("mv tp, $0" :: "r"(cpu_id));
}

pub fn id() -> usize {
    let cpu_id;
    unsafe {
        llvm_asm!("mv $0, tp" : "=r"(cpu_id));
    }
    cpu_id
}
