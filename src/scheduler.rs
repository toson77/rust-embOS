use crate::led;
use crate::lib1::StackFrame;
use crate::linked_list::{LinkedList, ListItem};
use crate::process::Process;
use crate::syscall::SYSCALL_FIRED;
use crate::syscall_id;
use cortex_m_semihosting::hprintln;
pub struct Scheduler<'a> {
    list: LinkedList<'a, Process<'a>>,
}
impl<'a> Scheduler<'a> {
    pub fn new() -> Self {
        Scheduler {
            list: LinkedList::new(),
        }
    }

    pub fn push(&mut self, item: &'a mut ListItem<'a, Process<'a>>) {
        self.list.push(item);
    }

    fn schedule_next(&mut self) {
        let current = self.list.pop().unwrap();
        self.list.push(current);
    }

    pub fn exec(&mut self) -> ! {
        loop {
            let current = self.list.head_mut();
            if current.is_none() {
                unimplemented!();
            }
            let mut syscall: Option<*const u32> = None;
            current.map(|p| {
                //privirage_task();
                p.exec();
                unsafe {
                    //hprintln!("svc {}", SYSCALL_FIRED);
                };
                unsafe { syscall.replace(p.sp as *const u32) };
            });
            match syscall {
                Some(sp) => {
                    let base_frame = unsafe { StackFrame::from_ptr_mut(sp) };
                    //hprintln!("r0 {}", base_frame.r0).unwrap();
                    let svc_id = base_frame.r0;
                    match svc_id {
                        syscall_id::LED_ON => {
                            //hprintln!("svc:led_on").unwrap();
                            led::init();
                            led::turn_on();
                        }
                        syscall_id::LED_OFF => {
                            led::turn_off();
                        }
                        _ => {
                            self.schedule_next();
                        }
                    }
                }
                None => {}
            }
            //self.schedule_next();
        }
    }
}
