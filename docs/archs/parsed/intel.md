---
tags: [assembly, intel]
endianness: [little]
instruction:
  len: fixed
  bits: [16, 32, 64]
---
* [code:: mov] Move data between general-purpose registers; move data between memory and general purpose or segment registers; move immediates to general-purpose registers.
* [code:: cmove] Conditional move if equal.
* [code:: cmovz] Conditional move if zero.
* [code:: cmovne] Conditional move if not equal.
* [code:: cmovnz] Conditional move if not zero.
* [code:: cmova] Conditional move if above.
* [code:: cmovnbe] Conditional move if not below or equal.
* [code:: cmovae] Conditional move if above or equal.
* [code:: cmovnb] Conditional move if not below.
* [code:: cmovb] Conditional move if below.
* [code:: cmovnae] Conditional move if not above or equal.
* [code:: cmovbe] Conditional move if below or equal.
* [code:: cmovna] Conditional move if not above.
* [code:: cmovg] Conditional move if greater.
* [code:: cmovnle] Conditional move if not less or equal.
* [code:: cmovge] Conditional move if greater or equal.
* [code:: cmovnl] Conditional move if not less.
* [code:: cmovl] Conditional move if less.
* [code:: cmovnge] Conditional move if not greater or equal.
* [code:: cmovle] Conditional move if less or equal.
* [code:: cmovng] Conditional move if not greater.
* [code:: cmovc] Conditional move if carry.
* [code:: cmovnc] Conditional move if not carry.
* [code:: cmovo] Conditional move if overflow.
* [code:: cmovno] Conditional move if not overflow.
* [code:: cmovs] Conditional move if sign (negative).
* [code:: cmovns] Conditional move if not sign (non-negative).
* [code:: cmovp] Conditional move if parity.
* [code:: cmovpe] Conditional move if parity even.
* [code:: cmovnp] Conditional move if not parity.
* [code:: cmovpo] Conditional move if parity odd.
* [code:: xchg] Exchange.
* [code:: bswap] Byte swap.
* [code:: xadd] Exchange and add.
* [code:: cmpxchg] Compare and exchange.
* [code:: cmpxchg8b] Compare and exchange 8 bytes.
* [code:: push] Push onto stack.
* [code:: pop] Pop off of stack.
* [code:: pusha] Push general-purpose registers onto stack.
* [code:: pushad] Push general-purpose registers onto stack.
* [code:: popa] Pop general-purpose registers from stack.
* [code:: popad] Pop general-purpose registers from stack.
* [code:: cwd] Convert word to doubleword.
* [code:: cdq] Convert doubleword to quadword.
* [code:: cbw] Convert byte to word.
* [code:: cwde] Convert word to doubleword in EAX register.
* [code:: movsx] Move and sign extend.
* [code:: movzx] Move and zero extend.
* [code:: adcx] Unsigned integer add with carry.
* [code:: adox] Unsigned integer add with overflow.
* [code:: add] Integer add.
* [code:: adc] Add with carry.
* [code:: sub] Subtract.
* [code:: sbb] Subtract with borrow.
* [code:: imul] Signed multiply.
* [code:: mul] Unsigned multiply.
* [code:: idiv] Signed divide.
* [code:: div] Unsigned divide.
* [code:: inc] Increment.
* [code:: dec] Decrement.
* [code:: neg] Negate.
* [code:: daa] Decimal adjust after addition.
* [code:: das] Decimal adjust after subtraction.
* [code:: aaa] ASCII adjust after addition.
* [code:: aas] ASCII adjust after subtraction.
* [code:: aam] ASCII adjust after multiplication.
* [code:: aad] ASCII adjust before division.
* [code:: and] Perform bitwise logical AND.
* [code:: or] Perform bitwise logical OR.
* [code:: xor] Perform bitwise logical exclusive OR.
* [code:: not] Perform bitwise logical NOT.
* [code:: sar] Shift arithmetic right.
* [code:: shr] Shift logical right.
* [code:: sal] Shift arithmetic left.
* [code:: shl] Shift logical left.
* [code:: shrd] Shift right double.
* [code:: shld] Shift left double.
* [code:: ror] Rotate right.
* [code:: rol] Rotate left.
* [code:: rcr] Rotate through carry right.
* [code:: rcl] Rotate through carry left.
* [code:: bt] Bit test.
* [code:: bts] Bit test and set.
* [code:: btr] Bit test and reset.
* [code:: btc] Bit test and complement.
* [code:: bsf] Bit scan forward.
* [code:: bsr] Bit scan reverse.
* [code:: sete] Set byte if equal.
* [code:: setz] Set byte if zero.
* [code:: setne] Set byte if not equal.
* [code:: setnz] Set byte if not zero.
* [code:: seta] Set byte if above.
* [code:: setnbe] Set byte if not below or equal.
* [code:: setae] Set byte if above or equal.
* [code:: setnb] Set byte if not below.
* [code:: setnc] Set byte if not carry.
* [code:: setb] Set byte if below.
* [code:: setnae] Set byte if not above or equal.
* [code:: setc] Set byte if carry.
* [code:: setbe] Set byte if below or equal.
* [code:: setna] Set byte if not above.
* [code:: setg] Set byte if greater.
* [code:: setnle] Set byte if not less or equal.
* [code:: setge] Set byte if greater or equal.
* [code:: setnl] Set byte if not less.
* [code:: setl] Set byte if less.
* [code:: setnge] Set byte if not greater or equal.
* [code:: setle] Set byte if less or equal.
* [code:: setng] Set byte if not greater.
* [code:: sets] Set byte if sign (negative).
* [code:: setns] Set byte if not sign (non-negative).
* [code:: seto] Set byte if overflow.
* [code:: setno] Set byte if not overflow.
* [code:: setpe] Set byte if parity even.
* [code:: setp] Set byte if parity.
* [code:: setpo] Set byte if parity odd.
* [code:: setnp] Set byte if not parity.
* [code:: test] Logical compare.
* [code:: crc32] Provides hardware acceleration to calculate cyclic redundancy checks for fast and efficient implementation of data integrity protocols.
* [code:: popcnt] Calculates of number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
* [code:: jmp] Jump.
* [code:: je] Jump if equal.
* [code:: jz] Jump if zero.
* [code:: jne] Jump if not equal.
* [code:: jnz] Jump if not zero.
* [code:: ja] Jump if above.
* [code:: jnbe] Jump if not below or equal.
* [code:: jae] Jump if above or equal.
* [code:: jnb] Jump if not below.
* [code:: jb] Jump if below.
* [code:: jnae] Jump if not above or equal.
* [code:: jbe] Jump if below or equal.
* [code:: jna] Jump if not above.
* [code:: jg] Jump if greater.
* [code:: jnle] Jump if not less or equal.
* [code:: jge] Jump if greater or equal.
* [code:: jnl] Jump if not less.
* [code:: jl] Jump if less.
* [code:: jnge] Jump if not greater or equal.
* [code:: jle] Jump if less or equal.
* [code:: jng] Jump if not greater.
* [code:: jc] Jump if carry.
* [code:: jnc] Jump if not carry.
* [code:: jo] Jump if overflow.
* [code:: jno] Jump if not overflow.
* [code:: js] Jump if sign (negative).
* [code:: jns] Jump if not sign (non-negative).
* [code:: jpo] Jump if parity odd.
* [code:: jnp] Jump if not parity.
* [code:: jpe] Jump if parity even.
* [code:: jp] Jump if parity.
* [code:: jcxz] Jump register CX zero.
* [code:: jecxz] Jump register ECX zero.
* [code:: loop] Loop with ECX counter.
* [code:: loopz] Loop with ECX and zero.
* [code:: loope] Loop with ECX and equal.
* [code:: loopnz] Loop with ECX and not zero.
* [code:: loopne] Loop with ECX and not equal.
* [code:: call] Call procedure.
* [code:: ret] Return.
* [code:: iret] Return from interrupt.
* [code:: int] Software interrupt.
* [code:: into] Interrupt on overflow.
* [code:: bound] Detect value out of range.
* [code:: enter] High-level procedure entry.
* [code:: leave] High-level procedure exit.
* [code:: movs] Move string.
* [code:: movsb] Move byte string.
* [code:: movs] Move string.
* [code:: movsw] Move word string.
* [code:: movs] Move string.
* [code:: movsd] Move doubleword string.
* [code:: cmps] Compare string.
* [code:: cmpsb] Compare byte string.
* [code:: cmps] Compare string.
* [code:: cmpsw] Compare word string.
* [code:: cmps] Compare string.
* [code:: cmpsd] Compare doubleword string.
* [code:: scas] Scan string.
* [code:: scasb] Scan byte string.
* [code:: scas] Scan string.
* [code:: scasw] Scan word string.
* [code:: scas] Scan string.
* [code:: scasd] Scan doubleword string.
* [code:: lods] Load string.
* [code:: lodsb] Load byte string.
* [code:: lods] Load string.
* [code:: lodsw] Load word string.
* [code:: lods] Load string.
* [code:: lodsd] Load doubleword string.
* [code:: stos] Store string.
* [code:: stosb] Store byte string.
* [code:: stos] Store string.
* [code:: stosw] Store word string.
* [code:: stos] Store string.
* [code:: stosd] Store doubleword string.
* [code:: rep] Repeat while ECX not zero.
* [code:: repe] Repeat while equal.
* [code:: repz] Repeat while zero.
* [code:: repne] Repeat while not equal.
* [code:: repnz] Repeat while not zero.
* [code:: in] Read from a port.
* [code:: out] Write to a port.
* [code:: ins] Input string from port.
* [code:: insb] Input byte string from port.
* [code:: ins] Input string from port.
* [code:: insw] Input word string from port.
* [code:: ins] Input string from port.
* [code:: insd] Input doubleword string from port.
* [code:: outs] Output string to port.
* [code:: outsb] Output byte string to port.
* [code:: outs] Output string to port.
* [code:: outsw] Output word string to port.
* [code:: outs] Output string to port.
* [code:: outsd] Output doubleword string to port.
* [code:: enter] High-level procedure entry.
* [code:: leave] High-level procedure exit.
* [code:: stc] Set carry flag.
* [code:: clc] Clear the carry flag.
* [code:: cmc] Complement the carry flag.
* [code:: cld] Clear the direction flag.
* [code:: std] Set direction flag.
* [code:: lahf] Load flags into AH register.
* [code:: sahf] Store AH register into flags.
* [code:: pushf] Push EFLAGS onto stack.
* [code:: popf] Pop EFLAGS from stack.
* [code:: pushfd] Push EFLAGS onto stack.
* [code:: popfd] Pop EFLAGS from stack.
* [code:: sti] Set interrupt flag.
* [code:: cli] Clear the interrupt flag.
* [code:: lds] Load far pointer using DS.
* [code:: les] Load far pointer using ES.
* [code:: lfs] Load far pointer using FS.
* [code:: lgs] Load far pointer using GS.
* [code:: lss] Load far pointer using SS.
* [code:: lea] Load effective address.
* [code:: nop] No operation.
* [code:: ud] Undefined instruction.
* [code:: xlat] Table lookup translation.
* [code:: xlatb] Table lookup translation.
* [code:: cpuid] Processor identification.
* [code:: movbe] Move data after swapping data bytes.
* [code:: prefetchw] Prefetch data into cache in anticipation of write.
* [code:: prefetchwt1] Prefetch hint T1 with intent to write.
* [code:: clflush] Flushes and invalidates a memory operand and its associated cache line from all levels of the processor’s cache hierarchy.
* [code:: clflushopt] Flushes and invalidates a memory operand and its associated cache line from all levels of the processor’s cache hierarchy with optimized memory system throughput.
* [code:: xsave] Save processor extended states to memory.
* [code:: xsavec] Save processor extended states with compaction to memory.
* [code:: xsaveopt] Save processor extended states to memory, optimized.
* [code:: xrstor] Restore processor extended states from memory.
* [code:: xgetbv] Reads the state of an extended control register.
* [code:: rdrand] Retrieves a random number generated from hardware.
* [code:: rdseed] Retrieves a random number generated from hardware.
* [code:: andn] Bitwise AND of first source with inverted second source operands.
* [code:: bextr] Contiguous bitwise extract.
* [code:: blsi] Extract lowest set bit.
* [code:: blsmsk] Set all lower bits below first set bit to 1.
* [code:: blsr] Reset lowest set bit.
* [code:: bzhi] Zero high bits starting from specified bit position.
* [code:: lzcnt] Count the number of leading zero bits.
* [code:: mulx] Unsigned multiply without affecting arithmetic flags.
* [code:: pdep] Parallel deposit of bits using a mask.
* [code:: pext] Parallel extraction of bits using a mask.
* [code:: rorx] Rotate right without affecting arithmetic flags.
* [code:: sarx] Shift arithmetic right.
* [code:: shlx] Shift logic left.
* [code:: shrx] Shift logic right.
* [code:: tzcnt] Count the number of trailing zero bits.
* [code:: fld] Load floating-point value.
* [code:: fst] Store floating-point value.
* [code:: fstp] Store floating-point value and pop.
* [code:: fild] Load integer.
* [code:: fist] Store integer.
* [code:: fistp] Store integer and pop.
* [code:: fbld] Load BCD.
* [code:: fbstp] Store BCD and pop.
* [code:: fxch] Exchange registers.
* [code:: fcmove] Floating-point conditional move if equal.
* [code:: fcmovne] Floating-point conditional move if not equal.
* [code:: fcmovb] Floating-point conditional move if below.
* [code:: fcmovbe] Floating-point conditional move if below or equal.
* [code:: fcmovnb] Floating-point conditional move if not below.
* [code:: fcmovnbe] Floating-point conditional move if not below or equal.
* [code:: fcmovu] Floating-point conditional move if unordered.
* [code:: fcmovnu] Floating-point conditional move if not unordered.
* [code:: fadd] Add floating-point.
* [code:: faddp] Add floating-point and pop.
* [code:: fiadd] Add integer.
* [code:: fsub] Subtract floating-point.
* [code:: fsubp] Subtract floating-point and pop.
* [code:: fisub] Subtract integer.
* [code:: fsubr] Subtract floating-point reverse.
* [code:: fsubrp] Subtract floating-point reverse and pop.
* [code:: fisubr] Subtract integer reverse.
* [code:: fmul] Multiply floating-point.
* [code:: fmulp] Multiply floating-point and pop.
* [code:: fimul] Multiply integer.
* [code:: fdiv] Divide floating-point.
* [code:: fdivp] Divide floating-point and pop.
* [code:: fidiv] Divide integer.
* [code:: fdivr] Divide floating-point reverse.
* [code:: fdivrp] Divide floating-point reverse and pop.
* [code:: fidivr] Divide integer reverse.
* [code:: fprem] Partial remainder.
* [code:: fprem1] IEEE partial remainder.
* [code:: fabs] Absolute value.
* [code:: fchs] Change sign.
* [code:: frndint] Round to integer.
* [code:: fscale] Scale by power of two.
* [code:: fsqrt] Square root.
* [code:: fxtract] Extract exponent and significand.
* [code:: fcom] Compare floating-point.
* [code:: fcomp] Compare floating-point and pop.
* [code:: fcompp] Compare floating-point and pop twice.
* [code:: fucom] Unordered compare floating-point.
* [code:: fucomp] Unordered compare floating-point and pop.
* [code:: fucompp] Unordered compare floating-point and pop twice.
* [code:: ficom] Compare integer.
* [code:: ficomp] Compare integer and pop.
* [code:: fcomi] Compare floating-point and set EFLAGS.
* [code:: fucomi] Unordered compare floating-point and set EFLAGS.
* [code:: fcomip] Compare floating-point, set EFLAGS, and pop.
* [code:: fucomip] Unordered compare floating-point, set EFLAGS, and pop.
* [code:: ftst] Test floating-point (compare with 0.0).
* [code:: fxam] Examine floating-point.
* [code:: fsin] Sine.
* [code:: fcos] Cosine.
* [code:: fsincos] Sine and cosine.
* [code:: fptan] Partial tangent.
* [code:: fpatan] Partial arctangent.
* [code:: f2xm1] 2x − 1.
* [code:: fyl2x] y∗log2x.
* [code:: fyl2xp1] y∗log2(x+1).
* [code:: fld1] Load +1.0.
* [code:: fldz] Load +0.0.
* [code:: fldpi] Load π.
* [code:: fldl2e] Load log2e.
* [code:: fldln2] Load loge2.
* [code:: fldl2t] Load log210.
* [code:: fldlg2] Load log102.
* [code:: fincstp] Increment FPU register stack pointer.
* [code:: fdecstp] Decrement FPU register stack pointer.
* [code:: ffree] Free floating-point register.
* [code:: finit] Initialize FPU after checking error conditions.
* [code:: fninit] Initialize FPU without checking error conditions.
* [code:: fclex] Clear floating-point exception flags after checking for error conditions.
* [code:: fnclex] Clear floating-point exception flags without checking for error conditions.
* [code:: fstcw] Store FPU control word after checking error conditions.
* [code:: fnstcw] Store FPU control word without checking error conditions.
* [code:: fldcw] Load FPU control word.
* [code:: fstenv] Store FPU environment after checking error conditions.
* [code:: fnstenv] Store FPU environment without checking error conditions.
* [code:: fldenv] Load FPU environment.
* [code:: fsave] Save FPU state after checking error conditions.
* [code:: fnsave] Save FPU state without checking error conditions.
* [code:: frstor] Restore FPU state.
* [code:: fstsw] Store FPU status word after checking error conditions.
* [code:: fnstsw] Store FPU status word without checking error conditions.
* [code:: wait] Wait for FPU.
* [code:: fwait] Wait for FPU.
* [code:: fnop] FPU no operation.
* [code:: fxsave] Save x87 FPU and SIMD state.
* [code:: fxrstor] Restore x87 FPU and SIMD state.
* [code:: movd] Move doubleword.
* [code:: movq] Move quadword.
* [code:: packsswb] Pack words into bytes with signed saturation.
* [code:: packssdw] Pack doublewords into words with signed saturation.
* [code:: packuswb] Pack words into bytes with unsigned saturation.
* [code:: punpckhbw] Unpack high-order bytes.
* [code:: punpckhwd] Unpack high-order words.
* [code:: punpckhdq] Unpack high-order doublewords.
* [code:: punpcklbw] Unpack low-order bytes.
* [code:: punpcklwd] Unpack low-order words.
* [code:: punpckldq] Unpack low-order doublewords.
* [code:: paddb] Add packed byte integers.
* [code:: paddw] Add packed word integers.
* [code:: paddd] Add packed doubleword integers.
* [code:: paddsb] Add packed signed byte integers with signed saturation.
* [code:: paddsw] Add packed signed word integers with signed saturation.
* [code:: paddusb] Add packed unsigned byte integers with unsigned saturation.
* [code:: paddusw] Add packed unsigned word integers with unsigned saturation.
* [code:: psubb] Subtract packed byte integers.
* [code:: psubw] Subtract packed word integers.
* [code:: psubd] Subtract packed doubleword integers.
* [code:: psubsb] Subtract packed signed byte integers with signed saturation.
* [code:: psubsw] Subtract packed signed word integers with signed saturation.
* [code:: psubusb] Subtract packed unsigned byte integers with unsigned saturation.
* [code:: psubusw] Subtract packed unsigned word integers with unsigned saturation.
* [code:: pmulhw] Multiply packed signed word integers and store high result.
* [code:: pmullw] Multiply packed signed word integers and store low result.
* [code:: pmaddwd] Multiply and add packed word integers.
* [code:: pcmpeqb] Compare packed bytes for equal.
* [code:: pcmpeqw] Compare packed words for equal.
* [code:: pcmpeqd] Compare packed doublewords for equal.
* [code:: pcmpgtb] Compare packed signed byte integers for greater than.
* [code:: pcmpgtw] Compare packed signed word integers for greater than.
* [code:: pcmpgtd] Compare packed signed doubleword integers for greater than.
* [code:: pand] Bitwise logical AND.
* [code:: pandn] Bitwise logical AND NOT.
* [code:: por] Bitwise logical OR.
* [code:: pxor] Bitwise logical exclusive OR.
* [code:: psllw] Shift packed words left logical.
* [code:: pslld] Shift packed doublewords left logical.
* [code:: psllq] Shift packed quadword left logical.
* [code:: psrlw] Shift packed words right logical.
* [code:: psrld] Shift packed doublewords right logical.
* [code:: psrlq] Shift packed quadword right logical.
* [code:: psraw] Shift packed words right arithmetic.
* [code:: psrad] Shift packed doublewords right arithmetic.
* [code:: emms] Empty MMX state.
* [code:: movaps] Move four aligned packed single precision floating-point values between XMM registers or between an XMM register and memory.
* [code:: movups] Move four unaligned packed single precision floating-point values between XMM registers or between an XMM register and memory.
* [code:: movhps] Move two packed single precision floating-point values to and from the high quadword of an XMM register and memory.
* [code:: movhlps] Move two packed single precision floating-point values from the high quadword of an XMM register to the low quadword of another XMM register.
* [code:: movlps] Move two packed single precision floating-point values to and from the low quadword of an XMM register and memory.
* [code:: movlhps] Move two packed single precision floating-point values from the low quadword of an XMM register to the high quadword of another XMM register.
* [code:: movmskps] Extract sign mask from four packed single precision floating-point values.
* [code:: movss] Move scalar single precision floating-point value between XMM registers or between an XMM register and memory.
* [code:: addps] Add packed single precision floating-point values.
* [code:: addss] Add scalar single precision floating-point values.
* [code:: subps] Subtract packed single precision floating-point values.
* [code:: subss] Subtract scalar single precision floating-point values.
* [code:: mulps] Multiply packed single precision floating-point values.
* [code:: mulss] Multiply scalar single precision floating-point values.
* [code:: divps] Divide packed single precision floating-point values.
* [code:: divss] Divide scalar single precision floating-point values.
* [code:: rcpps] Compute reciprocals of packed single precision floating-point values.
* [code:: rcpss] Compute reciprocal of scalar single precision floating-point values.
* [code:: sqrtps] Compute square roots of packed single precision floating-point values.
* [code:: sqrtss] Compute square root of scalar single precision floating-point values.
* [code:: rsqrtps] Compute reciprocals of square roots of packed single precision floating-point values.
* [code:: rsqrtss] Compute reciprocal of square root of scalar single precision floating-point values.
* [code:: maxps] Return maximum packed single precision floating-point values.
* [code:: maxss] Return maximum scalar single precision floating-point values.
* [code:: minps] Return minimum packed single precision floating-point values.
* [code:: minss] Return minimum scalar single precision floating-point values.
* [code:: cmpps] Compare packed single precision floating-point values.
* [code:: cmpss] Compare scalar single precision floating-point values.
* [code:: comiss] Perform ordered comparison of scalar single precision floating-point values and set flags in EFLAGS register.
* [code:: ucomiss] Perform unordered comparison of scalar single precision floating-point values and set flags in EFLAGS register.
* [code:: andps] Perform bitwise logical AND of packed single precision floating-point values.
* [code:: andnps] Perform bitwise logical AND NOT of packed single precision floating-point values.
* [code:: orps] Perform bitwise logical OR of packed single precision floating-point values.
* [code:: xorps] Perform bitwise logical XOR of packed single precision floating-point values.
* [code:: shufps] Shuffles values in packed single precision floating-point operands.
* [code:: unpckhps] Unpacks and interleaves the two high-order values from two single precision floating-point operands.
* [code:: unpcklps] Unpacks and interleaves the two low-order values from two single precision floating-point operands.
* [code:: cvtpi2ps] Convert packed doubleword integers to packed single precision floating-point values.
* [code:: cvtsi2ss] Convert doubleword integer to scalar single precision floating-point value.
* [code:: cvtps2pi] Convert packed single precision floating-point values to packed doubleword integers.
* [code:: cvttps2pi] Convert with truncation packed single precision floating-point values to packed double word integers.
* [code:: cvtss2si] Convert a scalar single precision floating-point value to a doubleword integer.
* [code:: cvttss2si] Convert with truncation a scalar single precision floating-point value to a scalar double word integer.
* [code:: ldmxcsr] Load MXCSR register.
* [code:: stmxcsr] Save MXCSR register state.
* [code:: pavgb] Compute average of packed unsigned byte integers.
* [code:: pavgw] Compute average of packed unsigned word integers.
* [code:: pextrw] Extract word.
* [code:: pinsrw] Insert word.
* [code:: pmaxub] Maximum of packed unsigned byte integers.
* [code:: pmaxsw] Maximum of packed signed word integers.
* [code:: pminub] Minimum of packed unsigned byte integers.
* [code:: pminsw] Minimum of packed signed word integers.
* [code:: pmovmskb] Move byte mask.
* [code:: pmulhuw] Multiply packed unsigned integers and store high result.
* [code:: psadbw] Compute sum of absolute differences.
* [code:: pshufw] Shuffle packed integer word in MMX register.
* [code:: maskmovq] Non-temporal store of selected bytes from an MMX register into memory.
* [code:: movntq] Non-temporal store of quadword from an MMX register into memory.
* [code:: movntps] Non-temporal store of four packed single precision floating-point values from an XMM register into memory.
* [code:: prefetchh] Load 32 or more of bytes from memory to a selected level of the processor’s cache hierarchy.
* [code:: sfence] Serializes store operations.
* [code:: movapd] Move two aligned packed double precision floating-point values between XMM registers or between an XMM register and memory.
* [code:: movupd] Move two unaligned packed double precision floating-point values between XMM registers or between an XMM register and memory.
* [code:: movhpd] Move high packed double precision floating-point value to and from the high quadword of an XMM register and memory.
* [code:: movlpd] Move low packed single precision floating-point value to and from the low quadword of an XMM register and memory.
* [code:: movmskpd] Extract sign mask from two packed double precision floating-point values.
* [code:: movsd] Move scalar double precision floating-point value between XMM registers or between an XMM register and memory.
* [code:: addpd] Add packed double precision floating-point values.
* [code:: addsd] Add scalar double precision floating-point values.
* [code:: subpd] Subtract packed double precision floating-point values.
* [code:: subsd] Subtract scalar double precision floating-point values.
* [code:: mulpd] Multiply packed double precision floating-point values.
* [code:: mulsd] Multiply scalar double precision floating-point values.
* [code:: divpd] Divide packed double precision floating-point values.
* [code:: divsd] Divide scalar double precision floating-point values.
* [code:: sqrtpd] Compute packed square roots of packed double precision floating-point values.
* [code:: sqrtsd] Compute scalar square root of scalar double precision floating-point values.
* [code:: maxpd] Return maximum packed double precision floating-point values.
* [code:: maxsd] Return maximum scalar double precision floating-point values.
* [code:: minpd] Return minimum packed double precision floating-point values.
* [code:: minsd] Return minimum scalar double precision floating-point values.
* [code:: andpd] Perform bitwise logical AND of packed double precision floating-point values.
* [code:: andnpd] Perform bitwise logical AND NOT of packed double precision floating-point values.
* [code:: orpd] Perform bitwise logical OR of packed double precision floating-point values.
* [code:: xorpd] Perform bitwise logical XOR of packed double precision floating-point values.
* [code:: cmppd] Compare packed double precision floating-point values.
* [code:: cmpsd] Compare scalar double precision floating-point values.
* [code:: comisd] Perform ordered comparison of scalar double precision floating-point values and set flags in EFLAGS register.
* [code:: ucomisd] Perform unordered comparison of scalar double precision floating-point values and set flags in EFLAGS register.
* [code:: shufpd] Shuffles values in packed double precision floating-point operands.
* [code:: unpckhpd] Unpacks and interleaves the high values from two packed double precision floating-point operands.
* [code:: unpcklpd] Unpacks and interleaves the low values from two packed double precision floating-point operands.
* [code:: cvtpd2pi] Convert packed double precision floating-point values to packed doubleword integers.
* [code:: cvttpd2pi] Convert with truncation packed double precision floating-point values to packed double-word integers.
* [code:: cvtpi2pd] Convert packed doubleword integers to packed double precision floating-point values.
* [code:: cvtpd2dq] Convert packed double precision floating-point values to packed doubleword integers.
* [code:: cvttpd2dq] Convert with truncation packed double precision floating-point values to packed double-word integers.
* [code:: cvtdq2pd] Convert packed doubleword integers to packed double precision floating-point values.
* [code:: cvtps2pd] Convert packed single precision floating-point values to packed double precision floating-point values.
* [code:: cvtpd2ps] Convert packed double precision floating-point values to packed single precision floating-point values.
* [code:: cvtss2sd] Convert scalar single precision floating-point values to scalar double precision floating-point values.
* [code:: cvtsd2ss] Convert scalar double precision floating-point values to scalar single precision floating-point values.
* [code:: cvtsd2si] Convert scalar double precision floating-point values to a doubleword integer.
* [code:: cvttsd2si] Convert with truncation scalar double precision floating-point values to scalar doubleword integers.
* [code:: cvtsi2sd] Convert doubleword integer to scalar double precision floating-point value.
* [code:: cvtdq2ps] Convert packed doubleword integers to packed single precision floating-point values.
* [code:: cvtps2dq] Convert packed single precision floating-point values to packed doubleword integers.
* [code:: cvttps2dq] Convert with truncation packed single precision floating-point values to packed double-word integers.
* [code:: movdqa] Move aligned double quadword.
* [code:: movdqu] Move unaligned double quadword.
* [code:: movq2dq] Move quadword integer from MMX to XMM registers.
* [code:: movdq2q] Move quadword integer from XMM to MMX registers.
* [code:: pmuludq] Multiply packed unsigned doubleword integers.
* [code:: paddq] Add packed quadword integers.
* [code:: psubq] Subtract packed quadword integers.
* [code:: pshuflw] Shuffle packed low words.
* [code:: pshufhw] Shuffle packed high words.
* [code:: pshufd] Shuffle packed doublewords.
* [code:: pslldq] Shift double quadword left logical.
* [code:: psrldq] Shift double quadword right logical.
* [code:: punpckhqdq] Unpack high quadwords.
* [code:: punpcklqdq] Unpack low quadwords.
* [code:: lfence] Serializes load operations.
* [code:: mfence] Serializes load and store operations.
* [code:: pause] Improves the performance of “spin-wait loops”.
* [code:: maskmovdqu] Non-temporal store of selected bytes from an XMM register into memory.
* [code:: movntpd] Non-temporal store of two packed double precision floating-point values from an XMM register into memory.
* [code:: movntdq] Non-temporal store of double quadword from an XMM register into memory.
* [code:: fisttp] Behaves like the FISTP instruction but uses truncation, irrespective of the rounding mode specified in the floating-point control word (FCW).
* [code:: lddqu] Special 128-bit unaligned load designed to avoid cache line splits.
* [code:: addsubps] Performs single precision addition on the second and fourth pairs of 32-bit data elements within the operands; single precision subtraction on the first and third pairs.
* [code:: addsubpd] Performs double precision addition on the second pair of quadwords, and double precision subtraction on the first pair.
* [code:: haddps] Performs a single precision addition on contiguous data elements. The first data element of the result is obtained by adding the first and second elements of the first operand; the second element by adding the third and fourth elements of the first operand; the third by adding the first and second elements of the second operand; and the fourth by adding the third and fourth elements of the second operand.
* [code:: hsubps] Performs a single precision subtraction on contiguous data elements. The first data element of the result is obtained by subtracting the second element of the first operand from the first element of the first operand; the second element by subtracting the fourth element of the first operand from the third element of the first operand; the third by subtracting the second element of the second operand from the first element of the second operand; and the fourth by subtracting the fourth element of the second operand from the third element of the second operand.
* [code:: haddpd] Performs a double precision addition on contiguous data elements. The first data element of the result is obtained by adding the first and second elements of the first operand; the second element by adding the first and second elements of the second operand.
* [code:: hsubpd] Performs a double precision subtraction on contiguous data elements. The first data element of the result is obtained by subtracting the second element of the first operand from the first element of the first operand; the second element by subtracting the second element of the second operand from the first element of the second operand.
* [code:: movshdup] Loads/moves 128 bits; duplicating the second and fourth 32-bit data elements.
* [code:: movsldup] Loads/moves 128 bits; duplicating the first and third 32-bit data elements.
* [code:: movddup] Loads/moves 64 bits (bits[63:0] if the source is a register) and returns the same 64 bits in both the lower and upper halves of the 128-bit result register; duplicates the 64 bits from the source.
* [code:: monitor] Sets up an address range used to monitor write-back stores.
* [code:: mwait] Enables a logical processor to enter into an optimized state while waiting for a write-back store to the address range set up by the MONITOR instruction.
* [code:: phaddw] Adds two adjacent, signed 16-bit integers horizontally from the source and destination operands and packs the signed 16-bit results to the destination operand.
* [code:: phaddsw] Adds two adjacent, signed 16-bit integers horizontally from the source and destination operands and packs the signed, saturated 16-bit results to the destination operand.
* [code:: phaddd] Adds two adjacent, signed 32-bit integers horizontally from the source and destination operands and packs the signed 32-bit results to the destination operand.
* [code:: phsubw] Performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed 16-bit results are packed and written to the destination operand.
* [code:: phsubsw] Performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed and written to the destination operand.
* [code:: phsubd] Performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant double word of each pair in the source and destination operands. The signed 32-bit results are packed and written to the destination operand.
* [code:: pabsb] Computes the absolute value of each signed byte data element.
* [code:: pabsw] Computes the absolute value of each signed 16-bit data element.
* [code:: pabsd] Computes the absolute value of each signed 32-bit data element.
* [code:: pmaddubsw] Multiplies each unsigned byte value with the corresponding signed byte value to produce an intermediate, 16-bit signed integer. Each adjacent pair of 16-bit signed values are added horizontally. The signed, saturated 16-bit results are packed to the destination operand.
* [code:: pmulhrsw] Multiplies vertically each signed 16-bit integer from the destination operand with the corresponding signed 16-bit integer of the source operand, producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
* [code:: pshufb] Permutes each byte in place, according to a shuffle control mask. The least significant three or four bits of each shuffle control byte of the control mask form the shuffle index. The shuffle mask is unaffected. If the most significant bit (bit 7) of a shuffle control byte is set, the constant zero is written in the result byte.
* [code:: psignb] Negates each signed integer element of the destination operand if the sign of the corresponding data element in the source operand is less than zero.
* [code:: psignw] Negates each signed integer element of the destination operand if the sign of the corresponding data element in the source operand is less than zero.
* [code:: psignd] Negates each signed integer element of the destination operand if the sign of the corresponding data element in the source operand is less than zero.
* [code:: palignr] Source operand is appended after the destination operand forming an intermediate value of twice the width of an operand. The result is extracted from the intermediate value into the destination operand by selecting the 128-bit or 64-bit value that are right-aligned to the byte offset specified by the immediate value.
* [code:: pmulld] Returns four lower 32-bits of the 64-bit results of signed 32-bit integer multiplies.
* [code:: pmuldq] Returns two 64-bit signed result of signed 32-bit integer multiplies.
* [code:: dppd] Perform double precision dot product for up to 2 elements and broadcast.
* [code:: dpps] Perform single precision dot products for up to 4 elements and broadcast.
* [code:: movntdqa] Provides a non-temporal hint that can cause adjacent 16-byte items within an aligned 64-byte region (a streaming line) to be fetched and held in a small set of temporary buffers (“streaming load buffers”). Subsequent streaming loads to other aligned 16-byte items in the same streaming line may be supplied from the streaming load buffer and can improve throughput.
* [code:: blendpd] Conditionally copies specified double precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an immediate byte control.
* [code:: blendps] Conditionally copies specified single precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an immediate byte control.
* [code:: blendvpd] Conditionally copies specified double precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an implied mask.
* [code:: blendvps] Conditionally copies specified single precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an implied mask.
* [code:: pblendvb] Conditionally copies specified byte elements in the source operand to the corresponding elements in the destination, using an implied mask.
* [code:: pblendw] Conditionally copies specified word elements in the source operand to the corresponding elements in the destination, using an immediate byte control.
* [code:: pminuw] Compare packed unsigned word integers.
* [code:: pminud] Compare packed unsigned dword integers.
* [code:: pminsb] Compare packed signed byte integers.
* [code:: pminsd] Compare packed signed dword integers.
* [code:: pmaxuw] Compare packed unsigned word integers.
* [code:: pmaxud] Compare packed unsigned dword integers.
* [code:: pmaxsb] Compare packed signed byte integers.
* [code:: pmaxsd] Compare packed signed dword integers.
* [code:: roundps] Round packed single precision floating-point values into integer values and return rounded floating-point values.
* [code:: roundpd] Round packed double precision floating-point values into integer values and return rounded floating-point values.
* [code:: roundss] Round the low packed single precision floating-point value into an integer value and return a rounded floating-point value.
* [code:: roundsd] Round the low packed double precision floating-point value into an integer value and return a rounded floating-point value.
* [code:: extractps] Extracts a single precision floating-point value from a specified offset in an XMM register and stores the result to memory or a general-purpose register.
* [code:: insertps] Inserts a single precision floating-point value from either a 32-bit memory location or selected from a specified offset in an XMM register to a specified offset in the destination XMM register. In addition, INSERTPS allows zeroing out selected data elements in the destination, using a mask.
* [code:: pinsrb] Insert a byte value from a register or memory into an XMM register.
* [code:: pinsrd] Insert a dword value from 32-bit register or memory into an XMM register.
* [code:: pinsrq] Insert a qword value from 64-bit register or memory into an XMM register.
* [code:: pextrb] Extract a byte from an XMM register and insert the value into a general-purpose register or memory.
* [code:: pextrw] Extract a word from an XMM register and insert the value into a general-purpose register or memory.
* [code:: pextrd] Extract a dword from an XMM register and insert the value into a general-purpose register or memory.
* [code:: pextrq] Extract a qword from an XMM register and insert the value into a general-purpose register or memory.
* [code:: pmovsxbw] Sign extend the lower 8-bit integer of each packed word element into packed signed word integers.
* [code:: pmovzxbw] Zero extend the lower 8-bit integer of each packed word element into packed signed word integers.
* [code:: pmovsxbd] Sign extend the lower 8-bit integer of each packed dword element into packed signed dword integers.
* [code:: pmovzxbd] Zero extend the lower 8-bit integer of each packed dword element into packed signed dword integers.
* [code:: pmovsxwd] Sign extend the lower 16-bit integer of each packed dword element into packed signed dword integers.
* [code:: pmovzxwd] Zero extend the lower 16-bit integer of each packed dword element into packed signed dword integers.
* [code:: pmovsxbq] Sign extend the lower 8-bit integer of each packed qword element into packed signed qword integers.
* [code:: pmovzxbq] Zero extend the lower 8-bit integer of each packed qword element into packed signed qword integers.
* [code:: pmovsxwq] Sign extend the lower 16-bit integer of each packed qword element into packed signed qword integers.
* [code:: pmovzxwq] Zero extend the lower 16-bit integer of each packed qword element into packed signed qword integers.
* [code:: pmovsxdq] Sign extend the lower 32-bit integer of each packed qword element into packed signed qword integers.
* [code:: pmovzxdq] Zero extend the lower 32-bit integer of each packed qword element into packed signed qword integers.
* [code:: mpsadbw] Performs eight 4-byte wide Sum of Absolute Differences operations to produce eight word integers.
* [code:: phminposuw] Finds the value and location of the minimum unsigned word from one of 8 horizontally packed unsigned words. The resulting value and location (offset within the source) are packed into the low dword of the destination XMM register.
* [code:: ptest] Performs a logical AND between the destination with this mask and sets the ZF flag if the result is zero. The CF flag (zero for TEST) is set if the inverted mask AND’d with the destination is all zeroes.
* [code:: pcmpeqq] 128-bit packed qword equality test.
* [code:: packusdw] Packs dword to word with unsigned saturation.
* [code:: pcmpestri] Packed compare explicit-length strings, return index in ECX/RCX.
* [code:: pcmpestrm] Packed compare explicit-length strings, return mask in XMM0.
* [code:: pcmpistri] Packed compare implicit-length strings, return index in ECX/RCX.
* [code:: pcmpistrm] Packed compare implicit-length strings, return mask in XMM0.
* [code:: pcmpgtq] Performs logical compare of greater-than on packed integer quadwords.
* [code:: aesdec] Perform an AES decryption round using an 128-bit state and a round key.
* [code:: aesdeclast] Perform the last AES decryption round using an 128-bit state and a round key.
* [code:: aesenc] Perform an AES encryption round using an 128-bit state and a round key.
* [code:: aesenclast] Perform the last AES encryption round using an 128-bit state and a round key.
* [code:: aesimc] Perform an inverse mix column transformation primitive.
* [code:: aeskeygenassist] Assist the creation of round keys with a key expansion schedule.
* [code:: pclmulqdq] Perform carryless multiplication of two 64-bit numbers.
* [code:: vcvtph2ps] Convert eight/four data elements containing 16-bit floating-point data into eight/four single precision floating-point data.
* [code:: vcvtps2ph] Convert eight/four data elements containing single precision floating-point data into eight/four 16-bit floating-point data.
* [code:: xabort] Abort an RTM transaction execution.
* [code:: xacquire] Prefix hint to the beginning of an HLE transaction region.
* [code:: xrelease] Prefix hint to the end of an HLE transaction region.
* [code:: xbegin] Transaction begin of an RTM transaction region.
* [code:: xend] Transaction end of an RTM transaction region.
* [code:: xtest] Test if executing in a transactional region.
* [code:: xresldtrk] Resume tracking load addresses.
* [code:: xsusldtrk] Suspend tracking load addresses.
* [code:: sha1msg1] Perform an intermediate calculation for the next four SHA1 message dwords from the previous message dwords.
* [code:: sha1msg2] Perform the final calculation for the next four SHA1 message dwords from the intermediate message dwords.
* [code:: sha1nexte] Calculate SHA1 state E after four rounds.
* [code:: sha1rnds4] Perform four rounds of SHA1 operations.
* [code:: sha256msg1] Perform an intermediate calculation for the next four SHA256 message dwords.
* [code:: sha256msg2] Perform the final calculation for the next four SHA256 message dwords.
* [code:: sha256rnds2] Perform two rounds of SHA256 operations.
* [code:: valignd] Perform dword alignment of two concatenated source vectors.
* [code:: valignq] Perform qword alignment of two concatenated source vectors.
* [code:: vblendmpd] Replace the VBLENDVPD instructions (using opmask as select control).
* [code:: vblendmps] Replace the VBLENDVPS instructions (using opmask as select control).
* [code:: vcompresspd] Compress packed DP elements of a vector.
* [code:: vcompressps] Compress packed SP elements of a vector.
* [code:: vcvtpd2udq] Convert packed DP FP elements of a vector to packed unsigned 32-bit integers.
* [code:: vcvtps2udq] Convert packed SP FP elements of a vector to packed unsigned 32-bit integers.
* [code:: vcvtqq2pd] Convert packed signed 64-bit integers to packed DP FP elements.
* [code:: vcvtqq2ps] Convert packed signed 64-bit integers to packed SP FP elements.
* [code:: vcvtsd2usi] Convert the low DP FP element of a vector to an unsigned integer.
* [code:: vcvtss2usi] Convert the low SP FP element of a vector to an unsigned integer.
* [code:: vcvtudq2pd] Convert packed unsigned 32-bit integers to packed DP FP elements.
* [code:: vcvtudq2ps] Convert packed unsigned 32-bit integers to packed SP FP elements.
* [code:: vcvtusi2usd] Convert an unsigned integer to the low DP FP element and merge to a vector.
* [code:: vcvtusi2uss] Convert an unsigned integer to the low SP FP element and merge to a vector.
* [code:: vexpandpd] Expand packed DP elements of a vector.
* [code:: vexpandps] Expand packed SP elements of a vector.
* [code:: vextractf32x4] Extract a vector from a full-length vector with 32-bit granular update.
* [code:: vextractf64x4] Extract a vector from a full-length vector with 64-bit granular update.
* [code:: vextracti32x4] Extract a vector from a full-length vector with 32-bit granular update.
* [code:: vextracti64x4] Extract a vector from a full-length vector with 64-bit granular update.
* [code:: vfixupimmpd] Perform fix-up to special values in DP FP vectors.
* [code:: vfixupimmsd] Perform fix-up to special values of the low DP FP element.
* [code:: vfixupimmps] Perform fix-up to special values in SP FP vectors.
* [code:: vfixupimmss] Perform fix-up to special values of the low SP FP element.
* [code:: vgetexppd] Convert the exponent of DP FP elements of a vector into FP values.
* [code:: vgetexpsd] Convert the exponent of the low DP FP element in a vector into FP value.
* [code:: vgetexpps] Convert the exponent of SP FP elements of a vector into FP values.
* [code:: vgetexpss] Convert the exponent of the low SP FP element in a vector into FP value.
* [code:: vgetmantpd] Convert the mantissa of DP FP elements of a vector into FP values.
* [code:: vgetmantsd] Convert the mantissa of the low DP FP element of a vector into FP value.
* [code:: vgetmantps] Convert the mantissa of SP FP elements of a vector into FP values.
* [code:: vgetmantss] Convert the mantissa of the low SP FP element of a vector into FP value.
* [code:: vinsertf32x4] Insert a 128-bit vector into a full-length vector with 32-bit granular update.
* [code:: vinsertf64x4] Insert a 256-bit vector into a full-length vector with 64-bit granular update.
* [code:: vmovdqa32] VMOVDQA with 32-bit granular conditional update.
* [code:: vmovdqa64] VMOVDQA with 64-bit granular conditional update.
* [code:: vmovdqu32] VMOVDQU with 32-bit granular conditional update.
* [code:: vmovdqu64] VMOVDQU with 64-bit granular conditional update.
* [code:: vpblendmd] Blend dword elements using opmask as select control.
* [code:: vpblendmq] Blend qword elements using opmask as select control.
* [code:: vpbroadcastd] Broadcast from general-purpose register to vector register.
* [code:: vpbroadcastq] Broadcast from general-purpose register to vector register.
* [code:: vpcmpd] Compare packed signed dwords using specified primitive.
* [code:: vpcmpud] Compare packed unsigned dwords using specified primitive.
* [code:: vpcmpq] Compare packed signed quadwords using specified primitive.
* [code:: vpcmpuq] Compare packed unsigned quadwords using specified primitive.
* [code:: vpcompressq] Compress packed 64-bit elements of a vector.
* [code:: vpcompressd] Compress packed 32-bit elements of a vector.
* [code:: vpermi2d] Full permute of two tables of dword elements overwriting the index vector.
* [code:: vpermi2q] Full permute of two tables of qword elements overwriting the index vector.
* [code:: vpermi2pd] Full permute of two tables of DP elements overwriting the index vector.
* [code:: vpermi2ps] Full permute of two tables of SP elements overwriting the index vector.
* [code:: vpermt2d] Full permute of two tables of dword elements overwriting one source table.
* [code:: vpermt2q] Full permute of two tables of qword elements overwriting one source table.
* [code:: vpermt2pd] Full permute of two tables of DP elements overwriting one source table.
* [code:: vpermt2ps] Full permute of two tables of SP elements overwriting one source table.
* [code:: vpexpandd] Expand packed dword elements of a vector.
* [code:: vpexpandq] Expand packed qword elements of a vector.
* [code:: vpmaxsq] Compute maximum of packed signed 64-bit integer elements.
* [code:: vpmaxud] Compute maximum of packed unsigned 32-bit integer elements.
* [code:: vpmaxuq] Compute maximum of packed unsigned 64-bit integer elements.
* [code:: vpminsq] Compute minimum of packed signed 64-bit integer elements.
* [code:: vpminud] Compute minimum of packed unsigned 32-bit integer elements.
* [code:: vpminuq] Compute minimum of packed unsigned 64-bit integer elements.
* [code:: vpmovsqb] Down convert qword elements in a vector to byte elements using truncation with saturation.
* [code:: vpmovusqb] Down convert qword elements in a vector to byte elements using truncation with unsigned saturation.
* [code:: vpmovsqw] Down convert qword elements in a vector to word elements using truncation with saturation.
* [code:: vpmovusqw] Down convert qword elements in a vector to word elements using truncation with unsigned saturation.
* [code:: vpmovsqd] Down convert qword elements in a vector to dword elements using truncation with saturation.
* [code:: vpmovusqd] Down convert qword elements in a vector to dword elements using truncation with unsigned saturation.
* [code:: vpmovsdb] Down convert dword elements in a vector to byte elements using truncation with saturation.
* [code:: vpmovusdb] Down convert dword elements in a vector to byte elements using truncation with unsigned saturation.
* [code:: vpmovsdw] Down convert dword elements in a vector to word elements using truncation with saturation.
* [code:: vpmovusdw] Down convert dword elements in a vector to word elements using truncation with unsigned saturation.
* [code:: vprold] Rotate dword element left by a constant shift count with conditional update.
* [code:: vprolq] Rotate qword element left by a constant shift count with conditional update.
* [code:: vprolvd] Rotate dword element left by shift counts specified in a vector with conditional update.
* [code:: vprolvq] Rotate qword element left by shift counts specified in a vector with conditional update.
* [code:: vprord] Rotate dword element right by a constant shift count with conditional update.
* [code:: vprorq] Rotate qword element right by a constant shift count with conditional update.
* [code:: vprorrd] Rotate dword element right by shift counts specified in a vector with conditional update.
* [code:: vprorrq] Rotate qword element right by shift counts specified in a vector with conditional update.
* [code:: vpscatterdd] Scatter dword elements in a vector to memory using dword indices.
* [code:: vpscatterdq] Scatter qword elements in a vector to memory using dword indices.
* [code:: vpscatterqd] Scatter dword elements in a vector to memory using qword indices.
* [code:: vpscatterqq] Scatter qword elements in a vector to memory using qword indices.
* [code:: vpsraq] Shift qwords right by a constant shift count and shifting in sign bits.
* [code:: vpsravq] Shift qwords right by shift counts in a vector and shifting in sign bits.
* [code:: vptestnmd] Perform bitwise NAND of dword elements of two vectors and write results to opmask.
* [code:: vptestnmq] Perform bitwise NAND of qword elements of two vectors and write results to opmask.
* [code:: vpterlogd] Perform bitwise ternary logic operation of three vectors with 32-bit granular conditional update.
* [code:: vpterlogq] Perform bitwise ternary logic operation of three vectors with 64-bit granular conditional update.
* [code:: vptestmd] Perform bitwise AND of dword elements of two vectors and write results to opmask.
* [code:: vptestmq] Perform bitwise AND of qword elements of two vectors and write results to opmask.
* [code:: vrcp14pd] Compute approximate reciprocals of packed DP FP elements of a vector.
* [code:: vrcp14ps] Compute approximate reciprocals of packed SP FP elements of a vector.
* [code:: vrcp14sd] Compute the approximate reciprocal of the low DP FP element of a vector.
* [code:: vrcp14ss] Compute the approximate reciprocal of the low SP FP element of a vector.
* [code:: vrndscalepd] Round packed DP FP elements of a vector to specified number of fraction bits.
* [code:: vrndscaleps] Round packed SP FP elements of a vector to specified number of fraction bits.
* [code:: vrndscalesd] Round the low DP FP element of a vector to specified number of fraction bits.
* [code:: vrndscaless] Round the low SP FP element of a vector to specified number of fraction bits.
* [code:: vrsqrt14pd] Compute approximate reciprocals of square roots of packed DP FP elements of a vector.
* [code:: vrsqrt14ps] Compute approximate reciprocals of square roots of packed SP FP elements of a vector.
* [code:: vrsqrt14sd] Compute the approximate reciprocal of square root of the low DP FP element of a vector.
* [code:: vrsqrt14ss] Compute the approximate reciprocal of square root of the low SP FP element of a vector.
* [code:: vscalepd] Multiply packed DP FP elements of a vector by powers of two with exponents specified in a second vector.
* [code:: vscaleps] Multiply packed SP FP elements of a vector by powers of two with exponents specified in a second vector.
* [code:: vscalesd] Multiply the low DP FP element of a vector by powers of two with exponent specified in the corresponding element of a second vector.
* [code:: vscaless] Multiply the low SP FP element of a vector by powers of two with exponent specified in the corresponding element of a second vector.
* [code:: vscatterdd] Scatter SP FP elements in a vector to memory using dword indices.
* [code:: vscatterdq] Scatter DP FP elements in a vector to memory using dword indices.
* [code:: vscatterqd] Scatter SP FP elements in a vector to memory using qword indices.
* [code:: vscatterqq] Scatter DP FP elements in a vector to memory using qword indices.
* [code:: vshuff32x4] Shuffle 128-bit lanes of a vector with 32-bit granular conditional update.
* [code:: vshuff64x2] Shuffle 128-bit lanes of a vector with 64-bit granular conditional update.
* [code:: vshufi32x4] Shuffle 128-bit lanes of a vector with 32-bit granular conditional update.
* [code:: vshufi64x2] Shuffle 128-bit lanes of a vector with 64-bit granular conditional update.
* [code:: vcvtpd2qq] Convert packed DP FP elements of a vector to packed signed 64-bit integers.
* [code:: vcvtpd2uqq] Convert packed DP FP elements of a vector to packed unsigned 64-bit integers.
* [code:: vcvtps2qq] Convert packed SP FP elements of a vector to packed signed 64-bit integers.
* [code:: vcvtps2uqq] Convert packed SP FP elements of a vector to packed unsigned 64-bit integers.
* [code:: vcvtuqq2pd] Convert packed unsigned 64-bit integers to packed DP FP elements.
* [code:: vcvtuqq2ps] Convert packed unsigned 64-bit integers to packed SP FP elements.
* [code:: vextractf64x2] Extract a vector from a full-length vector with 64-bit granular update.
* [code:: vextracti64x2] Extract a vector from a full-length vector with 64-bit granular update.
* [code:: vfpclasspd] Test packed DP FP elements in a vector by numeric/special-value category.
* [code:: vfpclassps] Test packed SP FP elements in a vector by numeric/special-value category.
* [code:: vfpclasssd] Test the low DP FP element by numeric/special-value category.
* [code:: vfpclassss] Test the low SP FP element by numeric/special-value category.
* [code:: vinsertf64x2] Insert a 128-bit vector into a full-length vector with 64-bit granular update.
* [code:: vinserti64x2] Insert a 128-bit vector into a full-length vector with 64-bit granular update.
* [code:: vpmovm2d] Convert opmask register to vector register in 32-bit granularity.
* [code:: vpmovm2q] Convert opmask register to vector register in 64-bit granularity.
* [code:: vpmovb2d] Convert a vector register in 32-bit granularity to an opmask register.
* [code:: vpmovb2q] Convert a vector register in 64-bit granularity to an opmask register.
* [code:: vpmovb2d2m] Convert a vector register in 32-bit granularity to an opmask register.
* [code:: vpmovb2q2m] Convert a vector register in 64-bit granularity to an opmask register.
* [code:: vpmullq] Multiply packed signed 64-bit integer elements of two vectors and store low 64-bit signed result.
* [code:: vrangepd] Perform RANGE operation on each pair of DP FP elements of two vectors using specified range primitive in imm8.
* [code:: vrangesd] Perform RANGE operation on the pair of low DP FP elements of two vectors using specified range primitive in imm8.
* [code:: vreducepd] Perform Reduction operation on packed DP FP elements of a vector using specified reduction primitive in imm8.
* [code:: vreducesd] Perform Reduction operation on the low DP FP element of a vector using specified reduction primitive in imm8.
* [code:: vdbpsadbw] Double block packed Sum-Absolute-Differences on unsigned bytes.
* [code:: vmovdqu8] VMOVDQU with 8-bit granular conditional update.
* [code:: vmovdqu16] VMOVDQU with 16-bit granular conditional update.
* [code:: vpblendmb] Replaces the VPBLENDVB instruction (using opmask as select control).
* [code:: vpblendmw] Blend word elements using opmask as select control.
* [code:: vpbroadcastb] Broadcast from general-purpose register to vector register.
* [code:: vpbroadcastw] Broadcast from general-purpose register to vector register.
* [code:: vpcmpb] Compare packed signed bytes using specified primitive.
* [code:: vpcmpub] Compare packed unsigned bytes using specified primitive.
* [code:: vpcmpw] Compare packed signed words using specified primitive.
* [code:: vpcmpuw] Compare packed unsigned words using specified primitive.
* [code:: vpermw] Permute packed word elements.
* [code:: vpermi2b] Full permute from two tables of byte elements overwriting the index vector.
* [code:: vpermi2w] Full permute from two tables of word elements overwriting the index vector.
* [code:: vpmovm2b] Convert opmask register to vector register in 8-bit granularity.
* [code:: vpmovm2w] Convert opmask register to vector register in 16-bit granularity.
* [code:: vpmovb2m] Convert a vector register in 8-bit granularity to an opmask register.
* [code:: vpmovw2m] Convert a vector register in 16-bit granularity to an opmask register.
* [code:: vpmovwb] Down convert word elements in a vector to byte elements using truncation.
* [code:: vpmovswb] Down convert word elements in a vector to byte elements using saturation.
* [code:: vpmovuswb] Down convert word elements in a vector to byte elements using unsigned saturation.
* [code:: vpsllvw] Shift word elements in a vector left by shift counts in a vector.
* [code:: vpsravw] Shift words right by shift counts in a vector and shifting in sign bits.
* [code:: vpsrlvw] Shift word elements in a vector right by shift counts in a vector.
* [code:: vptestnmb] Perform bitwise NAND of byte elements of two vectors and write results to opmask.
* [code:: vptestnmw] Perform bitwise NAND of word elements of two vectors and write results to opmask.
* [code:: vptestmb] Perform bitwise AND of byte elements of two vectors and write results to opmask.
* [code:: vptestmw] Perform bitwise AND of word elements of two vectors and write results to opmask.
* [code:: vpbroadcastm] Broadcast from opmask register to vector register.
* [code:: vpconflictd] Detect conflicts within a vector of packed 32-bit integers.
* [code:: vpconflictq] Detect conflicts within a vector of packed 64-bit integers.
* [code:: vplzcntd] Count the number of leading zero bits of packed dword elements.
* [code:: vplzcntq] Count the number of leading zero bits of packed qword elements.
* [code:: kaddb] Add two 8-bit opmasks.
* [code:: kaddw] Add two 16-bit opmasks.
* [code:: kaddd] Add two 32-bit opmasks.
* [code:: kaddq] Add two 64-bit opmasks.
* [code:: kandb] Logical AND two 8-bit opmasks.
* [code:: kandw] Logical AND two 16-bit opmasks.
* [code:: kandd] Logical AND two 32-bit opmasks.
* [code:: kandq] Logical AND two 64-bit opmasks.
* [code:: kandnb] Logical AND NOT two 8-bit opmasks.
* [code:: kandnw] Logical AND NOT two 16-bit opmasks.
* [code:: kandnd] Logical AND NOT two 32-bit opmasks.
* [code:: kandnq] Logical AND NOT two 64-bit opmasks.
* [code:: kmovb] Move from or move to opmask register of 8-bit data.
* [code:: kmovw] Move from or move to opmask register of 16-bit data.
* [code:: kmovd] Move from or move to opmask register of 32-bit data.
* [code:: kmovq] Move from or move to opmask register of 64-bit data.
* [code:: knotb] Bitwise NOT of two 8-bit opmasks.
* [code:: knotw] Bitwise NOT of two 16-bit opmasks.
* [code:: knotd] Bitwise NOT of two 32-bit opmasks.
* [code:: knotq] Bitwise NOT of two 64-bit opmasks.
* [code:: korb] Logical OR two 8-bit opmasks.
* [code:: korw] Logical OR two 16-bit opmasks.
* [code:: kord] Logical OR two 32-bit opmasks.
* [code:: korq] Logical OR two 64-bit opmasks.
* [code:: kortestb] Update EFLAGS according to the result of bitwise OR of two 8-bit opmasks.
* [code:: kortestw] Update EFLAGS according to the result of bitwise OR of two 16-bit opmasks.
* [code:: kortestd] Update EFLAGS according to the result of bitwise OR of two 32-bit opmasks.
* [code:: kortestq] Update EFLAGS according to the result of bitwise OR of two 64-bit opmasks.
* [code:: kshiftlb] Shift left 8-bit opmask by specified count.
* [code:: kshiftlw] Shift left 16-bit opmask by specified count.
* [code:: kshiftdb] Shift left 32-bit opmask by specified count.
* [code:: kshiftlq] Shift left 64-bit opmask by specified count.
* [code:: kshiftrb] Shift right 8-bit opmask by specified count.
* [code:: kshiftrw] Shift right 16-bit opmask by specified count.
* [code:: kshiftrd] Shift right 32-bit opmask by specified count.
* [code:: kshiftrq] Shift right 64-bit opmask by specified count.
* [code:: ktestb] Update EFLAGS according to the result of bitwise TEST of two 8-bit opmasks.
* [code:: ktestw] Update EFLAGS according to the result of bitwise TEST of two 16-bit opmasks.
* [code:: ktestd] Update EFLAGS according to the result of bitwise TEST of two 32-bit opmasks.
* [code:: ktestq] Update EFLAGS according to the result of bitwise TEST of two 64-bit opmasks.
* [code:: kunpckbw] Unpack and interleave two 8-bit opmasks into 16-bit mask.
* [code:: kunpckwd] Unpack and interleave two 16-bit opmasks into 32-bit mask.
* [code:: kunpckdq] Unpack and interleave two 32-bit opmasks into 64-bit mask.
* [code:: kxnorb] Bitwise logical XNOR of two 8-bit opmasks.
* [code:: kxorb] Logical XOR of two 8-bit opmasks.
* [code:: kxornb] Bitwise logical XNOR of two 16-bit opmasks.
* [code:: kxornw] Logical XOR of two 16-bit opmasks.
* [code:: kxornb] Bitwise logical XNOR of two 32-bit opmasks.
* [code:: kxornw] Logical XOR of two 32-bit opmasks.
* [code:: kxornb] Bitwise logical XNOR of two 64-bit opmasks.
* [code:: kxornw] Logical XOR of two 64-bit opmasks.
* [code:: vexp2pd] Compute approximate base-2 exponential of packed DP FP elements of a vector.
* [code:: vexp2ps] Compute approximate base-2 exponential of packed SP FP elements of a vector.
* [code:: vexp2sd] Compute approximate base-2 exponential of the low DP FP element of a vector.
* [code:: vexp2ss] Compute approximate base-2 exponential of the low SP FP element of a vector.
* [code:: vrcp28pd] Compute approximate reciprocals to 28 bits of packed DP FP elements of a vector.
* [code:: vrcp28ps] Compute approximate reciprocals to 28 bits of packed SP FP elements of a vector.
* [code:: vrcp28sd] Compute the approximate reciprocal to 28 bits of the low DP FP element of a vector.
* [code:: vrcp28ss] Compute the approximate reciprocal to 28 bits of the low SP FP element of a vector.
* [code:: vrsqrt28pd] Compute approximate reciprocals of square roots to 28 bits of packed DP FP elements of a vector.
* [code:: vrsqrt28ps] Compute approximate reciprocals of square roots to 28 bits of packed SP FP elements of a vector.
* [code:: vrsqrt28sd] Compute the approximate reciprocal of square root to 28 bits of the low DP FP element of a vector.
* [code:: vrsqrt28ss] Compute the approximate reciprocal of square root to 28 bits of the low SP FP element of a vector.
* [code:: vgatherpf0dpd] Sparse prefetch of packed DP FP vector with T0 hint using dword indices.
* [code:: vgatherpf0dps] Sparse prefetch of packed DP FP vector with T0 hint using dword indices.
* [code:: vgatherpf0qpd] Sparse prefetch of packed DP FP vector with T0 hint using qword indices.
* [code:: vgatherpf0qps] Sparse prefetch of packed DP FP vector with T0 hint using qword indices.
* [code:: vgatherpf1dpd] Sparse prefetch of packed DP FP vector with T1 hint using dword indices.
* [code:: vgatherpf1dps] Sparse prefetch of packed DP FP vector with T1 hint using dword indices.
* [code:: vgatherpf1qpd] Sparse prefetch of packed DP FP vector with T1 hint using qword indices.
* [code:: vgatherpf1qps] Sparse prefetch of packed DP FP vector with T1 hint using qword indices.
* [code:: vscatterpf0dpd] Sparse prefetch of packed DP FP vector with T0 hint to write using dword indices.
* [code:: vscatterpf0dps] Sparse prefetch of packed DP FP vector with T0 hint to write using dword indices.
* [code:: vscatterpf0qpd] Sparse prefetch of packed DP FP vector with T0 hint to write using qword indices.
* [code:: vscatterpf0qps] Sparse prefetch of packed DP FP vector with T0 hint to write using qword indices.
* [code:: vscatterpf1dpd] Sparse prefetch of packed DP FP vector with T1 hint to write using dword indices.
* [code:: vscatterpf1dps] Sparse prefetch of packed DP FP vector with T1 hint to write using dword indices.
* [code:: vscatterpf1qpd] Sparse prefetch of packed DP FP vector with T1 hint to write using qword indices.
* [code:: vscatterpf1qps] Sparse prefetch of packed DP FP vector with T1 hint to write using qword indices.
* [code:: vaddph] Add packed FP16 values.
* [code:: vaddsh] Add scalar FP16 values.
* [code:: vcmpph] Compare packed FP16 values.
* [code:: vcmpsh] Compare scalar FP16 values.
* [code:: vcomish] Compare scalar ordered FP16 values and set EFLAGS.
* [code:: vcvtdq2ph] Convert packed signed doubleword integers to packed FP16 values.
* [code:: vcvtpd2ph] Convert packed double precision FP values to packed FP16 values.
* [code:: vcvtph2dq] Convert packed FP16 values to signed doubleword integers.
* [code:: vcvtph2qq] Convert packed FP16 values to signed quadword integers.
* [code:: vcvtph2dq] Convert packed FP16 values to signed doubleword integers.
* [code:: vcvtph2qq] Convert packed FP16 values to signed quadword integers.
* [code:: vcvtph2pd] Convert packed FP16 values to FP64 values.
* [code:: vcvtph2ps] Convert packed FP16 values to single precision floating-point values.
* [code:: vcvtph2psx] Convert packed FP16 values to single precision floating-point values.
* [code:: vcvtph2qq] Convert packed FP16 values to signed quadword integer values.
* [code:: vcvtph2udq] Convert packed FP16 values to unsigned doubleword integers.
* [code:: vcvtph2uqq] Convert packed FP16 values to unsigned quadword integers.
* [code:: vcvtph2uw] Convert packed FP16 values to unsigned word integers.
* [code:: vcvtph2w] Convert packed FP16 values to signed word integers.
* [code:: vcvtps2ph] Convert packed single precision floating-point values to packed FP16 values.
* [code:: vcvtps2phx] Convert packed single precision floating-point values to packed FP16 values.
* [code:: vcvtqq2ph] Convert packed signed quadword integers to packed FP16 values.
* [code:: vcvtsd2sh] Convert low FP64 value to an FP16 value.
* [code:: vcvtsh2sd] Convert low FP16 value to an FP64 value.
* [code:: vcvtsh2ss] Convert low FP16 value to an FP32 value.
* [code:: vcvtsh2si] Convert low FP16 value to signed integer.
* [code:: vcvtsh2usi] Convert low FP16 value to unsigned integer.
* [code:: vcvtsi2sh] Convert a signed doubleword/quadword integer to an FP16 value.
* [code:: vcvtss2sh] Convert low FP32 value to an FP16 value.
* [code:: vcvttph2dq] Convert with truncation packed FP16 values to signed doubleword integers.
* [code:: vcvttph2qq] Convert with truncation packed FP16 values to signed quadword integers.
* [code:: vcvttph2udq] Convert with truncation packed FP16 values to unsigned doubleword integers.
* [code:: vcvttph2uqq] Convert with truncation packed FP16 values to unsigned quadword integers.
* [code:: vcvttph2uw] Convert packed FP16 values to unsigned word integers.
* [code:: vcvttph2w] Convert packed FP16 values to signed word integers.
* [code:: vcvttsh2si] Convert with truncation low FP16 value to a signed integer.
* [code:: vcvttsh2usi] Convert with truncation low FP16 value to a unsigned integer.
* [code:: vcvtudq2ph] Convert packed unsigned doubleword integers to packed FP16 values.
* [code:: vcvtuqq2ph] Convert packed unsigned quadword integers to packed FP16 values.
* [code:: vcvtusi2sh] Convert unsigned doubleword integer to an FP16 value.
* [code:: vcvtuw2ph] Convert packed unsigned word integers to FP16 values.
* [code:: vcvtw2ph] Convert packed signed word integers to FP16 values.
* [code:: vdivph] Divide packed FP16 values.
* [code:: vdivsh] Divide scalar FP16 values.
* [code:: vfmaddcph] Complex multiply and accumulate FP16 values.
* [code:: vfcmaddcph] Complex multiply and accumulate FP16 values.
* [code:: vfmaddcsh] Complex multiply and accumulate scalar FP16 values.
* [code:: vfcmaddcsh] Complex multiply and accumulate scalar FP16 values.
* [code:: vfmulcph] Complex multiply FP16 values.
* [code:: vfcmulcph] Complex multiply FP16 values.
* [code:: vfmulcsh] Complex multiply scalar FP16 values.
* [code:: vfcmulcsh] Complex multiply scalar FP16 values.
* [code:: vfmadd132ph] Fused multiply-add of packed FP16 values.
* [code:: vfmadd213ph] Fused multiply-add of packed FP16 values.
* [code:: vfmadd231ph] Fused multiply-add of packed FP16 values.
* [code:: vfnmadd132ph] Fused multiply-add of packed FP16 values.
* [code:: vfnmadd213ph] Fused multiply-add of packed FP16 values.
* [code:: vfnmadd231ph] Fused multiply-add of packed FP16 values.
* [code:: vfmadd132sh] Fused multiply-add of scalar FP16 values.
* [code:: vfmadd213sh] Fused multiply-add of scalar FP16 values.
* [code:: vfmadd231sh] Fused multiply-add of scalar FP16 values.
* [code:: vfnmadd132sh] Fused multiply-add of scalar FP16 values.
* [code:: vfnmadd213sh] Fused multiply-add of scalar FP16 values.
* [code:: vfnmadd231sh] Fused multiply-add of scalar FP16 values.
* [code:: vfmaddsub132ph] Fused multiply-alternating add/subtract of packed FP16 values.
* [code:: vfmaddsub213ph] Fused multiply-alternating add/subtract of packed FP16 values.
* [code:: vfmaddsub231ph] Fused multiply-alternating add/subtract of packed FP16 values.
* [code:: vfmsubadd132ph] Fused multiply-alternating subtract/add of packed FP16 values.
* [code:: vfmsubadd213ph] Fused multiply-alternating subtract/add of packed FP16 values.
* [code:: vfmsubadd231ph] Fused multiply-alternating subtract/add of packed FP16 values.
* [code:: vfmsub132ph] Fused multiply-subtract of packed FP16 values.
* [code:: vfmsub213ph] Fused multiply-subtract of packed FP16 values.
* [code:: vfmsub231ph] Fused multiply-subtract of packed FP16 values.
* [code:: vfnmsub132ph] Fused multiply-subtract of packed FP16 values.
* [code:: vfnmsub213ph] Fused multiply-subtract of packed FP16 values.
* [code:: vfnmsub231ph] Fused multiply-subtract of packed FP16 values.
* [code:: vfmsub132sh] Fused multiply-subtract of scalar FP16 values.
* [code:: vfmsub213sh] Fused multiply-subtract of scalar FP16 values.
* [code:: vfmsub231sh] Fused multiply-subtract of scalar FP16 values.
* [code:: vfnmsub132sh] Fused multiply-subtract of scalar FP16 values.
* [code:: vfnmsub213sh] Fused multiply-subtract of scalar FP16 values.
* [code:: vfnmsub231sh] Fused multiply-subtract of scalar FP16 values.
* [code:: vfpclassph] Test types of packed FP16 values.
* [code:: vfpclasssh] Test types of scalar FP16 values.
* [code:: vgetexpph] Convert exponents of packed FP16 values to FP16 values.
* [code:: vgetexpsh] Convert exponents of scalar FP16 values to FP16 values.
* [code:: vgetmantph] Extract FP16 vector of normalized mantissas from FP16 vector.
* [code:: vgetmantsh] Extract FP16 vector of normalized mantissas from FP16 scalar.
* [code:: vmaxph] Return maximum of packed FP16 values.
* [code:: vmaxps] Return maximum of scalar FP16 values.
* [code:: vminph] Return minimum of packed FP16 values.
* [code:: vminps] Return minimum of scalar FP16 values.
* [code:: vmovsh] Move scalar FP16 value.
* [code:: vmovw] Move word.
* [code:: vmulph] Multiply packed FP16 values.
* [code:: vmulsh] Multiply scalar FP16 values.
* [code:: vrcpph] Compute reciprocals of packed FP16 values.
* [code:: vrcpsh] Compute reciprocals of scalar FP16 values.
* [code:: vreduceph] Perform reduction transformation on packed FP16 values.
* [code:: vreducesh] Perform reduction transformation on scalar FP16 values.
* [code:: vrndscaleph] Round packed FP16 values to include a given number of fraction bits.
* [code:: vrndscalesh] Round scalar FP16 values to include a given number of fraction bits.
* [code:: vrsqrtph] Compute reciprocals of square roots of packed FP16 values.
* [code:: vrsqrtsh] Compute reciprocals of square roots of scalar FP16 values.
* [code:: vscaleph] Scale packed FP16 values with FP16 values.
* [code:: vscalesh] Scale scalar FP16 values with FP16 values.
* [code:: vsqrtph] Compute square root of packed FP16 values.
* [code:: vsqrtsh] Compute square root of scalar FP16 values.
* [code:: vsubph] Subtract packed FP16 values.
* [code:: vsubsh] Subtract scalar FP16 values.
* [code:: vucomish] Unordered compare scalar FP16 values and set EFLAGS.
* [code:: clac] Clear AC Flag in EFLAGS register.
* [code:: stac] Set AC Flag in EFLAGS register.
* [code:: lgdt] Load global descriptor table (GDT) register.
* [code:: sgdt] Store global descriptor table (GDT) register.
* [code:: lldt] Load local descriptor table (LDT) register.
* [code:: sldt] Store local descriptor table (LDT) register.
* [code:: ltr] Load task register.
* [code:: str] Store task register.
* [code:: lidt] Load interrupt descriptor table (IDT) register.
* [code:: sidt] Store interrupt descriptor table (IDT) register.
* [code:: mov] Load and store control registers.
* [code:: lmsw] Load machine status word.
* [code:: smsw] Store machine status word.
* [code:: clts] Clear the task-switched flag.
* [code:: arpl] Adjust requested privilege level.
* [code:: lar] Load access rights.
* [code:: lsl] Load segment limit.
* [code:: verr] Verify segment for reading.
* [code:: verw] Verify segment for writing.
* [code:: mov] Load and store debug registers.
* [code:: invd] Invalidate cache, no writeback.
* [code:: wbinvd] Invalidate cache, with writeback.
* [code:: invlpg] Invalidate TLB Entry.
* [code:: invpcid] Invalidate Process-Context Identifier.
* [code:: lock] (prefix) Perform atomic access to memory (can be applied to a number of general purpose instructions that provide memory source/destination access).
* [code:: hlt] Halt processor.
* [code:: rsm] Return from system management mode (SMM).
* [code:: rdmsr] Read model-specific register.
* [code:: wrmsr] Write model-specific register.
* [code:: rdpmc] Read performance monitoring counters.
* [code:: rdtsc] Read time stamp counter.
* [code:: rdtscp] Read time stamp counter and processor ID.
* [code:: sysenter] Fast System Call, transfers to a flat protected mode kernel at CPL = 0.
* [code:: sysexit] Fast System Call, transfers to a flat protected mode kernel at CPL = 3.
* [code:: xsave] Save processor extended states to memory.
* [code:: xsavec] Save processor extended states with compaction to memory.
* [code:: xsaveopt] Save processor extended states to memory, optimized.
* [code:: xsaves] Save processor supervisor-mode extended states to memory.
* [code:: xrstor] Restore processor extended states from memory.
* [code:: xrstors] Restore processor supervisor-mode extended states from memory.
* [code:: xgetbv] Reads the state of an extended control register.
* [code:: xsetbv] Writes the state of an extended control register.
* [code:: rdfsbase] Reads from FS base address at any privilege level.
* [code:: rdgsbase] Reads from GS base address at any privilege level.
* [code:: wrfsbase] Writes to FS base address at any privilege level.
* [code:: wrgsbase] Writes to GS base address at any privilege level.
* [code:: cdqe] Convert doubleword to quadword.
* [code:: cmpsq] Compare string operands.
* [code:: cmpxchg16b] Compare RDX:RAX with m128.
* [code:: lodsq] Load qword at address (R)SI into RAX.
* [code:: movsq] Move qword from address (R)SI to (R)DI.
* [code:: movzx] (64-bits) Move bytes/words to doublewords/quadwords, zero-extension.
* [code:: stosq] Store RAX at address RDI.
* [code:: swapgs] Exchanges current GS base register value with value in MSR address C0000102H.
* [code:: syscall] Fast call to privilege level 0 system procedures.
* [code:: vmptrld] Takes a single 64-bit source operand in memory. It makes the referenced VMCS active and current.
* [code:: vmptrst] Takes a single 64-bit destination operand that is in memory. Current-VMCS pointer is stored into the destination operand.
* [code:: vmclear] Takes a single 64-bit operand in memory. The instruction sets the launch state of the VMCS referenced by the operand to “clear”, renders that VMCS inactive, and ensures that data for the VMCS have been written to the VMCS-data area in the referenced VMCS region.
* [code:: vmread] Reads a component from the VMCS (the encoding of that field is given in a register operand) and stores it into a destination operand.
* [code:: vmwrite] Writes a component to the VMCS (the encoding of that field is given in a register operand) from a source operand.
* [code:: vmlaunch] Launches a virtual machine managed by the VMCS. A VM entry occurs, transferring control to the VM.
* [code:: vmresume] Resumes a virtual machine managed by the VMCS. A VM entry occurs, transferring control to the VM.
* [code:: vmxoff] Causes the processor to leave VMX operation.
* [code:: vmxon] Takes a single 64-bit source operand in memory. It causes a logical processor to enter VMX root operation and to use the memory referenced by the operand to support VMX operation.
* [code:: invept] Invalidate cached Extended Page Table (EPT) mappings in the processor to synchronize address translation in virtual machines with memory-resident EPT pages.
* [code:: invvpid] Invalidate cached mappings of address translation based on the Virtual Processor ID (VPID).
* [code:: vmcall] Allows a guest in VMX non-root operation to call the VMM for service. A VM exit occurs, transferring control to the VMM.
* [code:: vmfunc] Allows software in VMX non-root operation to invoke a VM function, which is processor functionality enabled and configured by software in VMX root operation. No VM exit occurs.
* [code:: getseccapabilities] Returns the available leaf functions of the GETSEC instruction.
* [code:: getsecenteraccs] Loads an authenticated code chipset module and enters authenticated code execution mode.
* [code:: getsecexitac] Exits authenticated code execution mode.
* [code:: getsecsenter] Establishes a Measured Launched Environment (MLE) which has its dynamic root of trust anchored to a chipset supporting Intel Trusted Execution Technology.
* [code:: getsecsexit] Exits the MLE.
* [code:: getsecparameters] Returns SMX related parameter information.
* [code:: getsecsmcrtl] SMX mode control.
* [code:: getsecwakeup] Wakes up sleeping logical processors inside an MLE.
* [code:: bndmk] Create a LowerBound and an UpperBound in a register.
* [code:: bndcl] Check the address of a memory reference against a LowerBound.
* [code:: bndcu] Check the address of a memory reference against an UpperBound in 1’s complement form.
* [code:: bndcn] Check the address of a memory reference against an UpperBound not in 1’s complement form.
* [code:: bndmov] Copy or load from memory of the LowerBound and UpperBound to a register.
* [code:: bndmov] Store to memory of the LowerBound and UpperBound from a register.
* [code:: bndldx] Load bounds using address translation.
* [code:: bndstx] Store bounds using address translation.
* [code:: clrssbsy] Clear busy bit in a supervisor shadow stack token.
* [code:: incssp] Increment the shadow stack pointer (SSP).
* [code:: rdssp] Read shadow stack point (SSP).
* [code:: rstorssp] Restore a shadow stack pointer (SSP).
* [code:: saveprevssp] Save previous shadow stack pointer (SSP).
* [code:: setssbsy] Set busy bit in a supervisor shadow stack token.
* [code:: wrss] Write to a shadow stack.
* [code:: wruss] Write to a user mode shadow stack.
* [code:: endbr32] Terminate an Indirect Branch in 32-bit and Compatibility Mode.
* [code:: endbr64] Terminate an Indirect Branch in 64-bit Mode.
* [code:: ldtilecfg] Load tile configuration.
* [code:: sttilecfg] Store tile configuration.
* [code:: tdpbf16ps] Dot product of BF16 tiles accumulated into packed single precision tile.
* [code:: tdpbssd] Dot product of signed bytes with dword accumulation.
* [code:: tdpbsud] Dot product of signed/unsigned bytes with dword accumulation.
* [code:: tdpbusd] Dot product of unsigned/signed bytes with dword accumulation.
* [code:: tdpbuud] Dot product of unsigned bytes with dword accumulation.
* [code:: tileloadd] Load data into tile.
* [code:: tileloaddt1] Load data into tile with hint to optimize data caching.
* [code:: tilerelease] Release tile.
* [code:: tilestored] Store tile.
* [code:: tilezero] Zero tile.
* [code:: clui] Clear user interrupt flag.
* [code:: senduipi] Send user interprocessor interrupt.
* [code:: stui] Set user interrupt flag.
* [code:: testui] Determine user interrupt flag.
* [code:: uiret] User-interrupt return.
* [code:: enqcmd] Enqueue command.
* [code:: enqcmds] Enqueue command supervisor.
