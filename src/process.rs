use crate::led;
use core::marker::PhantomData;
#[repr(C)]
pub struct ContextFrame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub return_addr: u32,
    pub xpsr: u32,
}

pub struct Process<'a> {
    pub sp: *mut u8,
    pub regs: [u32; 8],
    pub id: u8,
    marker: PhantomData<&'a u8>,
}

impl<'a> Process<'a> {
    pub fn new(stack: &'a mut [u8], app_main: extern "C" fn() -> !, id: u8) -> Self {
        let sp = (&stack[0] as *const u8 as usize) + stack.len() - 0x20;
        let context_frame: &mut ContextFrame = unsafe { &mut *(sp as *mut ContextFrame) };
        context_frame.r0 = 0;
        context_frame.r1 = 0;
        context_frame.r3 = 0;
        context_frame.r12 = 0;
        context_frame.lr = 0;
        context_frame.return_addr = app_main as u32;
        context_frame.xpsr = 0x0100_0000;

        Process {
            sp: sp as *mut u8,
            regs: [0; 8],
            id,
            marker: PhantomData,
        }
    }

    pub fn exec(&mut self) {
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
    pub fn led(&mut self) {
        led::init();
        led::turn_on();
    }
}
