.syntax unified
.cpu cortex-a8
.globl _start

.text
_start:
adr r0, #fun1
adr lr, #end
bx r0
end:
# stop the vm
bkpt

fun1:
mov r0, #1
bx lr
nop
