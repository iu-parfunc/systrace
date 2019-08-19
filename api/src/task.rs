//! task structure and traits

use nix::sys::wait::WaitStatus;
use nix::sys::{ptrace, signal, uio, wait};
use nix::unistd::Pid;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::ptr::NonNull;

use core::pin::Pin;
use futures::prelude::Future;
use futures::task::{Poll, Context};
use futures::task::Poll::*;

use syscalls::SyscallNo;

use crate::remote::Injector;

pub type EventHandler = Box<dyn FnMut(&dyn Task) -> Result<()>>;

pub trait TaskEventHandler {
    fn new_event_handler(
        on_exec: EventHandler,
        on_fork: EventHandler,
        on_clone: EventHandler,
        on_exit: EventHandler,
    ) -> Self;
}

pub struct TaskEventCB {
    pub on_task_exec: Box<dyn FnMut(&dyn Task) -> Result<()>>,
    pub on_task_fork: Box<dyn FnMut(&dyn Task) -> Result<()>>,
    pub on_task_clone: Box<dyn FnMut(&dyn Task) -> Result<()>>,
    pub on_task_exit: Box<dyn FnMut(&dyn Task) -> Result<()>>,
}

pub trait GlobalState {
    fn new() -> Self
    where
        Self: Sized;
}

pub trait ProcessState: Task + Injector {
    fn new(pid: Pid) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Stopped(signal::Signal),
    Signaled(signal::Signal),
    Exec,
    Clone(Pid),
    Fork(Pid),
    Seccomp(SyscallNo),
    Syscall, // XXX: internal only
    Exited(i32),
}

/// Task which can be scheduled by `Sched`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunTask<Task> {
    /// `Task` Exited with an exit code
    Exited(i32),
    /// `Task` can be scheduled
    Runnable(Task),
    /// Blocked `Task`
    Blocked(Task),
    /// A task tuple `(prent, child)` returned from `fork`/`vfork`/`clone`
    Forked(Task, Task),
}

/*
impl <Task> futures::Future for RunTask<Task>
    where Task: std::marker::Unpin
{
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match Pin::into_inner(self) {
            RunTask::Blocked(_) => Pending,
            _                     => Ready(()),
        }
        
    }
}
*/

pub trait Task {
    fn new(pid: Pid) -> Self
    where
        Self: Sized;
    fn cloned(&self, child: Pid) -> Self
    where
        Self: Sized;
    fn forked(&self, child: Pid) -> Self
    where
        Self: Sized;
    fn getpid(&self) -> Pid;
    fn gettid(&self) -> Pid;
    fn getppid(&self) -> Pid;
    fn getpgid(&self) -> Pid;
    fn exited(&self, code: i32) -> Option<i32>;
}

pub trait Runnable<G>
where
    G: GlobalState,
{
    type Item;
    /// take ownership of `self`
    fn run(self, glob: &mut G) -> Pin<Box<dyn Future<Output = RunTask<Self::Item>>>>;
}
