.globl __start

.text
__start:
jal fun1
# stop the vm
syscall

fun1:
xor $v0, $v0, $v0
addi $v0, 1
jr $ra
