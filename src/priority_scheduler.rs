use crate::led;
use crate::lib1::StackFrame;
use crate::linked_lists::{LinkedList, ListItem};
use crate::mpu;
use crate::process::Process;
use crate::syscall::SYSCALL_FIRED;
use crate::syscall_id;
use cortex_m_semihosting::hprintln;

pub struct Scheduler<'a> {
    list1: LinkedList<'a, Process<'a>>,
    list2: LinkedList<'a, Process<'a>>,
    list3: LinkedList<'a, Process<'a>>,
    current_list_num: u32,
    lists: [LinkedList<'a, Process<'a>>; 3],
}
impl<'a> Scheduler<'a> {
    pub fn new() -> Self {
        Scheduler {
            list1: LinkedList::new(1),
            list2: LinkedList::new(2),
            list3: LinkedList::new(3),
            lists: [LinkedList::new(1), LinkedList::new(2), LinkedList::new(3)],
            current_list_num: 1,
        }
    }

    pub fn push(&mut self, item: &'a mut ListItem<'a, Process<'a>>) {
        match item.priority {
            1 => self.list1.push(item),
            2 => self.list2.push(item),
            3 => self.list3.push(item),
            _ => {}
        }
        //self.lists[item.priority as usize].push(item);
    }

    fn schedule_next(&mut self) {
        match self.current_list_num {
            1 => {
                let current = self.list1.pop().unwrap();
                self.list1.push(current);
            }
            2 => {
                let current = self.list2.pop().unwrap();
                self.list2.push(current);
            }
            3 => {
                let current = self.list3.pop().unwrap();
                self.list3.push(current);
            }
            _ => {}
        }
        //let current = self.lists[self.current_list_num as usize].pop().unwrap();
        //self.lists[self.current_list_num as usize].push(current);
    }

    pub fn exec(&mut self) -> ! {
        loop {
            let current = match self.current_list_num {
                1 => self.list1.head_mut(),
                2 => self.list2.head_mut(),
                3 => self.list3.head_mut(),
                _ => self.list1.head_mut(),
            };
            //let current = self.lists[self.current_list_num as usize].head_mut();
            if current.is_none() {
                unimplemented!();
            }
            let mut syscall: Option<*const u32> = None;
            current.map(|p| {
                //privirage_task();
                hprintln!("p:exec_b").unwrap();
                match p.id {
                    1 => mpu::stack_protect_thread1(),
                    2 => mpu::stack_protect_thread2(),
                    3 => mpu::stack_protect_thread3(),
                    _ => {}
                }
                p.exec();
                unsafe {
                    hprintln!("svc {}", SYSCALL_FIRED);
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
                        syscall_id::THRED_WAIT => {
                            todo!()
                        }
                        syscall_id::THRED_SLEEP => {
                            todo!()
                        }
                        syscall_id::THRED_WAKEUP => {
                            todo!()
                        }
                        syscall_id::THREAD_CHPRI => {
                            todo!()
                        }
                        _ => {
                            self.schedule_next();
                            for i in 1..=3 {
                                if i == 1 {
                                    if self.list1.head_mut().is_some() {
                                        self.current_list_num = 1;
                                        hprintln!("setp_1").unwrap();
                                        //mpu::stack_protect_test2();
                                        hprintln!("s").unwrap();
                                        break;
                                    }
                                } else if i == 2 {
                                    if self.list2.head_mut().is_some() {
                                        self.current_list_num = 2;
                                        break;
                                    }
                                } else if i == 3 {
                                    if self.list3.head_mut().is_some() {
                                        self.current_list_num = 3;
                                        break;
                                    }
                                }
                            }
                            hprintln!("current_list_num={}", self.current_list_num).unwrap();
                        }
                    }
                }
                None => {}
            }
            //self.schedule_next();
        }
    }
}
