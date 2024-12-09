---
tags: assembly, avr
endianness: [little]
instruction:
  len: variable
  bits: [16, 32]
---
* [code:: add] Add without Carry
* [code:: adc] Add with Carry
* [code:: adiw] Add Immediate to Word
* [code:: sub] Subtract without Carry
* [code:: subi] Subtract Immediate
* [code:: sbc] Subtract with Carry
* [code:: sbci] Subtract Immediate with Carry
* [code:: sbiw] Subtract Immediate from Word
* [code:: and] Logical AND
* [code:: andi] Logical AND with Immediate
* [code:: or] Logical OR
* [code:: ori] Logical OR with Immediate
* [code:: eor] Exclusive OR
* [code:: com] One's Complement
* [code:: neg] Two's Complement
* [code:: sbr] Set Bit(s) in Register
* [code:: cbr] Clear Bit(s) in Register
* [code:: inc] Increment
* [code:: dec] Decrement
* [code:: tst] Test for Zero or Minus
* [code:: clr] Clear Register
* [code:: ser] Set Register
* [code:: mul] Multiply Unsigned
* [code:: muls] Multiply Signed
* [code:: mulsu] Multiply Signed with Unsigned
* [code:: fmul] Fractional Multiply Unsigned
* [code:: fmuls] Fractional Multiply Signed
* [code:: fmulsu] Fractional Multiply Signed with Unsigned
* [code:: des] Data Encryption
* [code:: rjmp] Relative Jump
* [code:: ijmp] Indirect Jump to (Z)
* [code:: eijmp] Extended Indirect Jump to (Z)
* [code:: jmp] Jump
* [code:: rcall] Relative Call Subroutine
* [code:: icall] Indirect Call to (Z)
* [code:: eicall] Extended Indirect Call to (Z)
* [code:: call] Call Subroutine
* [code:: ret] Subroutine Return
* [code:: reti] Interrupt Return
* [code:: cpse] Compare, skip if Equal
* [code:: cp] Compare
* [code:: cpc] Compare with Carry
* [code:: cpi] Compare with Immediate
* [code:: sbrc] Skip if Bit in Register Cleared
* [code:: sbrs] Skip if Bit in Register Set
* [code:: sbic] Skip if Bit in I/O Register Cleared
* [code:: sbis] Skip if Bit in I/O Register Set
* [code:: brbs] Branch if Status Flag Set
* [code:: brbc] Branch if Status Flag Cleared
* [code:: breq] Branch if Equal
* [code:: brne] Branch if Not Equal
* [code:: brcs] Branch if Carry Set
* [code:: brcc] Branch if Carry Cleared
* [code:: brsh] Branch if Same or Higher
* [code:: brlo] Branch if Lower
* [code:: brmi] Branch if Minus
* [code:: brpl] Branch if Plus
* [code:: brge] Branch if Greater or Equal, Signed
* [code:: brlt] Branch if Less Than, Signed
* [code:: brhs] Branch if Half Carry Flag Set
* [code:: brhc] Branch if Half Carry Flag Cleared
* [code:: brts] Branch if T Bit Set
* [code:: brtc] Branch if T Bit Cleared
* [code:: brvs] Branch if Overflow Flag is Set
* [code:: brvc] Branch if Overflow Flag is Cleared
* [code:: brie] Branch if Interrupt Enabled
* [code:: brid] Branch if Interrupt Disabled
* [code:: mov] Copy Register
* [code:: movw] Copy Register Pair
* [code:: ldi] Load Immediate
* [code:: lds] Load Direct from Data Space
* [code:: ld] Load Indirect
* [code:: ld] Load Indirect and Post
* [code:: ld] Load Indirect and Pre -Decrement
* [code:: ld] Load Indirect
* [code:: ld] Load Indirect and Post
* [code:: ld] Load Indirect and Pre -Decrement
* [code:: ldd] Load Indirect with Di splacement q
* [code:: ld] Load Indirect
* [code:: ld] Load Indirect and Post
* [code:: ld] Load Indirect and Pre -Decrement
* [code:: ldd] Load Indirect with Di splacement q
* [code:: sts] Store Direct to Data Space
* [code:: st] Store Indirect
* [code:: st] Store Indirect , Post -Increment
* [code:: st] Store Indirect and Pre -Decrement
* [code:: st] Store Indirect
* [code:: st] Store Indirect , Post -Increment
* [code:: st] Store Indirect and Pre -Decrement
* [code:: std] Store with Di splacement
* [code:: st] Store Indirect
* [code:: st] Store Indirect , Post -Increment
* [code:: st] Store Indirect and Pre -Decrement
* [code:: std] Store with Di splacement
* [code:: lpm] Load Program Memory
* [code:: lpm] Load Program Memory
* [code:: lpm] Load Program Memory and Post Increment
* [code:: elpm] Extended Load Program Memory
* [code:: elpm] Extended Load Program Memory
* [code:: elpm] Extended Load Program Memory Post -Increment
* [code:: spm] Store Program Memory
* [code:: spm] Store Memory and Post Increment by 2
* [code:: in] In From I/O Location
* [code:: out] Out To I/O Location
* [code:: push] Push Register on Stack
* [code:: pop] Pop Register from Stack
* [code:: xch] Exchange
* [code:: las] Load and Set
* [code:: lac] Load and Clear
* [code:: lat] Load and Toggle
* [code:: lsl] Logical Shift Left
* [code:: lsr] Logical Shift Right
* [code:: rol] Rotate Left Through Carry
* [code:: ror] Rotate Right Through Carry
* [code:: asr] Arithmetic Shift Right
* [code:: swap] Swap Nibbles
* [code:: sbi] Set Bit in I/O Register
* [code:: cbi] Clear Bit in I/O Register
* [code:: bst] Bit Store from Register to T
* [code:: bld] Bit load from T to Register
* [code:: bset] Flag Set
* [code:: bclr] Flag Clear
* [code:: sec] Set Carry
* [code:: clc] Clear Carry
* [code:: sen] Set Negative Flag
* [code:: cln] Clear Negative Flag
* [code:: sez] Set Zero Flag
* [code:: clz] Clear Zero Flag
* [code:: sei] Global Interrupt Enable
* [code:: cli] Global Interrupt Disable
* [code:: ses] Set Sign Bit
* [code:: cls] Clear Sign Bit
* [code:: sev] Set Two's Complement Overflow
* [code:: clv] Clear Two's Complement Overflow
* [code:: set] Set T in SREG
* [code:: clt] Clear T in SREG
* [code:: seh] Set Half Carry Flag in SREG
* [code:: clh] Clear Half Carry Flag in SREG
* [code:: break] Break
* [code:: nop] No Operation
* [code:: sleep] Sleep
* [code:: wdr] Watchdog Reset
