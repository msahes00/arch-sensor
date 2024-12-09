---
tags: assembly, moxie
endianness: [big, little]
instruction:
  len: variable
  bits: [16, 32, 48]
---
* [code:: and] Logical and.
* [code:: add] Add, long.
* [code:: ashl] Arithmetic shift left.
* [code:: ashr] Arithmetic shift right.
* [code:: beq] Branch if equal.
* [code:: bge] Branch if greater than or equal.
* [code:: bgeu] Branch if greater than or equal, unsigned.
* [code:: bgt] Branch if greater than.
* [code:: bgtu] Branch if greater than, unsigned.
* [code:: ble] Branch if less than or equal.
* [code:: bleu] Branch if less than or equal, unsigned.
* [code:: blt] Branch if less than.
* [code:: bltu] Branch if less than, unsigned.
* [code:: bne] Branch if not equal.
* [code:: brk] Break.
* [code:: cmp] Compare.
* [code:: dec] Decrement.
* [code:: div] Divide, long.
* [code:: gsr] Get special register.
* [code:: inc] Increment.
* [code:: jmp] Jump.
* [code:: jmpa] Jump to address.
* [code:: jsr] Jump to subroutine.
* [code:: jsra] Jump to subroutine at absolute address.
* [code:: ld.b] Load byte.
* [code:: ld.l] Load long.
* [code:: ld.s] Load short.
* [code:: lda.b] Load absolute, byte.
* [code:: lda.l] Load absolute, long.
* [code:: lda.s] Load absolute, short.
* [code:: ldi.l] Load immediate, long.
* [code:: ldi.b] Load immediate, byte.
* [code:: ldi.s] Load immediate, short.
* [code:: ldo.b] Load offset, byte.
* [code:: ldo.l] Load offset, long.
* [code:: ldo.s] Load offset, short.
* [code:: lshr] Logical shift right.
* [code:: mod] Modulus, long.
* [code:: mov] Move register to register.
* [code:: mul] Multiply.
* [code:: mul.x] Signed multiply, upper word.
* [code:: neg] Negative.
* [code:: nop] Do nothing.
* [code:: not] Logical not.
* [code:: or] Logical or.
* [code:: pop] Pop the 32-bit contents of the top of the stack into $rB and update the stack pointer.
* [code:: push] Push the contents of $rB onto a stack pointed and update the stack pointer.
* [code:: ret] Return from subroutine.
* [code:: sex.b] Sign-extend byte.
* [code:: sex.s] Sign-extend short.
* [code:: ssr] Set special register.
* [code:: st.b] Store byte.
* [code:: st.l] Store long.
* [code:: st.s] Store short.
* [code:: sta.b] Store absolute, byte.
* [code:: sta.l] Store absolute, long.
* [code:: sta.s] Store absolute, short.
* [code:: sto.b] Store offset, byte.
* [code:: sto.l] Store offset, long.
* [code:: sto.s] Store offset, short.
* [code:: sub] Subtract, long.
* [code:: swi] Software interrupt.
* [code:: udiv] Divide unsigned, long.
* [code:: umod] Modulus unsigned, long.
* [code:: umul.x] Unsigned multiply, upper word.
* [code:: xor] Logical exclusive or.
* [code:: zex.b] Zero-extend byte.
* [code:: zex.s] Zero-extend short.