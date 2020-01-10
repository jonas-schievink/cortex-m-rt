# TODO: Handle ARMv6 / ARMv7 / ARMv8 differences and do possible thumbv2 optimizations

  # LLD requires that the section flags are explicitly set here
  .section .trampoline, "ax"
  .global FaultTrampoline
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type FaultTrampoline,%function
  .thumb_func
FaultTrampoline:
  # IPSR contains the exception number we're currently handling.
  mrs r1, IPSR
  # r1 is now the exception number from 1 (Reset) to N (depending on the target
  # architecture). The first exception we're interested in is 2 (NMI), which
  # gets index 0 in the table, so subtract 2. We'll keep the raw index in r1 to
  # pass it as an argument to the actual handler.
  sub r2, r1, #2
  # Convert the index into an offset into the adjusted vector table, which
  # contains 4-Byte entries.
  lsl r0, r2, #2
  # Now load the address of the actual exception handler into r2.
  ldr r2, =__TRAMPOLINED_EXCEPTIONS
  add r2, r2, r0
  ldr r2, [r2]

  # Depending on the stack mode in EXC_RETURN, fetch stack pointer from
  # PSP or MSP.
  mov r0, lr
  mov r3, #4
  # If bit 0b0100 is set, use PSP.
  tst r0, r3
  bne 0f
  mrs r0, MSP
  bx r2
0:
  mrs r0, PSP
  bx r2

# Future plan: armv6 only has NMI and HardFault faults, so duplicating the
# trampoline two times might be smaller. We could also just allow
# `&ExceptionFrame` for `HardFault`, not NMI.
# armv7 and v8 can definitely benefit from some optimizations here, since we
# move a lot of stuff around that doesn't really need to be (only on 16-bit
# thumb).
