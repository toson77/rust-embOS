use crate::led;
use crate::syscall_id;

#[no_mangle]
#[used]
pub static mut SYSCALL_FIRED: usize = 0;

pub fn led_on() {
    unsafe {
        asm!(
            "svc 1",
            in("r0") syscall_id::LED_ON,
        );
    }
}

pub fn led_off() {
    unsafe {
        asm!(
            "svc 1",
            in("r0") syscall_id::LED_OFF,
        );
    }
}
pub fn thread_wait() {
    unsafe {
        asm!(
            "svc 1",
            in("r0") syscall_id::THRED_WAIT,
        );
    }
}
pub fn thread_sleep() {
    unsafe {
        asm!(
            "svc 1",
            in("r0") syscall_id::THRED_SLEEP,
        );
    }
}
pub fn thread_wakeup() {
    unsafe {
        asm!(
            "svc 1",
            in("r0") syscall_id::THRED_WAKEUP,
        );
    }
}

pub fn thread_chpri(priority: u32) {
    unsafe {
        asm!(
            "svc 1",
            in("r0") syscall_id::THREAD_CHPRI,
        )
    }
}
fn led_on_kernel() {
    led::init();
    led::turn_on();
}
/*
fn exec() {
    unsafe {
        llvm_asm!(
            "
            msr psp, r0
            ldmia r1, {r4-r11}
            svc 0
            stmia r1, {r4-r11}
            mrs r0, psp
            "
            :"={r0}"(self.sp)
            :"{r0}"(self.sp), "{r1}"(&self.regs)
            :"r4","r5","r6","r8","r9","r10","r11"
            :"volatile"
        );
    }
}

pub fn privirage_task() {
    match (SysCall) {
        LED_ON => led_on_kernel(),
        _ => (),
    }
    exec();
}
*/
