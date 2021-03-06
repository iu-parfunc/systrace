/* must build with flags -pie -fPIE -O */


#include <sys/types.h>
#include <unistd.h>
#include <stdio.h>
#include <assert.h>

/**
 * 0000000000000740 <sys_getpid>:
 * 740:   b8 27 00 00 00          mov    $0x27,%eax
 * 745:   0f 05                   syscall
 * 747:   c3                      retq
 * 748:   0f 1f 84 00 00 00 00    nopl   0x0(%rax,%rax,1)
 * 74f:   00
 */
__attribute__((noinline)) static int sys_getpid(void) {
  int ret;
  asm volatile  ("mov $0x27, %%eax\n\t"
       "syscall\n\t"
       : "=r"(ret));
  return ret;
}

int main(int argc, char* argv[])
{
  int pid0 = getpid();
  int pid = sys_getpid();
  printf("pid = %d\n", pid);
  assert(pid0 == pid);

  return 0;
}
