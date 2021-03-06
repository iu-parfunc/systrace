/*
 * Copyright (c) 2013 Mozilla Foundation
 * Copyright 2015 VMware, Inc
 * Copyright 2015 Google Inc.
 *
 * Contributors: Albert Noll <noll.albert@gmail.com>, Thomas Anderegg <thomas@tanderegg.com>, Nimrod Partush <nimrodpar@gmail.com>
 *               Andrew Walton <awalton@vmware.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

/*
 * Copyright 2002 Niels Provos <provos@citi.umich.edu>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

        .text
        .globl _raw_syscall
        .hidden _raw_syscall
        .type _raw_syscall, @function
_raw_syscall:
        .cfi_startproc
        /* Incoming args are in %rdi, %rsi, %rdx, %rcx, %r8, %r9, and 8(%rsp).
	       Syscall arguments are %rdi, %rsi, %rdx, %r10, %r8, %r9. */
        movq %rdi, %rax         /* syscall number */
        movq %rsi, %rdi         /* first syscall arg */
        movq %rdx, %rsi         /* second syscall arg */
        movq %rcx, %rdx         /* third syscall arg */
        movq %r8, %r10          /* fourth syscall arg */
        movq %r9, %r8           /* fifth syscall arg */
        movq 8(%rsp), %r9       /* sixth syscall arg */

        pushq 32(%rsp)
        .cfi_adjust_cfa_offset 8
        pushq 32(%rsp)
        .cfi_adjust_cfa_offset 8

        /* During a system call the kernel makes some user-space-visible
           register changes:
           a) on entry, %r11 is set to %rflags
           b) %rcx is sometimes set to -1 (perhaps because of something rr does)
           c) on entry or exit, some flags are sometimes changed
           Also, during replay we may perform single-stepping which can set
           TF (trace flag). We need to hide this.

           fixup_syscall_registers is responsible for fixing up registers
           to hide these effects when we get a ptrace trap from system calls
           in the kernel: it clears TF from %r11, forces %rcx to -1, and sets
           flags to fixed values (ZF+PF+IF+reserved, same as for "xor reg,reg").
           Task::canonicalize_and_set_regs is responsible for fixing up registers
           when we emulate a system call that was traced during recording (by
           running to a breakpoint at that system call). It does the above
           effects after setting %r11 to %rflags.

           For untraced system calls there is no trap to rr during recording or
           replay, so we must handle these issues here. We do not need
           untraced system calls to behave exactly the same as traced
           system calls, since whether a given system call was traced or not is
           the same whether recording or replaying, but it's a good idea to
           make them as similar as possible. We do need register values
           to be perfectly consistent at every instruction in every replay
           whether or not singlestepping is used (because a ReplayTimeline::mark
           might be created at any point). During replay, untraced syscall
           instructions are replaced with "xor %eax,%eax".

           The following code is harmless for traced syscalls (and needs to be,
           because traced syscalls go through here too).
         */

        /* Set %r11 and %rcx to the values we expect them to have after the
           system call.
           Set flags to ZF+PF+IF+reserved (0x246) first. This simplifies
           everything.
           This all has to be independent of TF being set at any point during
           replay! But the way we're doing it here, it's trivial.
         */
        xor %ecx,%ecx
        /* At this point, flags are 0x246 + possibly TF. */
        movq $0x246,%r11
        movq $-1,%rcx

        callq *32(%rsp)

        /* At this point, during recording we don't trust the kernel to have
           restored flags correctly. It probably doesn't matter, but fix it
           anyway. */
        xor %ecx,%ecx
        /* At this point, the high 32 bits of %rcx are unknown. Fix that by
           setting to -1 to match traced syscalls. */
        movq $-1,%rcx
        /* At this point, %r11 is always 0x246 during replay and during
           recording (because TF is never set during recording). Nothing to
           fix in %r11. */

        addq $16,%rsp
        .cfi_adjust_cfa_offset -16
        ret
        .cfi_endproc
        .size _raw_syscall, . - _raw_syscall

        .section .note.GNU-stack,"",@progbits
        .previous
