#include <stdio.h>
#include <unistd.h>

long test_sys_2333()
{
    return syscall(2333);
}

int main()
{
    printf("test_sys_2333 result = %d\n", test_sys_2333());
}