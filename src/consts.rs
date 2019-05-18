pub const SYSTRACE_TRACEE_PRELOAD: &'static str = "SYSTRACE_TRACEE_PRELOAD";

pub const SYSTRACE_ENV_TOOL_LOG_KEY: &'static str = "TOOL_LOG";

pub const SYSCALL_INSN_SIZE: usize = 2;
pub const SYSCALL_INSN_MASK: u64 = 0xffff;
pub const SYSCALL_INSN: u64 = 0x050f;

pub const SYSTRACE_PRIVATE_PAGE_OFFSET: u64 = 0x7000_0000;
pub const SYSTRACE_PRIVATE_PAGE_SIZE: u64 = 0x4000;

pub const SYSTRACE_GLOBAL_STATE_FILE: &'static str = "systrace";
pub const SYSTRACE_GLOBAL_STATE_ADDR: u64 = 0x7020_0000;
pub const SYSTRACE_GLOBAL_STATE_SIZE: u64 = 0x1000;
pub const SYSTRACE_GLOBAL_STATE_FD: i32 = 1023;

pub const SYSTRACE_LOCAL_BASE: u64 = SYSTRACE_PRIVATE_PAGE_OFFSET + 0x1000;

pub const SYSTRACE_LOCAL_SYSCALL_HOOK_SIZE: u64 = SYSTRACE_LOCAL_BASE + 0x0;
pub const SYSTRACE_LOCAL_SYSCALL_HOOK_ADDR: u64 =
    SYSTRACE_LOCAL_SYSCALL_HOOK_SIZE + std::mem::size_of::<u64>() as u64;

pub const SYSTRACE_LOCAL_STUB_SCRATCH: u64 = SYSTRACE_LOCAL_SYSCALL_HOOK_ADDR + std::mem::size_of::<u64>() as u64;
pub const SYSTRACE_LOCAL_STACK_NESTING_LEVEL: u64 =
    SYSTRACE_LOCAL_STUB_SCRATCH + std::mem::size_of::<u64>() as u64;

pub const SYSTRACE_LOCAL_SYSCALL_TRAMPOLINE: u64 =
    SYSTRACE_LOCAL_STACK_NESTING_LEVEL + std::mem::size_of::<u64>() as u64;
pub const SYSTRACE_LOCAL_SYSTOOL_HOOK: u64 =
    SYSTRACE_LOCAL_SYSCALL_TRAMPOLINE + std::mem::size_of::<u64>() as u64;
pub const SYSTRACE_LOCAL_SYSCALL_PATCH_LOCK: u64 =
    SYSTRACE_LOCAL_SYSTOOL_HOOK + std::mem::size_of::<u64>() as u64;

pub const SYSTRACE_LOCAL_SYSTOOL_LOG_LEVEL: u64 =
    SYSTRACE_LOCAL_SYSCALL_PATCH_LOCK + std::mem::size_of::<u64>() as u64;

pub const SYSTRACE_LOCAL_SYSTRACE_LOCAL_STATE: u64 =
    SYSTRACE_LOCAL_SYSTOOL_LOG_LEVEL + std::mem::size_of::<u64>() as u64;

pub const SYSTRACE_LOCAL_SYSCALL_HELPER: u64 =
    SYSTRACE_LOCAL_SYSTRACE_LOCAL_STATE + std::mem::size_of::<u64>() as u64;

#[test]
fn det_tls_sanity_check() {
    assert_eq!(SYSTRACE_LOCAL_SYSCALL_HOOK_SIZE, SYSTRACE_LOCAL_BASE + 0);
    assert_eq!(SYSTRACE_LOCAL_SYSCALL_HOOK_ADDR, SYSTRACE_LOCAL_BASE + 8);
    assert_eq!(SYSTRACE_LOCAL_STUB_SCRATCH, SYSTRACE_LOCAL_BASE + 16);
    assert_eq!(SYSTRACE_LOCAL_STACK_NESTING_LEVEL, SYSTRACE_LOCAL_BASE + 24);
    assert_eq!(SYSTRACE_LOCAL_SYSCALL_TRAMPOLINE, SYSTRACE_LOCAL_BASE + 32);
    assert_eq!(SYSTRACE_LOCAL_SYSTOOL_HOOK, SYSTRACE_LOCAL_BASE + 40);
    assert_eq!(SYSTRACE_LOCAL_SYSCALL_PATCH_LOCK, SYSTRACE_LOCAL_BASE + 48);
    assert_eq!(SYSTRACE_LOCAL_SYSTOOL_LOG_LEVEL, SYSTRACE_LOCAL_BASE + 56);
    assert_eq!(SYSTRACE_LOCAL_SYSTRACE_LOCAL_STATE, SYSTRACE_LOCAL_BASE + 64);
    assert_eq!(SYSTRACE_LOCAL_SYSCALL_HELPER, SYSTRACE_LOCAL_BASE + 72);
}
