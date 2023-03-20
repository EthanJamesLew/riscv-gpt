.section .text
.globl _start

_start:
    # Initialize variables
    li a0, 0   # n = 0
    li a1, 1   # a = 1
    li a2, 1   # b = 1
    li a3, 10  # count = 10

fib_loop:
    # Check if we've printed enough numbers
    beqz a3, exit

    # Print current Fibonacci number
    mv a0, a1

    # Compute next Fibonacci number
    add t0, a1, a2
    mv a1, a2
    mv a2, t0

    # Decrement count
    #addi a3, a3, -1

    # Loop back to fib_loop
    j fib_loop

exit:
    # Exit program
    li a0, 10
    ecall
