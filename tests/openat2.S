/**
 * openat2.S: create syscalls sequence that cannot be patchable
 */
	.text
	.global main
	.type main, @function
main:
	sub $0x8, %rsp
	movq $0x6d6f646e, %rax
	push %rax
	movabs $0x6172752f7665642f, %rax
	push %rax
	mov $0x101, %eax
	mov $0xffffff9c, %rdi
	mov %rsp, %rsi
	mov $0x0, %rdx
	syscall
	mov $0xe7, %eax
	mov $0, %rdi
	syscall
