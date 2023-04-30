.syntax unified
.cpu cortex-a8
.globl _start

.text
_start:
bl #fun1
# stop the vm
bkpt

fun1:
mov r0, #1
bx lr
nop
