#![feature(asm)]
#![no_main]
#![no_std]
#![feature(naked_functions)]
#![feature(llvm_asm)]
mod led;
mod lib1;
mod linked_list;
mod linked_lists;
mod mpu;
mod priority_scheduler;
mod process;
mod scheduler;
mod svc;
mod syscall;
mod syscall_id;
mod systick;
use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::hprintln;
//use linked_list::ListItem;
use linked_lists::ListItem;
use process::ContextFrame;
use process::Process;
//use scheduler::Scheduler;
use priority_scheduler::Scheduler;
use syscall::SYSCALL_FIRED;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
    }
    // bss init zero
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    // copy .data from rom to ram
    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    hprintln!("Hello World").unwrap();

    //systick::init();
    mpu::init();

    #[link_section = ".app_stack"]
    pub static mut APP_STACK: [u8; 2048] = [0; 2048];
    let sp = (&APP_STACK[0] as *const u8 as usize) + APP_STACK.len() - 0x20;
    #[link_section = ".app_stack"]
    static mut APP_STACK2: [u8; 2048] = [0; 2048];
    #[link_section = ".app_stack"]
    static mut APP_STACK3: [u8; 2048] = [0; 2048];
    hprintln!("app_stack1_ptr={:p}", &APP_STACK[0]);
    hprintln!("app_stack1_sp={:x}", sp);
    hprintln!("app_stack1_end={:x}", sp + 0x20);
    hprintln!("app_stack2_ptr={:p}", &APP_STACK2[0]);
    hprintln!("app_stack3_ptr={:p}", &APP_STACK3[0]);
    let mut process1 = Process::new(&mut APP_STACK, app_main, 1);
    let mut item1 = ListItem::new(process1, 1, 1);
    let process2 = Process::new(&mut APP_STACK2, app_main2, 2);
    let mut item2 = ListItem::new(process2, 1, 2);
    let process3 = Process::new(&mut APP_STACK3, app_main3, 3);
    let mut item3 = ListItem::new(process3, 3, 3);
    let mut sched = Scheduler::new();
    sched.push(&mut item1);
    sched.push(&mut item2);
    sched.push(&mut item3);
    sched.exec();

    hprintln!("Kernel").unwrap();
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    //fn HardFault();
    //fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn PendSV();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    hprintln!("DefaultExceptionHandler").unwrap();
    loop {}
}

#[no_mangle]
pub extern "C" fn MemManage() {
    hprintln!("MemManage").unwrap();
    loop {}
}
#[no_mangle]
pub extern "C" fn HardFault() {
    hprintln!("HardFault").unwrap();
    loop {}
}

#[no_mangle]
pub extern "C" fn SysTick() {
    hprintln!("Systick").unwrap();
}
#[no_mangle]
#[naked]
pub unsafe extern "C" fn SVCall() {
    /*
    llvm_asm!(
        "
        cmp lr, #0xfffffff9
        bne to_kernel
        mov r0, #1
        msr CONTROL, r0
        movw lr, #0xfffd
        movt lr, #0xffff
        bx lr

        to_kernel:
        mov r0, #0
        msr CONTROL, r0
        movw lr, #0xfff9
        movt lr, #0xffff
        bx lr
        "
        ::::"volatile"
    );
    */
    asm!(
        "cmp lr, #0xfffffff9",
        "bne 1f",
        /* switch thread mode to unprivileged */
        "mov r0, #1",
        "msr CONTROL, r0",
        "movw lr, #0xfffd",
        "movt lr, #0xffff",
        "bx lr",
        "1:",
        "mov r0, #0",
        "msr CONTROL, r0",
        "ldr r0, =SYSCALL_FIRED",
        "mov r1, #1",
        "str r1, [r0, #0]",
        "movw lr, #0xfff9",
        "movt lr, #0xffff",
        "bx lr",
        options(noreturn),
    );
}

extern "C" fn app_main() -> ! {
    hprintln!("App1").unwrap();
    let mut num: u8 = 1;
    //let mut test: [u8; 1100] = [10; 1100];
    //let mut test: [u8; 1100] = [10; 1100];
    //hprintln!("test={:p}", &test).unwrap();
    let mut num2: u8 = 1;
    let num_ptr: *const u8 = &num2;
    hprintln!("{:p}", &num).unwrap();
    hprintln!("{:p}", &num2).unwrap();
    //hprintln!("test_end={:p}", &test[999]).unwrap();
    loop {
        hprintln!("App1").unwrap();
        //let mut test: [u8; 100] = [10; 100];
        //led::init();
        //led::turn_on();
        //svc::switch_led();
        hprintln!("led_on").unwrap();
        syscall::led_on();
        hprintln!("after_syscall").unwrap();
        hprintln!("app1_num={}", num).unwrap();
        num += 1;
        call_svc();
        hprintln!("after_call_svc").unwrap();
    }
}
extern "C" fn app_main2() -> ! {
    let mut num: u8 = 1;
    hprintln!("{:p}", &num).unwrap();
    loop {
        hprintln!("App2").unwrap();
        hprintln!("led_off").unwrap();
        syscall::led_off();
        hprintln!("after_syscall").unwrap();
        hprintln!("app2_num={}", num).unwrap();
        num += 1;
        call_svc();
    }
}
extern "C" fn app_main3() -> ! {
    loop {
        hprintln!("App3").unwrap();
        call_svc();
    }
}

fn call_svc() {
    unsafe {
        asm!("svc 0",
        in("r0") 0,
            );
    }
}
