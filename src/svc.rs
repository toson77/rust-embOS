use crate::led;
use cortex_m_semihosting::hprintln;

static mut svc_inst: usize = 0;
static mut return_psp_val: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn SvcHandler() {
    /*
    asm!(
        "
        mrs r1, psp
        ldr r3, [r1, #24]
        ldr r2, [r3, #-2]
        ":"={r1}"(return_psp_val),"={r2}"(svc_inst)::::"volatile"
    );
    let svc_arg = svc_inst as u8;
    hprintln!("svc handler arg: {}", svc_arg);
    */
    let svc_arg = 0;
    match svc_arg {
        0 => {
            to_kernel();
            return;
        }
        1 => {
            hprintln!("switching led val").unwrap();
            led::switch();
        }
        _ => (),
    }

    llvm_asm!(
        "
        mov r0, #1
        msr CONTROL, r0
        movw lr, #0xfffd
        movt lr, #0xffff
        bx lr
        "::"{r1}"(return_psp_val)::"volatile");
}

unsafe extern "C" fn to_kernel() {
    llvm_asm!(
        "
        movw lr, #0xfff9
        movt lr, #0xffff
        bx lr
        "
        ::::"volatile");
}

pub fn switch_led() {
    unsafe {
        llvm_asm!("svc 1"::::"volatile");
    }
}
