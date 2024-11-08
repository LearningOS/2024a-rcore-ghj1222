//! Types related to task management & Functions for completely changing TCB

use super::id::TaskUserRes;
use super::{kstack_alloc, KernelStack, ProcessControlBlock, TaskContext};
use crate::trap::TrapContext;
use crate::{mm::PhysPageNum, sync::UPSafeCell};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cell::RefMut;

/// Task control block structure
pub struct TaskControlBlock {
    /// immutable
    pub process: Weak<ProcessControlBlock>,
    /// Kernel stack corresponding to PID
    pub kstack: KernelStack,
    /// mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

impl TaskControlBlock {
    /// Get the mutable reference of the inner TCB
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
    /// Get the address of app's page table
    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.memory_set.token()
    }

    /// copy from kernel space to user space
    pub fn copy_out<T>(&self, data: &T, addr: *mut T) -> Result<(), ()> {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.memory_set.copy_out(data, addr)
    }

    /// set waiting mutex
    pub fn set_waiting_mutex(&self, mutex_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.waiting_mutex = Some(mutex_id);
    }

    /// clear waiting mutex
    pub fn clear_waiting_mutex(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.waiting_mutex = None;
    }

    /// set having mutex
    pub fn set_having_mutex(&self, mutex_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.waiting_mutex = None;
        while inner.having_mutex.len() <= mutex_id {
            inner.having_mutex.push(false);
        }
        inner.having_mutex[mutex_id] = true;
    }

    /// release mutex
    pub fn release_mutex(&self, mutex_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.having_mutex[mutex_id] = false;
    }

    /// set waiting semaphore
    pub fn set_waiting_semaphore(&self, semaphore_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.waiting_semaphore = Some(semaphore_id);
    }

    /// clear waiting semaphore
    pub fn clear_waiting_semaphore(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.waiting_semaphore = None;
    }

    /// set having semaphore
    pub fn set_having_semaphore(&self, semaphore_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.waiting_semaphore = None;
        while inner.having_semaphore.len() <= semaphore_id {
            inner.having_semaphore.push(0);
        }
        inner.having_semaphore[semaphore_id] += 1;
    }

    /// release semaphore
    pub fn release_semaphore(&self, semaphore_id: usize) {
        let mut inner = self.inner.exclusive_access();
        while inner.having_semaphore.len() <= semaphore_id {
            inner.having_semaphore.push(0);
        }
        inner.having_semaphore[semaphore_id] -= 1;
        if inner.having_semaphore[semaphore_id] < 0 {
            inner.having_semaphore[semaphore_id] = 0;
        }
    }
}

pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    /// The physical page number of the frame where the trap context is placed
    pub trap_cx_ppn: PhysPageNum,
    /// Save task context
    pub task_cx: TaskContext,

    /// Maintain the execution status of the current process
    pub task_status: TaskStatus,
    /// It is set when active exit or execution error occurs
    pub exit_code: Option<i32>,
    /// mutex is having
    pub having_mutex: Vec<bool>,
    /// semaphore_having_counter
    pub having_semaphore: Vec<isize>,
    /// mutex is waiting
    pub waiting_mutex: Option<usize>,
    /// semaphore is waiting
    pub waiting_semaphore: Option<usize>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
}

impl TaskControlBlock {
    /// Create a new task
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    res: Some(res),
                    trap_cx_ppn,
                    task_cx: TaskContext::goto_trap_return(kstack_top),
                    task_status: TaskStatus::Ready,
                    exit_code: None,
                    having_mutex: Vec::new(),
                    having_semaphore: Vec::new(),
                    waiting_semaphore: None,
                    waiting_mutex: None,
                })
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// The execution status of the current process
pub enum TaskStatus {
    /// ready to run
    Ready,
    /// running
    Running,
    /// blocked
    Blocked,
}
