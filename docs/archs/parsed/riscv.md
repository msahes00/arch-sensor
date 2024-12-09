---
tags: assembly, riscv
endianness: [little]
instruction:
  len: fixed
  bits: [16, 32]
---
* [code:: sret] Trap return from supervisor mode.
* [code:: mret] Trap return from machine mode.
* [code:: mnret] Trap return from restartable non-maskable interrupt (RNMI).
* [code:: wfi] Wait for interrupt or event that may cause an interrupt.
* [code:: sfence.vma] Synchronize the supervisor address translation cache.
* [code:: hfence.vvma] Synchronize the hypervisor address translation cache for VS stage address translation.
* [code:: hfence.gvma] Synchronize the hypervisor address translation cache for G stage address translation.
* [code:: hlv.b] Load as byte from hypervisor memory.
* [code:: hlv.bu] Load as unsigned byte from hypervisor memory.
* [code:: hlv.h] Load as halfword from hypervisor memory.
* [code:: hlv.hu] Load as unsigned halfword from hypervisor memory.
* [code:: hlv.w] Load as word from hypervisor memory.
* [code:: hlvx.hu] Load as indexed unsigned halfword from hypervisor memory.
* [code:: hlvx.wu] Load as indexed unsigned word from hypervisor memory.
* [code:: hsv.b] Store as byte in hypervisor memory.
* [code:: hsv.h] Store as halfword in hypervisor memory.
* [code:: hsv.w] Store as word in hypervisor memory.
* [code:: hlv.wu] Load as unsigned word from hypervisor memory.
* [code:: hlv.d] Load as doubleword from hypervisor memory.
* [code:: hsv.d] Store as doubleword in hypervisor memory.
* [code:: sinval.vma] Invalidate supervisor address translation cache entries.
* [code:: sfence.w.inval] Synchronize invalidation of supervisor address translation cache entries with stores to supervisor memory.
* [code:: sfence.inval.ir] Synchronize invalidation of supervisor address translation cache entries with loads from supervisor memory.
* [code:: hinval.vvma] Invalidate virtual supervisor address translation cache entries.
* [code:: hinval.gvma] Invalidate of guest address translation cache entries.
* [code:: lui] Load Upper Immediate: Loads a 20-bit immediate to the upper 20 bits of a register.
* [code:: auipc] Add Upper Immediate to PC: Adds a 20-bit immediate to the PC and stores in a register.
* [code:: jal] Jump and Link: Jumps to a target address and stores return address in a register.
* [code:: jalr] Jump and Link Register: Jumps to an address in a register plus offset.
* [code:: beq] Branch if Equal: Branches if two registers are equal.
* [code:: bne] Branch if Not Equal: Branches if two registers are not equal.
* [code:: blt] Branch if Less Than: Branches if the first register is less than the second.
* [code:: bge] Branch if Greater or Equal: Branches if the first register is greater than or equal to the second.
* [code:: bltu] Branch if Less Than Unsigned: Branches if the first register is less than the second (unsigned).
* [code:: bgeu] Branch if Greater or Equal Unsigned: Branches if the first register is greater than or equal (unsigned).
* [code:: lb] Load Byte: Loads a signed byte from memory.
* [code:: lh] Load Halfword: Loads a signed 16-bit value from memory.
* [code:: lw] Load Word: Loads a signed 32-bit word from memory.
* [code:: lbu] Load Byte Unsigned: Loads an unsigned byte from memory.
* [code:: lhu] Load Halfword Unsigned: Loads an unsigned 16-bit value from memory.
* [code:: sb] Store Byte: Stores the least-significant byte of a register to memory.
* [code:: sh] Store Halfword: Stores the least-significant 16 bits of a register to memory.
* [code:: sw] Store Word: Stores a 32-bit value from a register to memory.
* [code:: addi] Add Immediate: Adds a sign-extended immediate to a register.
* [code:: slti] Set Less Than Immediate: Sets a register if less than the immediate.
* [code:: sltiu] Set Less Than Immediate Unsigned: Sets a register if less than the immediate (unsigned).
* [code:: xori] XOR Immediate: XORs a register with an immediate.
* [code:: ori] OR Immediate: ORs a register with an immediate.
* [code:: andi] AND Immediate: ANDs a register with an immediate.
* [code:: slli] Shift Left Logical Immediate: Shifts a register left by a constant.
* [code:: srli] Shift Right Logical Immediate: Shifts a register right (logical) by a constant.
* [code:: srai] Shift Right Arithmetic Immediate: Shifts a register right (arithmetic) by a constant.
* [code:: add] Add: Adds two registers.
* [code:: sub] Subtract: Subtracts the second register from the first.
* [code:: sll] Shift Left Logical: Shifts the first register left by the number of bits in the second.
* [code:: slt] Set Less Than: Sets a register if less than the second register.
* [code:: sltu] Set Less Than Unsigned: Sets a register if less than the second (unsigned).
* [code:: xor] XOR: XORs two registers.
* [code:: srl] Shift Right Logical: Shifts the first register right (logical).
* [code:: sra] Shift Right Arithmetic: Shifts the first register right (arithmetic).
* [code:: or] OR: ORs two registers.
* [code:: and] AND: ANDs two registers.
* [code:: fence] Fence: Ensures memory ordering constraints are satisfied.
* [code:: fence.tso] Fence Total Store Order: Enforces strict memory order constraints.
* [code:: pause] Pause: Provides a hint for multithreaded synchronization.
* [code:: ecall] Environment Call: Triggers a call to the operating system or environment.
* [code:: ebreak] Environment Break: Causes a debugger breakpoint.
* [code:: lwu] Load Word Unsigned: Loads a 32-bit unsigned value from memory (RV64/RV128).
* [code:: ld] Load Doubleword: Loads a 64-bit value from memory (RV64/RV128).
* [code:: sd] Store Doubleword: Stores a 64-bit value from a register to memory (RV64/RV128).
* [code:: slli] Shift Left Logical Immediate: Shifts a register left by a constant (RV64/RV128).
* [code:: srli] Shift Right Logical Immediate: Shifts a register right (logical) by a constant (RV64/RV128).
* [code:: srai] Shift Right Arithmetic Immediate: Shifts a register right (arithmetic) by a constant (RV64/RV128).
* [code:: addiw] Add Immediate Word: Adds a sign-extended immediate to the lower 32 bits of a register (RV64).
* [code:: slliw] Shift Left Logical Immediate Word: Shifts the lower 32 bits left by a constant (RV64).
* [code:: srliw] Shift Right Logical Immediate Word: Shifts the lower 32 bits right (logical) by a constant (RV64).
* [code:: sraiw] Shift Right Arithmetic Immediate Word: Shifts the lower 32 bits right (arithmetic) by a constant (RV64).
* [code:: addw] Add Word: Adds the lower 32 bits of two registers (RV64).
* [code:: subw] Subtract Word: Subtracts the lower 32 bits of the second register from the first (RV64).
* [code:: sllw] Shift Left Logical Word: Shifts the lower 32 bits left by the number of bits in the second register (RV64).
* [code:: srlw] Shift Right Logical Word: Shifts the lower 32 bits right (logical) by the number of bits in the second register (RV64).
* [code:: sraw] Shift Right Arithmetic Word: Shifts the lower 32 bits right (arithmetic) by the number of bits in the second register (RV64).
* [code:: fence.i] Fence Instruction: Ensures the ordering of instruction fetches.
* [code:: csrrw] CSR Read and Write: Reads a CSR and writes a value from a register.
* [code:: csrrs] CSR Read and Set: Reads a CSR and sets bits using a register.
* [code:: csrrc] CSR Read and Clear: Reads a CSR and clears bits using a register.
* [code:: csrrwi] CSR Read and Write Immediate: Reads a CSR and writes a value from an immediate.
* [code:: csrrsi] CSR Read and Set Immediate: Reads a CSR and sets bits using an immediate.
* [code:: csrrci] CSR Read and Clear Immediate: Reads a CSR and clears bits using an immediate.
* [code:: mul] Multiply: Multiplies two registers and stores the result in the lower 32 bits.
* [code:: mulh] Multiply High Signed: Multiplies two signed registers and stores the upper 32 bits of the result.
* [code:: mulhsu] Multiply High Signed-Unsigned: Multiplies a signed and unsigned register, storing the upper 32 bits of the result.
* [code:: mulhu] Multiply High Unsigned: Multiplies two unsigned registers, storing the upper 32 bits of the result.
* [code:: div] Divide: Divides the first register by the second (signed).
* [code:: divu] Divide Unsigned: Divides the first register by the second (unsigned).
* [code:: rem] Remainder: Computes the signed remainder of a division.
* [code:: remu] Remainder Unsigned: Computes the unsigned remainder of a division.
* [code:: mulw] Multiply Word: Multiplies the lower 32 bits of two registers and stores the lower 32 bits (RV64).
* [code:: divw] Divide Word: Divides the lower 32 bits of the first register by the second (RV64).
* [code:: divuw] Divide Unsigned Word: Divides the lower 32 bits of the first register by the second (unsigned) (RV64).
* [code:: remw] Remainder Word: Computes the signed remainder of a division of the lower 32 bits (RV64).
* [code:: remuw] Remainder Unsigned Word: Computes the unsigned remainder of a division of the lower 32 bits (RV64).
* [code:: lr.w] Load Reserved Word: Performs a reservation-based atomic load (RV32).
* [code:: sc.w] Store Conditional Word: Atomically stores data if a reservation is valid (RV32).
* [code:: amoswap.w] Atomic Swap Word: Atomically swaps memory with a register value (RV32).
* [code:: amoadd.w] Atomic Add Word: Atomically adds a register value to memory (RV32).
* [code:: amoxor.w] Atomic XOR Word: Atomically XORs memory with a register value (RV32).
* [code:: amoand.w] Atomic AND Word: Atomically ANDs memory with a register value (RV32).
* [code:: amoor.w] Atomic OR Word: Atomically ORs memory with a register value (RV32).
* [code:: amomin.w] Atomic Min Word: Atomically updates memory with the minimum value (signed) (RV32).
* [code:: amomax.w] Atomic Max Word: Atomically updates memory with the maximum value (signed) (RV32).
* [code:: amominu.w] Atomic Min Unsigned Word: Atomically updates memory with the minimum value (unsigned) (RV32).
* [code:: amomaxu.w] Atomic Max Unsigned Word: Atomically updates memory with the maximum value (unsigned) (RV32).
* [code:: lr.d] Load Reserved Doubleword: Performs a reservation-based atomic load (RV64).
* [code:: sc.d] Store Conditional Doubleword: Atomically stores data if a reservation is valid (RV64).
* [code:: amoswap.d] Atomic Swap Doubleword: Atomically swaps memory with a register value (RV64).
* [code:: amoadd.d] Atomic Add Doubleword: Atomically adds a register value to memory (RV64).
* [code:: amoxor.d] Atomic XOR Doubleword: Atomically XORs memory with a register value (RV64).
* [code:: amoand.d] Atomic AND Doubleword: Atomically ANDs memory with a register value (RV64).
* [code:: amoor.d] Atomic OR Doubleword: Atomically ORs memory with a register value (RV64).
* [code:: amomin.d] Atomic Min Doubleword: Atomically updates memory with the minimum value (signed) (RV64).
* [code:: amomax.d] Atomic Max Doubleword: Atomically updates memory with the maximum value (signed) (RV64).
* [code:: amominu.d] Atomic Min Unsigned Doubleword: Atomically updates memory with the minimum value (unsigned) (RV64).
* [code:: amomaxu.d] Atomic Max Unsigned Doubleword: Atomically updates memory with the maximum value (unsigned) (RV64).
* [code:: flw] Load Floating-Point Word: Loads a single-precision floating-point value from memory.
* [code:: fsw] Store Floating-Point Word: Stores a single-precision floating-point value to memory.
* [code:: fmadd.s] Floating-Point Multiply-Add Single: Performs (a × b) + c with single-precision operands.
* [code:: fmsub.s] Floating-Point Multiply-Subtract Single: Performs (a × b) − c with single-precision operands.
* [code:: fnmsub.s] Floating-Point Negative Multiply-Subtract Single: Performs −(a × b) + c with single-precision operands.
* [code:: fnmadd.s] Floating-Point Negative Multiply-Add Single: Performs −(a × b) − c with single-precision operands.
* [code:: fadd.s] Floating-Point Add Single: Adds two single-precision floating-point values.
* [code:: fsub.s] Floating-Point Subtract Single: Subtracts one single-precision floating-point value from another.
* [code:: fmul.s] Floating-Point Multiply Single: Multiplies two single-precision floating-point values.
* [code:: fdiv.s] Floating-Point Divide Single: Divides one single-precision floating-point value by another.
* [code:: fsqrt.s] Floating-Point Square Root Single: Computes the square root of a single-precision floating-point value.
* [code:: fsgnj.s] Floating-Point Sign Inject Single: Copies the sign of one single-precision value to another.
* [code:: fsgnjn.s] Floating-Point Sign Negate Single: Copies the negated sign of one single-precision value to another.
* [code:: fsgnjx.s] Floating-Point Sign XOR Single: Copies the XORed sign of two single-precision values.
* [code:: fmin.s] Floating-Point Minimum Single: Returns the smaller of two single-precision values.
* [code:: fmax.s] Floating-Point Maximum Single: Returns the larger of two single-precision values.
* [code:: fcvt.w.s] Convert Single to Word: Converts a single-precision value to a signed 32-bit integer.
* [code:: fcvt.wu.s] Convert Single to Word Unsigned: Converts a single-precision value to an unsigned 32-bit integer.
* [code:: fmv.x.w] Move Float to Integer: Moves a single-precision value to an integer register.
* [code:: feq.s] Floating-Point Equal Single: Compares two single-precision values for equality.
* [code:: flt.s] Floating-Point Less Than Single: Compares two single-precision values for less-than.
* [code:: fle.s] Floating-Point Less or Equal Single: Compares two single-precision values for less-or-equal.
* [code:: fclass.s] Floating-Point Classify Single: Classifies a single-precision value (e.g., NaN, infinity).
* [code:: fcvt.s.w] Convert Word to Single: Converts a signed 32-bit integer to a single-precision value.
* [code:: fcvt.s.wu] Convert Word Unsigned to Single: Converts an unsigned 32-bit integer to a single-precision value.
* [code:: fmv.w.x] Move Integer to Float: Moves an integer register value to a single-precision register.
* [code:: fcvt.l.s] Convert Single to Long: Converts a single-precision value to a signed 64-bit integer.
* [code:: fcvt.lu.s] Convert Single to Long Unsigned: Converts a single-precision value to an unsigned 64-bit integer.
* [code:: fcvt.s.l] Convert Long to Single: Converts a signed 64-bit integer to a single-precision value.
* [code:: fcvt.s.lu] Convert Long Unsigned to Single: Converts an unsigned 64-bit integer to a single-precision value.
* [code:: fld] Load Floating-Point Double: Loads a double-precision floating-point value from memory.
* [code:: fsd] Store Floating-Point Double: Stores a double-precision floating-point value to memory.
* [code:: fmadd.d] Floating-Point Multiply-Add Double: Performs (a × b) + c with double-precision operands.
* [code:: fmsub.d] Floating-Point Multiply-Subtract Double: Performs (a × b) − c with double-precision operands.
* [code:: fnmsub.d] Floating-Point Negative Multiply-Subtract Double: Performs −(a × b) + c with double-precision operands.
* [code:: fnmadd.d] Floating-Point Negative Multiply-Add Double: Performs −(a × b) − c with double-precision operands.
* [code:: fadd.d] Floating-Point Add Double: Adds two double-precision floating-point values.
* [code:: fsub.d] Floating-Point Subtract Double: Subtracts one double-precision floating-point value from another.
* [code:: fmul.d] Floating-Point Multiply Double: Multiplies two double-precision floating-point values.
* [code:: fdiv.d] Floating-Point Divide Double: Divides one double-precision floating-point value by another.
* [code:: fsqrt.d] Floating-Point Square Root Double: Computes the square root of a double-precision floating-point value.
* [code:: fsgnj.d] Floating-Point Sign Inject Double: Copies the sign of one double-precision value to another.
* [code:: fsgnjn.d] Floating-Point Sign Negate Double: Copies the negated sign of one double-precision value to another.
* [code:: fsgnjx.d] Floating-Point Sign XOR Double: Copies the XORed sign of two double-precision values.
* [code:: fmin.d] Floating-Point Minimum Double: Returns the smaller of two double-precision values.
* [code:: fmax.d] Floating-Point Maximum Double: Returns the larger of two double-precision values.
* [code:: fcvt.s.d] Convert Double to Single: Converts a double-precision value to a single-precision value.
* [code:: fcvt.d.s] Convert Single to Double: Converts a single-precision value to a double-precision value.
* [code:: feq.d] Floating-Point Equal Double: Compares two double-precision values for equality.
* [code:: flt.d] Floating-Point Less Than Double: Compares two double-precision values for less-than.
* [code:: fle.d] Floating-Point Less or Equal Double: Compares two double-precision values for less-or-equal.
* [code:: fclass.d] Floating-Point Classify Double: Classifies a double-precision value (e.g., NaN, infinity).
* [code:: fcvt.w.d] Convert Double to Word: Converts a double-precision value to a signed 32-bit integer.
* [code:: fcvt.wu.d] Convert Double to Word Unsigned: Converts a double-precision value to an unsigned 32-bit integer.
* [code:: fcvt.d.w] Convert Word to Double: Converts a signed 32-bit integer to a double-precision value.
* [code:: fcvt.d.wu] Convert Word Unsigned to Double: Converts an unsigned 32-bit integer to a double-precision value.
* [code:: fcvt.l.d] Convert Double to Long: Converts a double-precision value to a signed 64-bit integer.
* [code:: fcvt.lu.d] Convert Double to Long Unsigned: Converts a double-precision value to an unsigned 64-bit integer.
* [code:: fmv.x.d] Move Double to Integer: Moves a double-precision value to an integer register.
* [code:: fcvt.d.l] Convert Long to Double: Converts a signed 64-bit integer to a double-precision value.
* [code:: fcvt.d.lu] Convert Long Unsigned to Double: Converts an unsigned 64-bit integer to a double-precision value.
* [code:: fmv.d.x] Move Integer to Double: Moves an integer register value to a double-precision register.
* [code:: flq] Load Floating-Point Quadword: Loads a quad-precision floating-point value from memory.
* [code:: fsq] Store Floating-Point Quadword: Stores a quad-precision floating-point value to memory.
* [code:: fmadd.q] Floating-Point Multiply-Add Quad: Performs (a × b) + c with quad-precision operands.
* [code:: fmsub.q] Floating-Point Multiply-Subtract Quad: Performs (a × b) − c with quad-precision operands.
* [code:: fnmsub.q] Floating-Point Negative Multiply-Subtract Quad: Performs −(a × b) + c with quad-precision operands.
* [code:: fnmadd.q] Floating-Point Negative Multiply-Add Quad: Performs −(a × b) − c with quad-precision operands.
* [code:: fadd.q] Floating-Point Add Quad: Adds two quad-precision floating-point values.
* [code:: fsub.q] Floating-Point Subtract Quad: Subtracts one quad-precision floating-point value from another.
* [code:: fmul.q] Floating-Point Multiply Quad: Multiplies two quad-precision floating-point values.
* [code:: fdiv.q] Floating-Point Divide Quad: Divides one quad-precision floating-point value by another.
* [code:: fsqrt.q] Floating-Point Square Root Quad: Computes the square root of a quad-precision floating-point value.
* [code:: fsgnj.q] Floating-Point Sign Inject Quad: Copies the sign of one quad-precision value to another.
* [code:: fsgnjn.q] Floating-Point Sign Negate Quad: Copies the negated sign of one quad-precision value to another.
* [code:: fsgnjx.q] Floating-Point Sign XOR Quad: Copies the XORed sign of two quad-precision values.
* [code:: fmin.q] Floating-Point Minimum Quad: Returns the smaller of two quad-precision values.
* [code:: fmax.q] Floating-Point Maximum Quad: Returns the larger of two quad-precision values.
* [code:: fcvt.s.q] Convert Quad to Single: Converts a quad-precision value to a single-precision value.
* [code:: fcvt.q.s] Convert Single to Quad: Converts a single-precision value to a quad-precision value.
* [code:: fcvt.d.q] Convert Quad to Double: Converts a quad-precision value to a double-precision value.
* [code:: fcvt.q.d] Convert Double to Quad: Converts a double-precision value to a quad-precision value.
* [code:: feq.q] Floating-Point Equal Quad: Compares two quad-precision values for equality.
* [code:: flt.q] Floating-Point Less Than Quad: Compares two quad-precision values for less-than.
* [code:: fle.q] Floating-Point Less or Equal Quad: Compares two quad-precision values for less-or-equal.
* [code:: fclass.q] Floating-Point Classify Quad: Classifies a quad-precision value (e.g., NaN, infinity).
* [code:: fcvt.w.q] Convert Quad to Word: Converts a quad-precision value to a signed 32-bit integer.
* [code:: fcvt.wu.q] Convert Quad to Word Unsigned: Converts a quad-precision value to an unsigned 32-bit integer.
* [code:: fcvt.q.w] Convert Word to Quad: Converts a signed 32-bit integer to a quad-precision value.
* [code:: fcvt.q.wu] Convert Word Unsigned to Quad: Converts an unsigned 32-bit integer to a quad-precision value.
* [code:: fcvt.l.q] Convert Quad to Long: Converts a quad-precision value to a signed 64-bit integer.
* [code:: fcvt.lu.q] Convert Quad to Long Unsigned: Converts a quad-precision value to an unsigned 64-bit integer.
* [code:: fcvt.q.l] Convert Long to Quad: Converts a signed 64-bit integer to a quad-precision value.
* [code:: fcvt.q.lu] Convert Long Unsigned to Quad: Converts an unsigned 64-bit integer to a quad-precision value.
* [code:: flh] Load Floating-Point Halfword: Loads a half-precision floating-point value from memory.
* [code:: fsh] Store Floating-Point Halfword: Stores a half-precision floating-point value to memory.
* [code:: fmadd.h] Floating-Point Multiply-Add Half: Performs (a × b) + c with half-precision operands.
* [code:: fmsub.h] Floating-Point Multiply-Subtract Half: Performs (a × b) − c with half-precision operands.
* [code:: fnmsub.h] Floating-Point Negative Multiply-Subtract Half: Performs −(a × b) + c with half-precision operands.
* [code:: fnmadd.h] Floating-Point Negative Multiply-Add Half: Performs −(a × b) − c with half-precision operands.
* [code:: fadd.h] Floating-Point Add Half: Adds two half-precision floating-point values.
* [code:: fsub.h] Floating-Point Subtract Half: Subtracts one half-precision floating-point value from another.
* [code:: fmul.h] Floating-Point Multiply Half: Multiplies two half-precision floating-point values.
* [code:: fdiv.h] Floating-Point Divide Half: Divides one half-precision floating-point value by another.
* [code:: fsqrt.h] Floating-Point Square Root Half: Computes the square root of a half-precision floating-point value.
* [code:: fsgnj.h] Floating-Point Sign Inject Half: Copies the sign of one half-precision value to another.
* [code:: fsgnjn.h] Floating-Point Sign Negate Half: Copies the negated sign of one half-precision value to another.
* [code:: fsgnjx.h] Floating-Point Sign XOR Half: Copies the XORed sign of two half-precision values.
* [code:: fmin.h] Floating-Point Minimum Half: Returns the smaller of two half-precision values.
* [code:: fmax.h] Floating-Point Maximum Half: Returns the larger of two half-precision values.
* [code:: fcvt.s.h] Convert Half to Single: Converts a half-precision value to a single-precision value.
* [code:: fcvt.h.s] Convert Single to Half: Converts a single-precision value to a half-precision value.
* [code:: fcvt.d.h] Convert Half to Double: Converts a half-precision value to a double-precision value.
* [code:: fcvt.h.d] Convert Double to Half: Converts a double-precision value to a half-precision value.
* [code:: fcvt.q.h] Convert Half to Quad: Converts a half-precision value to a quad-precision value.
* [code:: fcvt.h.q] Convert Quad to Half: Converts a quad-precision value to a half-precision value.
* [code:: feq.h] Floating-Point Equal Half: Compares two half-precision values for equality.
* [code:: flt.h] Floating-Point Less Than Half: Compares two half-precision values for less-than.
* [code:: fle.h] Floating-Point Less or Equal Half: Compares two half-precision values for less-or-equal.
* [code:: fclass.h] Floating-Point Classify Half: Classifies a half-precision value (e.g., NaN, infinity).
* [code:: fcvt.w.h] Convert Half to Word: Converts a half-precision value to a signed 32-bit integer.
* [code:: fcvt.wu.h] Convert Half to Word Unsigned: Converts a half-precision value to an unsigned 32-bit integer.
* [code:: fmv.x.h] Move Half to Integer: Moves a half-precision value to an integer register.
* [code:: fcvt.h.w] Convert Word to Half: Converts a signed 32-bit integer to a half-precision value.
* [code:: fcvt.h.wu] Convert Word Unsigned to Half: Converts an unsigned 32-bit integer to a half-precision value.
* [code:: fmv.h.x] Move Integer to Half: Moves an integer register value to a half-precision register.
* [code:: fcvt.l.h] Convert Half to Long: Converts a half-precision value to a signed 64
* [code:: fcvt.lu.h] Convert Half to Long Unsigned: Converts a half-precision value to an unsigned 64-bit integer.
* [code:: fcvt.h.l] Convert Long to Half: Converts a signed 64-bit integer to a half-precision value.
* [code:: fcvt.h.lu] Convert Long Unsigned to Half: Converts an unsigned 64-bit integer to a half-precision value.
* [code:: wrs.nto] Wait Reservation Set: Waits on a reservation set without timeout.
* [code:: wrs.sto] Wait Reservation Set with Timeout: Waits on a reservation set with a timeout.
