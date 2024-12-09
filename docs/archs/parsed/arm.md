---
tags: [assembly, arm]
endianness: [big, little]
instruction:
  len: fixed
  bits: [16, 32]
---
* [code:: abs] Absolute value.
* [code:: adc] Add with carry.
* [code:: adcs] Add with carry, setting flags.
* [code:: add] Add extended and scaled register.
* [code:: add] Add immediate value.
* [code:: add] Add optionally-shifted register.
* [code:: addg] Add with tag.
* [code:: addpt] Add checked pointer.
* [code:: adds] Add extended and scaled register, setting flags.
* [code:: adds] Add immediate value, setting flags.
* [code:: adds] Add optionally-shifted register, setting flags.
* [code:: adr] Form PC-relative address.
* [code:: adrp] Form PC-relative address to 4KB page.
* [code:: and] Bitwise AND (immediate).
* [code:: and] Bitwise AND (shifted register).
* [code:: ands] Bitwise AND (immediate), setting flags.
* [code:: ands] Bitwise AND (shifted register), setting flags.
* [code:: apas] Associate physical address space: an alias of SYS.
* [code:: asr] Arithmetic shift right (immediate): an alias of SBFM.
* [code:: asr] Arithmetic shift right (register): an alias of ASRV.
* [code:: asrv] Arithmetic shift right variable.
* [code:: at] Address translate: an alias of SYS.
* [code:: autda] Authenticate data address, using key A.
* [code:: autdza] Authenticate data address, using key A.
* [code:: autdb] Authenticate data address, using key B.
* [code:: autdzb] Authenticate data address, using key B.
* [code:: autia] Authenticate instruction address, using key A.
* [code:: autia1716] Authenticate instruction address, using key A.
* [code:: autiasp] Authenticate instruction address, using key A.
* [code:: autiaz] Authenticate instruction address, using key A.
* [code:: autiza] Authenticate instruction address, using key A.
* [code:: autia171615] Authenticate instruction address, using key A.
* [code:: autiasppc] Authenticate return address using key A, using an immediate offset.
* [code:: autiasppcr] Authenticate return address using key A, using a register.
* [code:: autib] Authenticate instruction address, using key B.
* [code:: autib1716] Authenticate instruction address, using key B.
* [code:: autibsp] Authenticate instruction address, using key B.
* [code:: autibz] Authenticate instruction address, using key B.
* [code:: autizb] Authenticate instruction address, using key B.
* [code:: autib171615] Authenticate instruction address, using key B.
* [code:: autibsppc] Authenticate return address using key B, using an immediate offset.
* [code:: autibsppcr] Authenticate return address using key B, using a register.
* [code:: axflag] Convert floating-point condition flags from Arm to external format.
* [code:: b] Branch.
* [code:: beq] Branch conditionally.
* [code:: bne] Branch conditionally.
* [code:: bcs] Branch conditionally.
* [code:: bcc] Branch conditionally.
* [code:: bmi] Branch conditionally.
* [code:: bpl] Branch conditionally.
* [code:: bvs] Branch conditionally.
* [code:: bvc] Branch conditionally.
* [code:: bhi] Branch conditionally.
* [code:: bls] Branch conditionally.
* [code:: bge] Branch conditionally.
* [code:: blt] Branch conditionally.
* [code:: bgt] Branch conditionally.
* [code:: ble] Branch conditionally.
* [code:: bal] Branch conditionally.
* [code:: bnv] Branch conditionally.
* [code:: bceq] Branch consistent conditionally.
* [code:: bcne] Branch consistent conditionally.
* [code:: bccs] Branch consistent conditionally.
* [code:: bccc] Branch consistent conditionally.
* [code:: bcmi] Branch consistent conditionally.
* [code:: bcpl] Branch consistent conditionally.
* [code:: bcvs] Branch consistent conditionally.
* [code:: bcvc] Branch consistent conditionally.
* [code:: bchi] Branch consistent conditionally.
* [code:: bcls] Branch consistent conditionally.
* [code:: bcge] Branch consistent conditionally.
* [code:: bclt] Branch consistent conditionally.
* [code:: bcgt] Branch consistent conditionally.
* [code:: bcle] Branch consistent conditionally.
* [code:: bcal] Branch consistent conditionally.
* [code:: bcnv] Branch consistent conditionally.
* [code:: bfc] Bitfield clear: an alias of BFM.
* [code:: bfi] Bitfield insert: an alias of BFM.
* [code:: bfm] Bitfield move.
* [code:: bfxil] Bitfield extract and insert at low end: an alias of BFM.
* [code:: bic] Bitwise bit clear (shifted register).
* [code:: bics] Bitwise bit clear (shifted register), setting flags.
* [code:: bl] Branch with link.
* [code:: blr] Branch with link to register.
* [code:: blraa] Branch with link to register, with pointer authentication.
* [code:: blraaz] Branch with link to register, with pointer authentication.
* [code:: blrab] Branch with link to register, with pointer authentication.
* [code:: blrabz] Branch with link to register, with pointer authentication.
* [code:: br] Branch to register.
* [code:: braa] Branch to register, with pointer authentication.
* [code:: braaz] Branch to register, with pointer authentication.
* [code:: brab] Branch to register, with pointer authentication.
* [code:: brabz] Branch to register, with pointer authentication.
* [code:: brb] Branch record buffer: an alias of SYS.
* [code:: brk] Breakpoint instruction.
* [code:: bti] Branch target identification.
* [code:: cas] Compare and swap word or doubleword in memory.
* [code:: casa] Compare and swap word or doubleword in memory.
* [code:: casal] Compare and swap word or doubleword in memory.
* [code:: casl] Compare and swap word or doubleword in memory.
* [code:: casb] Compare and swap byte in memory.
* [code:: casab] Compare and swap byte in memory.
* [code:: casalb] Compare and swap byte in memory.
* [code:: caslb] Compare and swap byte in memory.
* [code:: cash] Compare and swap halfword in memory.
* [code:: casah] Compare and swap halfword in memory.
* [code:: casalh] Compare and swap halfword in memory.
* [code:: caslh] Compare and swap halfword in memory.
* [code:: casp] Compare and swap pair of words or doublewords in memory.
* [code:: caspa] Compare and swap pair of words or doublewords in memory.
* [code:: caspal] Compare and swap pair of words or doublewords in memory.
* [code:: caspl] Compare and swap pair of words or doublewords in memory.
* [code:: caspt] Compare and swap pair unprivileged.
* [code:: caspat] Compare and swap pair unprivileged.
* [code:: caspalt] Compare and swap pair unprivileged.
* [code:: casplt] Compare and swap pair unprivileged.
* [code:: cast] Compare and swap unprivileged.
* [code:: casat] Compare and swap unprivileged.
* [code:: casalt] Compare and swap unprivileged.
* [code:: caslt] Compare and swap unprivileged.
* [code:: cbgt] Compare register with immediate and branch.
* [code:: cblt] Compare register with immediate and branch.
* [code:: cbhi] Compare register with immediate and branch.
* [code:: cblo] Compare register with immediate and branch.
* [code:: cbeq] Compare register with immediate and branch.
* [code:: cbne] Compare register with immediate and branch.
* [code:: cbgt] Compare register with immediate and branch.
* [code:: cblt] Compare register with immediate and branch.
* [code:: cbhi] Compare register with immediate and branch.
* [code:: cblo] Compare register with immediate and branch.
* [code:: cbeq] Compare register with immediate and branch.
* [code:: cbne] Compare register with immediate and branch.
* [code:: cbgt] Compare registers and branch.
* [code:: cbge] Compare registers and branch.
* [code:: cbhi] Compare registers and branch.
* [code:: cbhs] Compare registers and branch.
* [code:: cbeq] Compare registers and branch.
* [code:: cbne] Compare registers and branch.
* [code:: cbgt] Compare registers and branch.
* [code:: cbge] Compare registers and branch.
* [code:: cbhi] Compare registers and branch.
* [code:: cbhs] Compare registers and branch.
* [code:: cbeq] Compare registers and branch.
* [code:: cbne] Compare registers and branch.
* [code:: cbbgt] Compare bytes and branch.
* [code:: cbbge] Compare bytes and branch.
* [code:: cbbhi] Compare bytes and branch.
* [code:: cbbhs] Compare bytes and branch.
* [code:: cbbeq] Compare bytes and branch.
* [code:: cbbne] Compare bytes and branch.
* [code:: cbble] Compare signed less than or equal bytes and branch: an alias of CBB<cc>.
* [code:: cbblo] Compare unsigned lower than bytes and branch: an alias of CBB<cc>.
* [code:: cbbls] Compare unsigned lower than or equal bytes and branch: an alias of CBB<cc>.
* [code:: cbblt] Compare signed less than bytes and branch: an alias of CBB<cc>.
* [code:: cbge] Compare signed greater than or equal immediate and branch: an alias of CB<cc> (immediate).
* [code:: cbhgt] Compare halfwords and branch.
* [code:: cbhge] Compare halfwords and branch.
* [code:: cbhhi] Compare halfwords and branch.
* [code:: cbhhs] Compare halfwords and branch.
* [code:: cbheq] Compare halfwords and branch.
* [code:: cbhne] Compare halfwords and branch.
* [code:: cbhle] Compare signed less than or equal halfwords and branch: an alias of CBH<cc>.
* [code:: cbhlo] Compare unsigned lower than halfwords and branch: an alias of CBH<cc>.
* [code:: cbhls] Compare unsigned lower than or equal halfwords and branch: an alias of CBH<cc>.
* [code:: cbhlt] Compare signed less than halfwords and branch: an alias of CBH<cc>.
* [code:: cbhs] Compare unsigned greater than or equal immediate and branch: an alias of CB<cc> (immediate).
* [code:: cble] Compare signed less than or equal immediate and branch: an alias of CB<cc> (immediate).
* [code:: cble] Compare signed less than or equal register and branch: an alias of CB<cc> (register).
* [code:: cblo] Compare unsigned lower than register and branch: an alias of CB<cc> (register).
* [code:: cbls] Compare unsigned lower than or equal immediate and branch: an alias of CB<cc> (immediate).
* [code:: cbls] Compare unsigned lower than or equal register and branch: an alias of CB<cc> (register).
* [code:: cblt] Compare signed less than register and branch: an alias of CB<cc> (register).
* [code:: cbnz] Compare and branch on nonzero.
* [code:: cbz] Compare and branch on zero.
* [code:: ccmn] Conditional compare negative (immediate).
* [code:: ccmn] Conditional compare negative (register).
* [code:: ccmp] Conditional compare (immediate).
* [code:: ccmp] Conditional compare (register).
* [code:: cfinv] Invert carry flag.
* [code:: cfp] Control flow prediction restriction by context: an alias of SYS.
* [code:: chkfeat] Check feature status.
* [code:: cinc] Conditional increment: an alias of CSINC.
* [code:: cinv] Conditional invert: an alias of CSINV.
* [code:: clrbhb] Clear branch history.
* [code:: clrex] Clear exclusive.
* [code:: cls] Count leading sign bits.
* [code:: clz] Count leading zeros.
* [code:: cmn] Compare negative (extended register): an alias of ADDS (extended register).
* [code:: cmn] Compare negative (immediate): an alias of ADDS (immediate).
* [code:: cmn] Compare negative (shifted register): an alias of ADDS (shifted register).
* [code:: cmp] Compare (extended register): an alias of SUBS (extended register).
* [code:: cmp] Compare (immediate): an alias of SUBS (immediate).
* [code:: cmp] Compare (shifted register): an alias of SUBS (shifted register).
* [code:: cmpp] Compare with tag: an alias of SUBPS.
* [code:: cneg] Conditional negate: an alias of CSNEG.
* [code:: cnt] Count bits.
* [code:: cosp] Clear other speculative prediction restriction by context: an alias of SYS.
* [code:: cpp] Cache prefetch prediction restriction by context: an alias of SYS.
* [code:: cpyfp] Memory copy forward-only.
* [code:: cpyfm] Memory copy forward-only.
* [code:: cpyfe] Memory copy forward-only.
* [code:: cpyfpn] Memory copy forward-only, reads and writes non-temporal.
* [code:: cpyfmn] Memory copy forward-only, reads and writes non-temporal.
* [code:: cpyfen] Memory copy forward-only, reads and writes non-temporal.
* [code:: cpyfprn] Memory copy forward-only, reads non-temporal.
* [code:: cpyfmrn] Memory copy forward-only, reads non-temporal.
* [code:: cpyfern] Memory copy forward-only, reads non-temporal.
* [code:: cpyfprt] Memory copy forward-only, reads unprivileged.
* [code:: cpyfmrt] Memory copy forward-only, reads unprivileged.
* [code:: cpyfert] Memory copy forward-only, reads unprivileged.
* [code:: cpyfprtn] Memory copy forward-only, reads unprivileged, reads and writes non-temporal.
* [code:: cpyfmrtn] Memory copy forward-only, reads unprivileged, reads and writes non-temporal.
* [code:: cpyfertn] Memory copy forward-only, reads unprivileged, reads and writes non-temporal.
* [code:: cpyfprtrn] Memory copy forward-only, reads unprivileged and non-temporal.
* [code:: cpyfmrtrn] Memory copy forward-only, reads unprivileged and non-temporal.
* [code:: cpyfertrn] Memory copy forward-only, reads unprivileged and non-temporal.
* [code:: cpyfprtwn] Memory copy forward-only, reads unprivileged, writes non-temporal.
* [code:: cpyfmrtwn] Memory copy forward-only, reads unprivileged, writes non-temporal.
* [code:: cpyfertwn] Memory copy forward-only, reads unprivileged, writes non-temporal.
* [code:: cpyfpt] Memory copy forward-only, reads and writes unprivileged.
* [code:: cpyfmt] Memory copy forward-only, reads and writes unprivileged.
* [code:: cpyfet] Memory copy forward-only, reads and writes unprivileged.
* [code:: cpyfptn] Memory copy forward-only, reads and writes unprivileged and non-temporal.
* [code:: cpyfmtn] Memory copy forward-only, reads and writes unprivileged and non-temporal.
* [code:: cpyfetn] Memory copy forward-only, reads and writes unprivileged and non-temporal.
* [code:: cpyfptrn] Memory copy forward-only, reads and writes unprivileged, reads non-temporal.
* [code:: cpyfmtrn] Memory copy forward-only, reads and writes unprivileged, reads non-temporal.
* [code:: cpyfetrn] Memory copy forward-only, reads and writes unprivileged, reads non-temporal.
* [code:: cpyfptwn] Memory copy forward-only, reads and writes unprivileged, writes non-temporal.
* [code:: cpyfmtwn] Memory copy forward-only, reads and writes unprivileged, writes non-temporal.
* [code:: cpyfetwn] Memory copy forward-only, reads and writes unprivileged, writes non-temporal.
* [code:: cpyfpwn] Memory copy forward-only, writes non-temporal.
* [code:: cpyfmwn] Memory copy forward-only, writes non-temporal.
* [code:: cpyfewn] Memory copy forward-only, writes non-temporal.
* [code:: cpyfpwt] Memory copy forward-only, writes unprivileged.
* [code:: cpyfmwt] Memory copy forward-only, writes unprivileged.
* [code:: cpyfewt] Memory copy forward-only, writes unprivileged.
* [code:: cpyfpwtn] Memory copy forward-only, writes unprivileged, reads and writes non-temporal.
* [code:: cpyfmwtn] Memory copy forward-only, writes unprivileged, reads and writes non-temporal.
* [code:: cpyfewtn] Memory copy forward-only, writes unprivileged, reads and writes non-temporal.
* [code:: cpyfpwtrn] Memory copy forward-only, writes unprivileged, reads non-temporal.
* [code:: cpyfmwtrn] Memory copy forward-only, writes unprivileged, reads non-temporal.
* [code:: cpyfewtrn] Memory copy forward-only, writes unprivileged, reads non-temporal.
* [code:: cpyfpwtwn] Memory copy forward-only, writes unprivileged and non-temporal.
* [code:: cpyfmwtwn] Memory copy forward-only, writes unprivileged and non-temporal.
* [code:: cpyfewtwn] Memory copy forward-only, writes unprivileged and non-temporal.
* [code:: cpyp] Memory copy.
* [code:: cpym] Memory copy.
* [code:: cpye] Memory copy.
* [code:: cpypn] Memory copy, reads and writes non-temporal.
* [code:: cpymn] Memory copy, reads and writes non-temporal.
* [code:: cpyen] Memory copy, reads and writes non-temporal.
* [code:: cpyprn] Memory copy, reads non-temporal.
* [code:: cpymrn] Memory copy, reads non-temporal.
* [code:: cpyern] Memory copy, reads non-temporal.
* [code:: cpyprt] Memory copy, reads unprivileged.
* [code:: cpymrt] Memory copy, reads unprivileged.
* [code:: cpyert] Memory copy, reads unprivileged.
* [code:: cpyprtn] Memory copy, reads unprivileged, reads and writes non-temporal.
* [code:: cpymrtn] Memory copy, reads unprivileged, reads and writes non-temporal.
* [code:: cpyertn] Memory copy, reads unprivileged, reads and writes non-temporal.
* [code:: cpyprtrn] Memory copy, reads unprivileged and non-temporal.
* [code:: cpymrtrn] Memory copy, reads unprivileged and non-temporal.
* [code:: cpyertrn] Memory copy, reads unprivileged and non-temporal.
* [code:: cpyprtwn] Memory copy, reads unprivileged, writes non-temporal.
* [code:: cpymrtwn] Memory copy, reads unprivileged, writes non-temporal.
* [code:: cpyertwn] Memory copy, reads unprivileged, writes non-temporal.
* [code:: cpypt] Memory copy, reads and writes unprivileged.
* [code:: cpymt] Memory copy, reads and writes unprivileged.
* [code:: cpyet] Memory copy, reads and writes unprivileged.
* [code:: cpyptn] Memory copy, reads and writes unprivileged and non-temporal.
* [code:: cpymtn] Memory copy, reads and writes unprivileged and non-temporal.
* [code:: cpyetn] Memory copy, reads and writes unprivileged and non-temporal.
* [code:: cpyptrn] Memory copy, reads and writes unprivileged, reads non-temporal.
* [code:: cpymtrn] Memory copy, reads and writes unprivileged, reads non-temporal.
* [code:: cpyetrn] Memory copy, reads and writes unprivileged, reads non-temporal.
* [code:: cpyptwn] Memory copy, reads and writes unprivileged, writes non-temporal.
* [code:: cpymtwn] Memory copy, reads and writes unprivileged, writes non-temporal.
* [code:: cpyetwn] Memory copy, reads and writes unprivileged, writes non-temporal.
* [code:: cpypwn] Memory copy, writes non-temporal.
* [code:: cpymwn] Memory copy, writes non-temporal.
* [code:: cpyewn] Memory copy, writes non-temporal.
* [code:: cpypwt] Memory copy, writes unprivileged.
* [code:: cpymwt] Memory copy, writes unprivileged.
* [code:: cpyewt] Memory copy, writes unprivileged.
* [code:: cpypwtn] Memory copy, writes unprivileged, reads and writes non-temporal.
* [code:: cpymwtn] Memory copy, writes unprivileged, reads and writes non-temporal.
* [code:: cpyewtn] Memory copy, writes unprivileged, reads and writes non-temporal.
* [code:: cpypwtrn] Memory copy, writes unprivileged, reads non-temporal.
* [code:: cpymwtrn] Memory copy, writes unprivileged, reads non-temporal.
* [code:: cpyewtrn] Memory copy, writes unprivileged, reads non-temporal.
* [code:: cpypwtwn] Memory copy, writes unprivileged and non-temporal.
* [code:: cpymwtwn] Memory copy, writes unprivileged and non-temporal.
* [code:: cpyewtwn] Memory copy, writes unprivileged and non-temporal.
* [code:: crc32b] CRC32 checksum.
* [code:: crc32h] CRC32 checksum.
* [code:: crc32w] CRC32 checksum.
* [code:: crc32x] CRC32 checksum.
* [code:: crc32cb] CRC32C checksum.
* [code:: crc32ch] CRC32C checksum.
* [code:: crc32cw] CRC32C checksum.
* [code:: crc32cx] CRC32C checksum.
* [code:: csdb] Consumption of speculative data barrier.
* [code:: csel] Conditional select.
* [code:: cset] Conditional set: an alias of CSINC.
* [code:: csetm] Conditional set mask: an alias of CSINV.
* [code:: csinc] Conditional select increment.
* [code:: csinv] Conditional select invert.
* [code:: csneg] Conditional select negation.
* [code:: ctz] Count trailing zeros.
* [code:: dc] Data cache operation: an alias of SYS.
* [code:: dcps1] Debug change PE state to EL1.
* [code:: dcps2] Debug change PE state to EL2.
* [code:: dcps3] Debug change PE state to EL3.
* [code:: dgh] Data gathering hint.
* [code:: dmb] Data memory barrier.
* [code:: drps] Debug restore PE state.
* [code:: dsb] Data synchronization barrier.
* [code:: dvp] Data value prediction restriction by context: an alias of SYS.
* [code:: eon] Bitwise exclusive-OR NOT (shifted register).
* [code:: eor] Bitwise exclusive-OR (immediate).
* [code:: eor] Bitwise exclusive-OR (shifted register).
* [code:: eret] Exception return.
* [code:: eretaa] Exception return, with pointer authentication.
* [code:: eretab] Exception return, with pointer authentication.
* [code:: esb] Error synchronization barrier.
* [code:: extr] Extract register.
* [code:: gcsb] Guarded Control Stack barrier.
* [code:: gcspopcx] Guarded Control Stack pop and compare exception return record: an alias of SYS.
* [code:: gcspopm] Guarded Control Stack pop: an alias of SYSL.
* [code:: gcspopx] Guarded Control Stack pop exception return record: an alias of SYS.
* [code:: gcspushm] Guarded Control Stack push: an alias of SYS.
* [code:: gcspushx] Guarded Control Stack push exception return record: an alias of SYS.
* [code:: gcsss1] Guarded Control Stack switch stack 1: an alias of SYS.
* [code:: gcsss2] Guarded Control Stack switch stack 2: an alias of SYSL.
* [code:: gcsstr] Guarded Control Stack store register.
* [code:: gcssttr] Guarded Control Stack store register (unprivileged).
* [code:: gmi] Tag mask insert.
* [code:: hint] Hint instruction.
* [code:: hlt] Halt instruction.
* [code:: hvc] Hypervisor call.
* [code:: ic] Instruction cache operation: an alias of SYS.
* [code:: irg] Insert random tag.
* [code:: isb] Instruction synchronization barrier.
* [code:: ld64b] Single-copy atomic 64-byte Load.
* [code:: ldadd] Atomic add on word or doubleword in memory.
* [code:: ldadda] Atomic add on word or doubleword in memory.
* [code:: ldaddal] Atomic add on word or doubleword in memory.
* [code:: ldaddl] Atomic add on word or doubleword in memory.
* [code:: ldaddb] Atomic add on byte in memory.
* [code:: ldaddab] Atomic add on byte in memory.
* [code:: ldaddalb] Atomic add on byte in memory.
* [code:: ldaddlb] Atomic add on byte in memory.
* [code:: ldaddh] Atomic add on halfword in memory.
* [code:: ldaddah] Atomic add on halfword in memory.
* [code:: ldaddalh] Atomic add on halfword in memory.
* [code:: ldaddlh] Atomic add on halfword in memory.
* [code:: ldapr] Load-acquire RCpc register.
* [code:: ldaprb] Load-acquire RCpc register byte.
* [code:: ldaprh] Load-acquire RCpc register halfword.
* [code:: ldapur] Load-acquire RCpc register (unscaled).
* [code:: ldapurb] Load-acquire RCpc register byte (unscaled).
* [code:: ldapurh] Load-acquire RCpc register halfword (unscaled).
* [code:: ldapursb] Load-acquire RCpc register signed byte (unscaled).
* [code:: ldapursh] Load-acquire RCpc register signed halfword (unscaled).
* [code:: ldapursw] Load-acquire RCpc register signed word (unscaled).
* [code:: ldar] Load-acquire register.
* [code:: ldarb] Load-acquire register byte.
* [code:: ldarh] Load-acquire register halfword.
* [code:: ldatxr] Load-acquire unprivileged exclusive register.
* [code:: ldaxp] Load-acquire exclusive pair of registers.
* [code:: ldaxr] Load-acquire exclusive register.
* [code:: ldaxrb] Load-acquire exclusive register byte.
* [code:: ldaxrh] Load-acquire exclusive register halfword.
* [code:: ldclr] Atomic bit clear on word or doubleword in memory.
* [code:: ldclra] Atomic bit clear on word or doubleword in memory.
* [code:: ldclral] Atomic bit clear on word or doubleword in memory.
* [code:: ldclrl] Atomic bit clear on word or doubleword in memory.
* [code:: ldclrb] Atomic bit clear on byte in memory.
* [code:: ldclrab] Atomic bit clear on byte in memory.
* [code:: ldclralb] Atomic bit clear on byte in memory.
* [code:: ldclrlb] Atomic bit clear on byte in memory.
* [code:: ldclrh] Atomic bit clear on halfword in memory.
* [code:: ldclrah] Atomic bit clear on halfword in memory.
* [code:: ldclralh] Atomic bit clear on halfword in memory.
* [code:: ldclrlh] Atomic bit clear on halfword in memory.
* [code:: ldclrp] Atomic bit clear on quadword in memory.
* [code:: ldclrpa] Atomic bit clear on quadword in memory.
* [code:: ldclrpal] Atomic bit clear on quadword in memory.
* [code:: ldclrpl] Atomic bit clear on quadword in memory.
* [code:: ldeor] Atomic exclusive-OR on word or doubleword in memory.
* [code:: ldeora] Atomic exclusive-OR on word or doubleword in memory.
* [code:: ldeoral] Atomic exclusive-OR on word or doubleword in memory.
* [code:: ldeorl] Atomic exclusive-OR on word or doubleword in memory.
* [code:: ldeorb] Atomic exclusive-OR on byte in memory.
* [code:: ldeorab] Atomic exclusive-OR on byte in memory.
* [code:: ldeoralb] Atomic exclusive-OR on byte in memory.
* [code:: ldeorlb] Atomic exclusive-OR on byte in memory.
* [code:: ldeorh] Atomic exclusive-OR on halfword in memory.
* [code:: ldeorah] Atomic exclusive-OR on halfword in memory.
* [code:: ldeoralh] Atomic exclusive-OR on halfword in memory.
* [code:: ldeorlh] Atomic exclusive-OR on halfword in memory.
* [code:: ldg] Load Allocation Tag.
* [code:: ldgm] Load tag multiple.
* [code:: ldiapp] Load-Acquire RCpc ordered pair of registers.
* [code:: ldlar] Load LOAcquire register.
* [code:: ldlarb] Load LOAcquire register byte.
* [code:: ldlarh] Load LOAcquire register halfword.
* [code:: ldnp] Load pair of registers, with non-temporal hint.
* [code:: ldp] Load pair of registers.
* [code:: ldpsw] Load pair of registers signed word.
* [code:: ldr] Load register (immediate).
* [code:: ldr] Load register (literal).
* [code:: ldr] Load register (register).
* [code:: ldraa] Load register, with pointer authentication.
* [code:: ldrab] Load register, with pointer authentication.
* [code:: ldrb] Load register byte (immediate).
* [code:: ldrb] Load register byte (register).
* [code:: ldrh] Load register halfword (immediate).
* [code:: ldrh] Load register halfword (register).
* [code:: ldrsb] Load register signed byte (immediate).
* [code:: ldrsb] Load register signed byte (register).
* [code:: ldrsh] Load register signed halfword (immediate).
* [code:: ldrsh] Load register signed halfword (register).
* [code:: ldrsw] Load register signed word (immediate).
* [code:: ldrsw] Load register signed word (literal).
* [code:: ldrsw] Load register signed word (register).
* [code:: ldset] Atomic bit set on word or doubleword in memory.
* [code:: ldseta] Atomic bit set on word or doubleword in memory.
* [code:: ldsetal] Atomic bit set on word or doubleword in memory.
* [code:: ldsetl] Atomic bit set on word or doubleword in memory.
* [code:: ldsetb] Atomic bit set on byte in memory.
* [code:: ldsetab] Atomic bit set on byte in memory.
* [code:: ldsetalb] Atomic bit set on byte in memory.
* [code:: ldsetlb] Atomic bit set on byte in memory.
* [code:: ldseth] Atomic bit set on halfword in memory.
* [code:: ldsetah] Atomic bit set on halfword in memory.
* [code:: ldsetalh] Atomic bit set on halfword in memory.
* [code:: ldsetlh] Atomic bit set on halfword in memory.
* [code:: ldsetp] Atomic bit set on quadword in memory.
* [code:: ldsetpa] Atomic bit set on quadword in memory.
* [code:: ldsetpal] Atomic bit set on quadword in memory.
* [code:: ldsetpl] Atomic bit set on quadword in memory.
* [code:: ldsmax] Atomic signed maximum on word or doubleword in memory.
* [code:: ldsmaxa] Atomic signed maximum on word or doubleword in memory.
* [code:: ldsmaxal] Atomic signed maximum on word or doubleword in memory.
* [code:: ldsmaxl] Atomic signed maximum on word or doubleword in memory.
* [code:: ldsmaxb] Atomic signed maximum on byte in memory.
* [code:: ldsmaxab] Atomic signed maximum on byte in memory.
* [code:: ldsmaxalb] Atomic signed maximum on byte in memory.
* [code:: ldsmaxlb] Atomic signed maximum on byte in memory.
* [code:: ldsmaxh] Atomic signed maximum on halfword in memory.
* [code:: ldsmaxah] Atomic signed maximum on halfword in memory.
* [code:: ldsmaxalh] Atomic signed maximum on halfword in memory.
* [code:: ldsmaxlh] Atomic signed maximum on halfword in memory.
* [code:: ldsmin] Atomic signed minimum on word or doubleword in memory.
* [code:: ldsmina] Atomic signed minimum on word or doubleword in memory.
* [code:: ldsminal] Atomic signed minimum on word or doubleword in memory.
* [code:: ldsminl] Atomic signed minimum on word or doubleword in memory.
* [code:: ldsminb] Atomic signed minimum on byte in memory.
* [code:: ldsminab] Atomic signed minimum on byte in memory.
* [code:: ldsminalb] Atomic signed minimum on byte in memory.
* [code:: ldsminlb] Atomic signed minimum on byte in memory.
* [code:: ldsminh] Atomic signed minimum on halfword in memory.
* [code:: ldsminah] Atomic signed minimum on halfword in memory.
* [code:: ldsminalh] Atomic signed minimum on halfword in memory.
* [code:: ldsminlh] Atomic signed minimum on halfword in memory.
* [code:: ldtadd] Atomic add unprivileged.
* [code:: ldtadda] Atomic add unprivileged.
* [code:: ldtaddal] Atomic add unprivileged.
* [code:: ldtaddl] Atomic add unprivileged.
* [code:: ldtclr] Atomic bit clear unprivileged.
* [code:: ldtclra] Atomic bit clear unprivileged.
* [code:: ldtclral] Atomic bit clear unprivileged.
* [code:: ldtclrl] Atomic bit clear unprivileged.
* [code:: ldtnp] Load unprivileged pair of registers, with non-temporal hint.
* [code:: ldtp] Load unprivileged pair of registers.
* [code:: ldtr] Load register (unprivileged).
* [code:: ldtrb] Load register byte (unprivileged).
* [code:: ldtrh] Load register halfword (unprivileged).
* [code:: ldtrsb] Load register signed byte (unprivileged).
* [code:: ldtrsh] Load register signed halfword (unprivileged).
* [code:: ldtrsw] Load register signed word (unprivileged).
* [code:: ldtset] Atomic bit set unprivileged.
* [code:: ldtseta] Atomic bit set unprivileged.
* [code:: ldtsetal] Atomic bit set unprivileged.
* [code:: ldtsetl] Atomic bit set unprivileged.
* [code:: ldtxr] Load unprivileged exclusive register.
* [code:: ldumax] Atomic unsigned maximum on word or doubleword in memory.
* [code:: ldumaxa] Atomic unsigned maximum on word or doubleword in memory.
* [code:: ldumaxal] Atomic unsigned maximum on word or doubleword in memory.
* [code:: ldumaxl] Atomic unsigned maximum on word or doubleword in memory.
* [code:: ldumaxb] Atomic unsigned maximum on byte in memory.
* [code:: ldumaxab] Atomic unsigned maximum on byte in memory.
* [code:: ldumaxalb] Atomic unsigned maximum on byte in memory.
* [code:: ldumaxlb] Atomic unsigned maximum on byte in memory.
* [code:: ldumaxh] Atomic unsigned maximum on halfword in memory.
* [code:: ldumaxah] Atomic unsigned maximum on halfword in memory.
* [code:: ldumaxalh] Atomic unsigned maximum on halfword in memory.
* [code:: ldumaxlh] Atomic unsigned maximum on halfword in memory.
* [code:: ldumin] Atomic unsigned minimum on word or doubleword in memory.
* [code:: ldumina] Atomic unsigned minimum on word or doubleword in memory.
* [code:: lduminal] Atomic unsigned minimum on word or doubleword in memory.
* [code:: lduminl] Atomic unsigned minimum on word or doubleword in memory.
* [code:: lduminb] Atomic unsigned minimum on byte in memory.
* [code:: lduminab] Atomic unsigned minimum on byte in memory.
* [code:: lduminalb] Atomic unsigned minimum on byte in memory.
* [code:: lduminlb] Atomic unsigned minimum on byte in memory.
* [code:: lduminh] Atomic unsigned minimum on halfword in memory.
* [code:: lduminah] Atomic unsigned minimum on halfword in memory.
* [code:: lduminalh] Atomic unsigned minimum on halfword in memory.
* [code:: lduminlh] Atomic unsigned minimum on halfword in memory.
* [code:: ldur] Load register (unscaled).
* [code:: ldurb] Load register byte (unscaled).
* [code:: ldurh] Load register halfword (unscaled).
* [code:: ldursb] Load register signed byte (unscaled).
* [code:: ldursh] Load register signed halfword (unscaled).
* [code:: ldursw] Load register signed word (unscaled).
* [code:: ldxp] Load exclusive pair of registers.
* [code:: ldxr] Load exclusive register.
* [code:: ldxrb] Load exclusive register byte.
* [code:: ldxrh] Load exclusive register halfword.
* [code:: lsl] Logical shift left (immediate): an alias of UBFM.
* [code:: lsl] Logical shift left (register): an alias of LSLV.
* [code:: lslv] Logical shift left variable.
* [code:: lsr] Logical shift right (immediate): an alias of UBFM.
* [code:: lsr] Logical shift right (register): an alias of LSRV.
* [code:: lsrv] Logical shift right variable.
* [code:: madd] Multiply-add.
* [code:: maddpt] Multiply-add checked pointer.
* [code:: mneg] Multiply-negate: an alias of MSUB.
* [code:: mov] Move bitmask immediate value: an alias of ORR (immediate).
* [code:: mov] Move inverted wide immediate value: an alias of MOVN.
* [code:: mov] Move register value: an alias of ORR (shifted register).
* [code:: mov] Move register value to or from SP: an alias of ADD (immediate).
* [code:: mov] Move wide immediate value: an alias of MOVZ.
* [code:: movk] Move wide with keep.
* [code:: movn] Move wide with NOT.
* [code:: movz] Move wide with zero.
* [code:: mrrs] Move System register to two adjacent general-purpose registers.
* [code:: mrs] Move System register to general-purpose register.
* [code:: msr] Move immediate value to special register.
* [code:: msr] Move general-purpose register to System register.
* [code:: msrr] Move two adjacent general-purpose registers to System register.
* [code:: msub] Multiply-subtract.
* [code:: msubpt] Multiply-subtract checked pointer.
* [code:: mul] Multiply: an alias of MADD.
* [code:: mvn] Bitwise NOT: an alias of ORN (shifted register).
* [code:: neg] Negate (shifted register): an alias of SUB (shifted register).
* [code:: negs] Negate, setting flags: an alias of SUBS (shifted register).
* [code:: ngc] Negate with carry: an alias of SBC.
* [code:: ngcs] Negate with carry, setting flags: an alias of SBCS.
* [code:: nop] No operation.
* [code:: orn] Bitwise OR NOT (shifted register).
* [code:: orr] Bitwise OR (immediate).
* [code:: orr] Bitwise OR (shifted register).
* [code:: pacda] Pointer Authentication Code for data address, using key A.
* [code:: pacdza] Pointer Authentication Code for data address, using key A.
* [code:: pacdb] Pointer Authentication Code for data address, using key B.
* [code:: pacdzb] Pointer Authentication Code for data address, using key B.
* [code:: pacga] Pointer Authentication Code, using generic key.
* [code:: pacia] Pointer Authentication Code for instruction address, using key A.
* [code:: pacia1716] Pointer Authentication Code for instruction address, using key A.
* [code:: paciasp] Pointer Authentication Code for instruction address, using key A.
* [code:: paciaz] Pointer Authentication Code for instruction address, using key A.
* [code:: paciza] Pointer Authentication Code for instruction address, using key A.
* [code:: pacia171615] Pointer Authentication Code for instruction address, using key A.
* [code:: paciasppc] Pointer Authentication Code for return address, using key A.
* [code:: pacib] Pointer Authentication Code for instruction address, using key B.
* [code:: pacib1716] Pointer Authentication Code for instruction address, using key B.
* [code:: pacibsp] Pointer Authentication Code for instruction address, using key B.
* [code:: pacibz] Pointer Authentication Code for instruction address, using key B.
* [code:: pacizb] Pointer Authentication Code for instruction address, using key B.
* [code:: pacib171615] Pointer Authentication Code for instruction address, using key B.
* [code:: pacibsppc] Pointer Authentication Code for return address, using key B.
* [code:: pacm] Pointer authentication modifier.
* [code:: pacnbiasppc] Pointer Authentication Code for return address, using key A, not a branch target.
* [code:: pacnbibsppc] Pointer Authentication Code for return address, using key B, not a branch target.
* [code:: prfm] Prefetch memory (immediate).
* [code:: prfm] Prefetch memory (literal).
* [code:: prfm] Prefetch memory (register).
* [code:: prfum] Prefetch memory (unscaled offset).
* [code:: psb] Profiling synchronization barrier.
* [code:: pssbb] Physical speculative store bypass barrier: an alias of DSB.
* [code:: rbit] Reverse bits.
* [code:: rcwcas] Read check write compare and swap doubleword in memory.
* [code:: rcwcasa] Read check write compare and swap doubleword in memory.
* [code:: rcwcasal] Read check write compare and swap doubleword in memory.
* [code:: rcwcasl] Read check write compare and swap doubleword in memory.
* [code:: rcwcasp] Read check write compare and swap quadword in memory.
* [code:: rcwcaspa] Read check write compare and swap quadword in memory.
* [code:: rcwcaspal] Read check write compare and swap quadword in memory.
* [code:: rcwcaspl] Read check write compare and swap quadword in memory.
* [code:: rcwclr] Read check write atomic bit clear on doubleword in memory.
* [code:: rcwclra] Read check write atomic bit clear on doubleword in memory.
* [code:: rcwclral] Read check write atomic bit clear on doubleword in memory.
* [code:: rcwclrl] Read check write atomic bit clear on doubleword in memory.
* [code:: rcwclrp] Read check write atomic bit clear on quadword in memory.
* [code:: rcwclrpa] Read check write atomic bit clear on quadword in memory.
* [code:: rcwclrpal] Read check write atomic bit clear on quadword in memory.
* [code:: rcwclrpl] Read check write atomic bit clear on quadword in memory.
* [code:: rcwscas] Read check write software compare and swap doubleword in memory.
* [code:: rcwscasa] Read check write software compare and swap doubleword in memory.
* [code:: rcwscasal] Read check write software compare and swap doubleword in memory.
* [code:: rcwscasl] Read check write software compare and swap doubleword in memory.
* [code:: rcwscasp] Read check write software compare and swap quadword in memory.
* [code:: rcwscaspa] Read check write software compare and swap quadword in memory.
* [code:: rcwscaspal] Read check write software compare and swap quadword in memory.
* [code:: rcwscaspl] Read check write software compare and swap quadword in memory.
* [code:: rcwsclr] Read check write software atomic bit clear on doubleword in memory.
* [code:: rcwsclra] Read check write software atomic bit clear on doubleword in memory.
* [code:: rcwsclral] Read check write software atomic bit clear on doubleword in memory.
* [code:: rcwsclrl] Read check write software atomic bit clear on doubleword in memory.
* [code:: rcwsclrp] Read check write software atomic bit clear on quadword in memory.
* [code:: rcwsclrpa] Read check write software atomic bit clear on quadword in memory.
* [code:: rcwsclrpal] Read check write software atomic bit clear on quadword in memory.
* [code:: rcwsclrpl] Read check write software atomic bit clear on quadword in memory.
* [code:: rcwset] Read check write atomic bit set on doubleword in memory.
* [code:: rcwseta] Read check write atomic bit set on doubleword in memory.
* [code:: rcwsetal] Read check write atomic bit set on doubleword in memory.
* [code:: rcwsetl] Read check write atomic bit set on doubleword in memory.
* [code:: rcwsetp] Read check write atomic bit set on quadword in memory.
* [code:: rcwsetpa] Read check write atomic bit set on quadword in memory.
* [code:: rcwsetpal] Read check write atomic bit set on quadword in memory.
* [code:: rcwsetpl] Read check write atomic bit set on quadword in memory.
* [code:: rcwsset] Read check write software atomic bit set on doubleword in memory.
* [code:: rcwsseta] Read check write software atomic bit set on doubleword in memory.
* [code:: rcwssetal] Read check write software atomic bit set on doubleword in memory.
* [code:: rcwssetl] Read check write software atomic bit set on doubleword in memory.
* [code:: rcwssetp] Read check write software atomic bit set on quadword in memory.
* [code:: rcwssetpa] Read check write software atomic bit set on quadword in memory.
* [code:: rcwssetpal] Read check write software atomic bit set on quadword in memory.
* [code:: rcwssetpl] Read check write software atomic bit set on quadword in memory.
* [code:: rcwsswp] Read check write software swap doubleword in memory.
* [code:: rcwsswpa] Read check write software swap doubleword in memory.
* [code:: rcwsswpal] Read check write software swap doubleword in memory.
* [code:: rcwsswpl] Read check write software swap doubleword in memory.
* [code:: rcwsswpp] Read check write software swap quadword in memory.
* [code:: rcwsswppa] Read check write software swap quadword in memory.
* [code:: rcwsswppal] Read check write software swap quadword in memory.
* [code:: rcwsswppl] Read check write software swap quadword in memory.
* [code:: rcwswp] Read check write swap doubleword in memory.
* [code:: rcwswpa] Read check write swap doubleword in memory.
* [code:: rcwswpal] Read check write swap doubleword in memory.
* [code:: rcwswpl] Read check write swap doubleword in memory.
* [code:: rcwswpp] Read check write swap quadword in memory.
* [code:: rcwswppa] Read check write swap quadword in memory.
* [code:: rcwswppal] Read check write swap quadword in memory.
* [code:: rcwswppl] Read check write swap quadword in memory.
* [code:: ret] Return from subroutine.
* [code:: retaa] Return from subroutine, with pointer authentication.
* [code:: retab] Return from subroutine, with pointer authentication.
* [code:: retaasppc] Return from subroutine, with enhanced pointer authentication return using an immediate offset.
* [code:: retabsppc] Return from subroutine, with enhanced pointer authentication return using an immediate offset.
* [code:: retaasppcr] Return from subroutine, with enhanced pointer authentication return using a register.
* [code:: retabsppcr] Return from subroutine, with enhanced pointer authentication return using a register.
* [code:: rev] Reverse bytes.
* [code:: rev16] Reverse bytes in 16-bit halfwords.
* [code:: rev32] Reverse bytes in 32-bit words.
* [code:: rev64] Reverse bytes: an alias of REV.
* [code:: rmif] Rotate, mask insert flags.
* [code:: ror] Rotate right (immediate): an alias of EXTR.
* [code:: ror] Rotate right (register): an alias of RORV.
* [code:: rorv] Rotate right variable.
* [code:: rprfm] Range prefetch memory.
* [code:: sb] Speculation barrier.
* [code:: sbc] Subtract with carry.
* [code:: sbcs] Subtract with carry, setting flags.
* [code:: sbfiz] Signed bitfield insert in zeros: an alias of SBFM.
* [code:: sbfm] Signed bitfield move.
* [code:: sbfx] Signed bitfield extract: an alias of SBFM.
* [code:: sdiv] Signed divide.
* [code:: setf8] Evaluation of 8-bit or 16-bit flag values.
* [code:: setf16] Evaluation of 8-bit or 16-bit flag values.
* [code:: setgp] Memory set with tag setting.
* [code:: setgm] Memory set with tag setting.
* [code:: setge] Memory set with tag setting.
* [code:: setgpn] Memory set with tag setting, non-temporal.
* [code:: setgmn] Memory set with tag setting, non-temporal.
* [code:: setgen] Memory set with tag setting, non-temporal.
* [code:: setgpt] Memory set with tag setting, unprivileged.
* [code:: setgmt] Memory set with tag setting, unprivileged.
* [code:: setget] Memory set with tag setting, unprivileged.
* [code:: setgptn] Memory set with tag setting, unprivileged and non-temporal.
* [code:: setgmtn] Memory set with tag setting, unprivileged and non-temporal.
* [code:: setgetn] Memory set with tag setting, unprivileged and non-temporal.
* [code:: setp] Memory set.
* [code:: setm] Memory set.
* [code:: sete] Memory set.
* [code:: setpn] Memory set, non-temporal.
* [code:: setmn] Memory set, non-temporal.
* [code:: seten] Memory set, non-temporal.
* [code:: setpt] Memory set, unprivileged.
* [code:: setmt] Memory set, unprivileged.
* [code:: setet] Memory set, unprivileged.
* [code:: setptn] Memory set, unprivileged and non-temporal.
* [code:: setmtn] Memory set, unprivileged and non-temporal.
* [code:: setetn] Memory set, unprivileged and non-temporal.
* [code:: sev] Send event.
* [code:: sevl] Send event local.
* [code:: smaddl] Signed multiply-add long.
* [code:: smax] Signed maximum (immediate).
* [code:: smax] Signed maximum (register).
* [code:: smc] Secure monitor call.
* [code:: smin] Signed minimum (immediate).
* [code:: smin] Signed minimum (register).
* [code:: smnegl] Signed multiply-negate long: an alias of SMSUBL.
* [code:: smstart] Enables access to Streaming SVE mode and SME architectural state: an alias of MSR (immediate).
* [code:: smstop] Disables access to Streaming SVE mode and SME architectural state: an alias of MSR (immediate).
* [code:: smsubl] Signed multiply-subtract long.
* [code:: smulh] Signed multiply high.
* [code:: smull] Signed multiply long: an alias of SMADDL.
* [code:: ssbb] Speculative store bypass barrier: an alias of DSB.
* [code:: st2g] Store Allocation Tags.
* [code:: st64b] Single-copy atomic 64-byte store without status result.
* [code:: st64bv] Single-copy atomic 64-byte store with status result.
* [code:: st64bv0] Single-copy atomic 64-byte EL0 store with status result.
* [code:: stadd] Atomic add on word or doubleword in memory, without return: an alias of LDADD, LDADDA, LDADDAL, LDADDL.
* [code:: staddl] Atomic add on word or doubleword in memory, without return: an alias of LDADD, LDADDA, LDADDAL, LDADDL.
* [code:: staddb] Atomic add on byte in memory, without return: an alias of LDADDB, LDADDAB, LDADDALB, LDADDLB.
* [code:: staddlb] Atomic add on byte in memory, without return: an alias of LDADDB, LDADDAB, LDADDALB, LDADDLB.
* [code:: staddh] Atomic add on halfword in memory, without return: an alias of LDADDH, LDADDAH, LDADDALH, LDADDLH.
* [code:: staddlh] Atomic add on halfword in memory, without return: an alias of LDADDH, LDADDAH, LDADDALH, LDADDLH.
* [code:: stclr] Atomic bit clear on word or doubleword in memory, without return: an alias of LDCLR, LDCLRA, LDCLRAL, LDCLRL.
* [code:: stclrl] Atomic bit clear on word or doubleword in memory, without return: an alias of LDCLR, LDCLRA, LDCLRAL, LDCLRL.
* [code:: stclrb] Atomic bit clear on byte in memory, without return: an alias of LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB.
* [code:: stclrlb] Atomic bit clear on byte in memory, without return: an alias of LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB.
* [code:: stclrh] Atomic bit clear on halfword in memory, without return: an alias of LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH.
* [code:: stclrlh] Atomic bit clear on halfword in memory, without return: an alias of LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH.
* [code:: steor] Atomic exclusive-OR on word or doubleword in memory, without return: an alias of LDEOR, LDEORA, LDEORAL, LDEORL.
* [code:: steorl] Atomic exclusive-OR on word or doubleword in memory, without return: an alias of LDEOR, LDEORA, LDEORAL, LDEORL.
* [code:: steorb] Atomic exclusive-OR on byte in memory, without return: an alias of LDEORB, LDEORAB, LDEORALB, LDEORLB.
* [code:: steorlb] Atomic exclusive-OR on byte in memory, without return: an alias of LDEORB, LDEORAB, LDEORALB, LDEORLB.
* [code:: steorh] Atomic exclusive-OR on halfword in memory, without return: an alias of LDEORH, LDEORAH, LDEORALH, LDEORLH.
* [code:: steorlh] Atomic exclusive-OR on halfword in memory, without return: an alias of LDEORH, LDEORAH, LDEORALH, LDEORLH.
* [code:: stg] Store Allocation Tag.
* [code:: stgm] Store Allocation Tag multiple.
* [code:: stgp] Store Allocation Tag and pair of registers.
* [code:: stilp] Store-release ordered pair of registers.
* [code:: stllr] Store LORelease register.
* [code:: stllrb] Store LORelease register byte.
* [code:: stllrh] Store LORelease register halfword.
* [code:: stlr] Store-release register.
* [code:: stlrb] Store-release register byte.
* [code:: stlrh] Store-release register halfword.
* [code:: stltxr] Store-release unprivileged exclusive register.
* [code:: stlur] Store-release register (unscaled).
* [code:: stlurb] Store-release register byte (unscaled).
* [code:: stlurh] Store-release register halfword (unscaled).
* [code:: stlxp] Store-release exclusive pair of registers.
* [code:: stlxr] Store-release exclusive register.
* [code:: stlxrb] Store-release exclusive register byte.
* [code:: stlxrh] Store-release exclusive register halfword.
* [code:: stnp] Store pair of registers, with non-temporal hint.
* [code:: stp] Store pair of registers.
* [code:: str] Store register (immediate).
* [code:: str] Store register (register).
* [code:: strb] Store register byte (immediate).
* [code:: strb] Store register byte (register).
* [code:: strh] Store register halfword (immediate).
* [code:: strh] Store register halfword (register).
* [code:: stset] Atomic bit set on word or doubleword in memory, without return: an alias of LDSET, LDSETA, LDSETAL, LDSETL.
* [code:: stsetl] Atomic bit set on word or doubleword in memory, without return: an alias of LDSET, LDSETA, LDSETAL, LDSETL.
* [code:: stsetb] Atomic bit set on byte in memory, without return: an alias of LDSETB, LDSETAB, LDSETALB, LDSETLB.
* [code:: stsetlb] Atomic bit set on byte in memory, without return: an alias of LDSETB, LDSETAB, LDSETALB, LDSETLB.
* [code:: stseth] Atomic bit set on halfword in memory, without return: an alias of LDSETH, LDSETAH, LDSETALH, LDSETLH.
* [code:: stsetlh] Atomic bit set on halfword in memory, without return: an alias of LDSETH, LDSETAH, LDSETALH, LDSETLH.
* [code:: stshh] Store shared hint.
* [code:: stsmax] Atomic signed maximum on word or doubleword in memory, without return: an alias of LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL.
* [code:: stsmaxl] Atomic signed maximum on word or doubleword in memory, without return: an alias of LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL.
* [code:: stsmaxb] Atomic signed maximum on byte in memory, without return: an alias of LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB.
* [code:: stsmaxlb] Atomic signed maximum on byte in memory, without return: an alias of LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB.
* [code:: stsmaxh] Atomic signed maximum on halfword in memory, without return: an alias of LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH.
* [code:: stsmaxlh] Atomic signed maximum on halfword in memory, without return: an alias of LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH.
* [code:: stsmin] Atomic signed minimum on word or doubleword in memory, without return: an alias of LDSMIN, LDSMINA, LDSMINAL, LDSMINL.
* [code:: stsminl] Atomic signed minimum on word or doubleword in memory, without return: an alias of LDSMIN, LDSMINA, LDSMINAL, LDSMINL.
* [code:: stsminb] Atomic signed minimum on byte in memory, without return: an alias of LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB.
* [code:: stsminlb] Atomic signed minimum on byte in memory, without return: an alias of LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB.
* [code:: stsminh] Atomic signed minimum on halfword in memory, without return: an alias of LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH.
* [code:: stsminlh] Atomic signed minimum on halfword in memory, without return: an alias of LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH.
* [code:: sttadd] Atomic add unprivileged, without return: an alias of LDTADD, LDTADDA, LDTADDAL, LDTADDL.
* [code:: sttaddl] Atomic add unprivileged, without return: an alias of LDTADD, LDTADDA, LDTADDAL, LDTADDL.
* [code:: sttclr] Atomic bit clear unprivileged, without return: an alias of LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL.
* [code:: sttclrl] Atomic bit clear unprivileged, without return: an alias of LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL.
* [code:: sttnp] Store unprivileged pair of registers, with non-temporal hint.
* [code:: sttp] Store unprivileged pair of registers.
* [code:: sttr] Store register (unprivileged).
* [code:: sttrb] Store register byte (unprivileged).
* [code:: sttrh] Store register halfword (unprivileged).
* [code:: sttset] Atomic bit set unprivileged, without return: an alias of LDTSET, LDTSETA, LDTSETAL, LDTSETL.
* [code:: sttsetl] Atomic bit set unprivileged, without return: an alias of LDTSET, LDTSETA, LDTSETAL, LDTSETL.
* [code:: sttxr] Store unprivileged exclusive register.
* [code:: stumax] Atomic unsigned maximum on word or doubleword in memory, without return: an alias of LDUMAX, LDUMAXA, LDUMAXAL, LDUMAXL.
* [code:: stumaxl] Atomic unsigned maximum on word or doubleword in memory, without return: an alias of LDUMAX, LDUMAXA, LDUMAXAL, LDUMAXL.
* [code:: stumaxb] Atomic unsigned maximum on byte in memory, without return: an alias of LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB.
* [code:: stumaxlb] Atomic unsigned maximum on byte in memory, without return: an alias of LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB.
* [code:: stumaxh] Atomic unsigned maximum on halfword in memory, without return: an alias of LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH.
* [code:: stumaxlh] Atomic unsigned maximum on halfword in memory, without return: an alias of LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH.
* [code:: stumin] Atomic unsigned minimum on word or doubleword in memory, without return: an alias of LDUMIN, LDUMINA, LDUMINAL, LDUMINL.
* [code:: stuminl] Atomic unsigned minimum on word or doubleword in memory, without return: an alias of LDUMIN, LDUMINA, LDUMINAL, LDUMINL.
* [code:: stuminb] Atomic unsigned minimum on byte in memory, without return: an alias of LDUMINB, LDUMINAB, LDUMINALB, LDUMINLB.
* [code:: stuminlb] Atomic unsigned minimum on byte in memory, without return: an alias of LDUMINB, LDUMINAB, LDUMINALB, LDUMINLB.
* [code:: stuminh] Atomic unsigned minimum on halfword in memory, without return: an alias of LDUMINH, LDUMINAH, LDUMINALH, LDUMINLH.
* [code:: stuminlh] Atomic unsigned minimum on halfword in memory, without return: an alias of LDUMINH, LDUMINAH, LDUMINALH, LDUMINLH.
* [code:: stur] Store register (unscaled).
* [code:: sturb] Store register byte (unscaled).
* [code:: sturh] Store register halfword (unscaled).
* [code:: stxp] Store exclusive pair of registers.
* [code:: stxr] Store exclusive register.
* [code:: stxrb] Store exclusive register byte.
* [code:: stxrh] Store exclusive register halfword.
* [code:: stz2g] Store Allocation Tags, zeroing.
* [code:: stzg] Store Allocation Tag, zeroing.
* [code:: stzgm] Store Allocation Tag and zero multiple.
* [code:: sub] Subtract extended and scaled register.
* [code:: sub] Subtract immediate value.
* [code:: sub] Subtract optionally-shifted register.
* [code:: subg] Subtract with tag.
* [code:: subp] Subtract pointer.
* [code:: subps] Subtract pointer, setting flags.
* [code:: subpt] Subtract checked pointer.
* [code:: subs] Subtract extended and scaled register, setting flags.
* [code:: subs] Subtract immediate value, setting flags.
* [code:: subs] Subtract optionally-shifted register, setting flags.
* [code:: svc] Supervisor call.
* [code:: swp] Swap word or doubleword in memory.
* [code:: swpa] Swap word or doubleword in memory.
* [code:: swpal] Swap word or doubleword in memory.
* [code:: swpl] Swap word or doubleword in memory.
* [code:: swpb] Swap byte in memory.
* [code:: swpab] Swap byte in memory.
* [code:: swpalb] Swap byte in memory.
* [code:: swplb] Swap byte in memory.
* [code:: swph] Swap halfword in memory.
* [code:: swpah] Swap halfword in memory.
* [code:: swpalh] Swap halfword in memory.
* [code:: swplh] Swap halfword in memory.
* [code:: swpp] Swap quadword in memory.
* [code:: swppa] Swap quadword in memory.
* [code:: swppal] Swap quadword in memory.
* [code:: swppl] Swap quadword in memory.
* [code:: swpt] Swap unprivileged.
* [code:: swpta] Swap unprivileged.
* [code:: swptal] Swap unprivileged.
* [code:: swptl] Swap unprivileged.
* [code:: sxtb] Signed extend byte: an alias of SBFM.
* [code:: sxth] Sign extend halfword: an alias of SBFM.
* [code:: sxtw] Sign extend word: an alias of SBFM.
* [code:: sys] System instruction.
* [code:: sysl] System instruction with result.
* [code:: sysp] 128-bit system instruction.
* [code:: tbnz] Test bit and branch if nonzero.
* [code:: tbz] Test bit and branch if zero.
* [code:: tcancel] Cancel current transaction.
* [code:: tcommit] Commit current transaction.
* [code:: tlbi] TLB invalidate operation: an alias of SYS.
* [code:: tlbip] TLB invalidate pair operation: an alias of SYSP.
* [code:: trcit] Trace instrumentation: an alias of SYS.
* [code:: tsb] Trace synchronization barrier.
* [code:: tst] Test bits (immediate): an alias of ANDS (immediate).
* [code:: tst] Test (shifted register): an alias of ANDS (shifted register).
* [code:: tstart] Start transaction.
* [code:: ttest] Test transaction state.
* [code:: ubfiz] Unsigned bitfield insert in zeros: an alias of UBFM.
* [code:: ubfm] Unsigned bitfield move.
* [code:: ubfx] Unsigned bitfield extract: an alias of UBFM.
* [code:: udf] Permanently undefined.
* [code:: udiv] Unsigned divide.
* [code:: umaddl] Unsigned multiply-add long.
* [code:: umax] Unsigned maximum (immediate).
* [code:: umax] Unsigned maximum (register).
* [code:: umin] Unsigned minimum (immediate).
* [code:: umin] Unsigned minimum (register).
* [code:: umnegl] Unsigned multiply-negate long: an alias of UMSUBL.
* [code:: umsubl] Unsigned multiply-subtract long.
* [code:: umulh] Unsigned multiply high.
* [code:: umull] Unsigned multiply long: an alias of UMADDL.
* [code:: uxtb] Unsigned extend byte: an alias of UBFM.
* [code:: uxth] Unsigned extend halfword: an alias of UBFM.
* [code:: wfe] Wait for event.
* [code:: wfet] Wait for event with timeout.
* [code:: wfi] Wait for interrupt.
* [code:: wfit] Wait for interrupt with timeout.
* [code:: xaflag] Convert floating-point condition flags from external format to Arm format.
* [code:: xpacd] Strip Pointer Authentication Code.
* [code:: xpaci] Strip Pointer Authentication Code.
* [code:: xpaclri] Strip Pointer Authentication Code.
* [code:: yield] Yield.
* [code:: adc] Add with Carry (immediate).
* [code:: adcs] Add with Carry (immediate).
* [code:: adc] Add with Carry (register).
* [code:: adcs] Add with Carry (register).
* [code:: adc] Add with Carry (register-shifted register).
* [code:: adcs] Add with Carry (register-shifted register).
* [code:: add] Add to PC: an alias of ADR.
* [code:: add] Add (immediate).
* [code:: adds] Add (immediate).
* [code:: add] Add (register).
* [code:: adds] Add (register).
* [code:: add] Add (register-shifted register).
* [code:: adds] Add (register-shifted register).
* [code:: add] Add to SP (immediate).
* [code:: adds] Add to SP (immediate).
* [code:: add] Add to SP (register).
* [code:: adds] Add to SP (register).
* [code:: adr] Form PC-relative address.
* [code:: and] Bitwise AND (immediate).
* [code:: ands] Bitwise AND (immediate).
* [code:: and] Bitwise AND (register).
* [code:: ands] Bitwise AND (register).
* [code:: and] Bitwise AND (register-shifted register).
* [code:: ands] Bitwise AND (register-shifted register).
* [code:: asr] Arithmetic Shift Right (immediate): an alias of MOV, MOVS (register).
* [code:: asr] Arithmetic Shift Right (register): an alias of MOV, MOVS (register-shifted register).
* [code:: asrs] Arithmetic Shift Right, setting flags (immediate): an alias of MOV, MOVS (register).
* [code:: asrs] Arithmetic Shift Right, setting flags (register): an alias of MOV, MOVS (register-shifted register).
* [code:: b] Branch.
* [code:: bfc] Bit Field Clear.
* [code:: bfi] Bit Field Insert.
* [code:: bic] Bitwise Bit Clear (immediate).
* [code:: bics] Bitwise Bit Clear (immediate).
* [code:: bic] Bitwise Bit Clear (register).
* [code:: bics] Bitwise Bit Clear (register).
* [code:: bic] Bitwise Bit Clear (register-shifted register).
* [code:: bics] Bitwise Bit Clear (register-shifted register).
* [code:: bkpt] Breakpoint.
* [code:: bl] Branch with Link and optional Exchange (immediate).
* [code:: blx] Branch with Link and optional Exchange (immediate).
* [code:: blx] Branch with Link and Exchange (register).
* [code:: bx] Branch and Exchange.
* [code:: bxj] Branch and Exchange, previously Branch and Exchange Jazelle.
* [code:: cbnz] Compare and Branch on Nonzero or Zero.
* [code:: cbz] Compare and Branch on Nonzero or Zero.
* [code:: clrbhb] Clear Branch History.
* [code:: clrex] Clear-Exclusive.
* [code:: clz] Count Leading Zeros.
* [code:: cmn] Compare Negative (immediate).
* [code:: cmn] Compare Negative (register).
* [code:: cmn] Compare Negative (register-shifted register).
* [code:: cmp] Compare (immediate).
* [code:: cmp] Compare (register).
* [code:: cmp] Compare (register-shifted register).
* [code:: cps] Change PE State.
* [code:: cpsid] Change PE State.
* [code:: cpsie] Change PE State.
* [code:: crc32] CRC32.
* [code:: crc32c] CRC32C.
* [code:: csdb] Consumption of Speculative Data Barrier.
* [code:: dbg] Debug hint.
* [code:: dcps1] Debug Change PE State to EL1.
* [code:: dcps2] Debug Change PE State to EL2.
* [code:: dcps3] Debug Change PE State to EL3.
* [code:: dmb] Data Memory Barrier.
* [code:: dsb] Data Synchronization Barrier.
* [code:: eor] Bitwise Exclusive-OR (immediate).
* [code:: eors] Bitwise Exclusive-OR (immediate).
* [code:: eor] Bitwise Exclusive-OR (register).
* [code:: eors] Bitwise Exclusive-OR (register).
* [code:: eor] Bitwise Exclusive-OR (register-shifted register).
* [code:: eors] Bitwise Exclusive-OR (register-shifted register).
* [code:: eret] Exception Return.
* [code:: esb] Error Synchronization Barrier.
* [code:: hlt] Halting Breakpoint.
* [code:: hvc] Hypervisor Call.
* [code:: isb] Instruction Synchronization Barrier.
* [code:: it] If-Then.
* [code:: lda] Load-Acquire Word.
* [code:: ldab] Load-Acquire Byte.
* [code:: ldaex] Load-Acquire Exclusive Word.
* [code:: ldaexb] Load-Acquire Exclusive Byte.
* [code:: ldaexd] Load-Acquire Exclusive Doubleword.
* [code:: ldaexh] Load-Acquire Exclusive Halfword.
* [code:: ldah] Load-Acquire Halfword.
* [code:: ldc] Load data to System register (immediate).
* [code:: ldc] Load data to System register (literal).
* [code:: ldm] Load Multiple (exception return).
* [code:: ldm] Load Multiple (User registers).
* [code:: ldm] Load Multiple (Increment After, Full Descending).
* [code:: ldmia] Load Multiple (Increment After, Full Descending).
* [code:: ldmfd] Load Multiple (Increment After, Full Descending).
* [code:: ldmda] Load Multiple Decrement After (Full Ascending).
* [code:: ldmfa] Load Multiple Decrement After (Full Ascending).
* [code:: ldmdb] Load Multiple Decrement Before (Empty Ascending).
* [code:: ldmea] Load Multiple Decrement Before (Empty Ascending).
* [code:: ldmib] Load Multiple Increment Before (Empty Descending).
* [code:: ldmed] Load Multiple Increment Before (Empty Descending).
* [code:: ldr] Load Register (immediate).
* [code:: ldr] Load Register (literal).
* [code:: ldr] Load Register (register).
* [code:: ldrb] Load Register Byte (immediate).
* [code:: ldrb] Load Register Byte (literal).
* [code:: ldrb] Load Register Byte (register).
* [code:: ldrbt] Load Register Byte Unprivileged.
* [code:: ldrd] Load Register Dual (immediate).
* [code:: ldrd] Load Register Dual (literal).
* [code:: ldrd] Load Register Dual (register).
* [code:: ldrex] Load Register Exclusive.
* [code:: ldrexb] Load Register Exclusive Byte.
* [code:: ldrexd] Load Register Exclusive Doubleword.
* [code:: ldrexh] Load Register Exclusive Halfword.
* [code:: ldrh] Load Register Halfword (immediate).
* [code:: ldrh] Load Register Halfword (literal).
* [code:: ldrh] Load Register Halfword (register).
* [code:: ldrht] Load Register Halfword Unprivileged.
* [code:: ldrsb] Load Register Signed Byte (immediate).
* [code:: ldrsb] Load Register Signed Byte (literal).
* [code:: ldrsb] Load Register Signed Byte (register).
* [code:: ldrsbt] Load Register Signed Byte Unprivileged.
* [code:: ldrsh] Load Register Signed Halfword (immediate).
* [code:: ldrsh] Load Register Signed Halfword (literal).
* [code:: ldrsh] Load Register Signed Halfword (register).
* [code:: ldrsht] Load Register Signed Halfword Unprivileged.
* [code:: ldrt] Load Register Unprivileged.
* [code:: lsl] Logical Shift Left (immediate): an alias of MOV, MOVS (register).
* [code:: lsl] Logical Shift Left (register): an alias of MOV, MOVS (register-shifted register).
* [code:: lsls] Logical Shift Left, setting flags (immediate): an alias of MOV, MOVS (register).
* [code:: lsls] Logical Shift Left, setting flags (register): an alias of MOV, MOVS (register-shifted register).
* [code:: lsr] Logical Shift Right (immediate): an alias of MOV, MOVS (register).
* [code:: lsr] Logical Shift Right (register): an alias of MOV, MOVS (register-shifted register).
* [code:: lsrs] Logical Shift Right, setting flags (immediate): an alias of MOV, MOVS (register).
* [code:: lsrs] Logical Shift Right, setting flags (register): an alias of MOV, MOVS (register-shifted register).
* [code:: mcr] Move to System register from general-purpose register or execute a System instruction.
* [code:: mcrr] Move to System register from two general-purpose registers.
* [code:: mla] Multiply Accumulate.
* [code:: mlas] Multiply Accumulate.
* [code:: mls] Multiply and Subtract.
* [code:: mov] Move (immediate).
* [code:: movs] Move (immediate).
* [code:: mov] Move (register).
* [code:: movs] Move (register).
* [code:: mov] Move (register-shifted register).
* [code:: movs] Move (register-shifted register).
* [code:: movt] Move Top.
* [code:: mrc] Move to general-purpose register from System register.
* [code:: mrrc] Move to two general-purpose registers from System register.
* [code:: mrs] Move Special register to general-purpose register.
* [code:: mrs] Move Banked or Special register to general-purpose register.
* [code:: msr] Move general-purpose register to Banked or Special register.
* [code:: msr] Move immediate value to Special register.
* [code:: msr] Move general-purpose register to Special register.
* [code:: mul] Multiply.
* [code:: muls] Multiply.
* [code:: mvn] Bitwise NOT (immediate).
* [code:: mvns] Bitwise NOT (immediate).
* [code:: mvn] Bitwise NOT (register).
* [code:: mvns] Bitwise NOT (register).
* [code:: mvn] Bitwise NOT (register-shifted register).
* [code:: mvns] Bitwise NOT (register-shifted register).
* [code:: nop] No Operation.
* [code:: orn] Bitwise OR NOT (immediate).
* [code:: orns] Bitwise OR NOT (immediate).
* [code:: orn] Bitwise OR NOT (register).
* [code:: orns] Bitwise OR NOT (register).
* [code:: orr] Bitwise OR (immediate).
* [code:: orrs] Bitwise OR (immediate).
* [code:: orr] Bitwise OR (register).
* [code:: orrs] Bitwise OR (register).
* [code:: orr] Bitwise OR (register-shifted register).
* [code:: orrs] Bitwise OR (register-shifted register).
* [code:: pkhbt] Pack Halfword.
* [code:: pkhtb] Pack Halfword.
* [code:: pld] Preload Data (literal).
* [code:: pld] Preload Data (immediate).
* [code:: pldw] Preload Data (immediate).
* [code:: pld] Preload Data (register).
* [code:: pldw] Preload Data (register).
* [code:: pli] Preload Instruction (immediate, literal).
* [code:: pli] Preload Instruction (register).
* [code:: pop] Pop Multiple Registers from Stack.
* [code:: pop] Pop Multiple Registers from Stack: an alias of LDM, LDMIA, LDMFD.
* [code:: pop] Pop Single Register from Stack: an alias of LDR (immediate).
* [code:: pssbb] Physical Speculative Store Bypass Barrier.
* [code:: push] Push Multiple Registers to Stack.
* [code:: push] Push multiple registers to Stack: an alias of STMDB, STMFD.
* [code:: push] Push Single Register to Stack: an alias of STR (immediate).
* [code:: qadd] Saturating Add.
* [code:: qadd16] Saturating Add 16.
* [code:: qadd8] Saturating Add 8.
* [code:: qasx] Saturating Add and Subtract with Exchange.
* [code:: qdadd] Saturating Double and Add.
* [code:: qdsub] Saturating Double and Subtract.
* [code:: qsax] Saturating Subtract and Add with Exchange.
* [code:: qsub] Saturating Subtract.
* [code:: qsub16] Saturating Subtract 16.
* [code:: qsub8] Saturating Subtract 8.
* [code:: rbit] Reverse Bits.
* [code:: rev] Byte-Reverse Word.
* [code:: rev16] Byte-Reverse Packed Halfword.
* [code:: revsh] Byte-Reverse Signed Halfword.
* [code:: rfe] Return From Exception.
* [code:: rfeda] Return From Exception.
* [code:: rfedb] Return From Exception.
* [code:: rfeia] Return From Exception.
* [code:: rfeib] Return From Exception.
* [code:: ror] Rotate Right (immediate): an alias of MOV, MOVS (register).
* [code:: ror] Rotate Right (register): an alias of MOV, MOVS (register-shifted register).
* [code:: rors] Rotate Right, setting flags (immediate): an alias of MOV, MOVS (register).
* [code:: rors] Rotate Right, setting flags (register): an alias of MOV, MOVS (register-shifted register).
* [code:: rrx] Rotate Right with Extend: an alias of MOV, MOVS (register).
* [code:: rrxs] Rotate Right with Extend, setting flags: an alias of MOV, MOVS (register).
* [code:: rsb] Reverse Subtract (immediate).
* [code:: rsbs] Reverse Subtract (immediate).
* [code:: rsb] Reverse Subtract (register).
* [code:: rsbs] Reverse Subtract (register).
* [code:: rsb] Reverse Subtract (register-shifted register).
* [code:: rsbs] Reverse Subtract (register-shifted register).
* [code:: rsc] Reverse Subtract with Carry (immediate).
* [code:: rscs] Reverse Subtract with Carry (immediate).
* [code:: rsc] Reverse Subtract with Carry (register).
* [code:: rscs] Reverse Subtract with Carry (register).
* [code:: rsc] Reverse Subtract (register-shifted register).
* [code:: rscs] Reverse Subtract (register-shifted register).
* [code:: sadd16] Signed Add 16.
* [code:: sadd8] Signed Add 8.
* [code:: sasx] Signed Add and Subtract with Exchange.
* [code:: sb] Speculation Barrier.
* [code:: sbc] Subtract with Carry (immediate).
* [code:: sbcs] Subtract with Carry (immediate).
* [code:: sbc] Subtract with Carry (register).
* [code:: sbcs] Subtract with Carry (register).
* [code:: sbc] Subtract with Carry (register-shifted register).
* [code:: sbcs] Subtract with Carry (register-shifted register).
* [code:: sbfx] Signed Bit Field Extract.
* [code:: sdiv] Signed Divide.
* [code:: sel] Select Bytes.
* [code:: setend] Set Endianness.
* [code:: setpan] Set Privileged Access Never.
* [code:: sev] Send Event.
* [code:: sevl] Send Event Local.
* [code:: shadd16] Signed Halving Add 16.
* [code:: shadd8] Signed Halving Add 8.
* [code:: shasx] Signed Halving Add and Subtract with Exchange.
* [code:: shsax] Signed Halving Subtract and Add with Exchange.
* [code:: shsub16] Signed Halving Subtract 16.
* [code:: shsub8] Signed Halving Subtract 8.
* [code:: smc] Secure Monitor Call.
* [code:: smlabb] Signed Multiply Accumulate (halfwords).
* [code:: smlabt] Signed Multiply Accumulate (halfwords).
* [code:: smlatb] Signed Multiply Accumulate (halfwords).
* [code:: smlatt] Signed Multiply Accumulate (halfwords).
* [code:: smlad] Signed Multiply Accumulate Dual.
* [code:: smladx] Signed Multiply Accumulate Dual.
* [code:: smlal] Signed Multiply Accumulate Long.
* [code:: smlals] Signed Multiply Accumulate Long.
* [code:: smlalbb] Signed Multiply Accumulate Long (halfwords).
* [code:: smlalbt] Signed Multiply Accumulate Long (halfwords).
* [code:: smlaltb] Signed Multiply Accumulate Long (halfwords).
* [code:: smlaltt] Signed Multiply Accumulate Long (halfwords).
* [code:: smlald] Signed Multiply Accumulate Long Dual.
* [code:: smlaldx] Signed Multiply Accumulate Long Dual.
* [code:: smlawb] Signed Multiply Accumulate (word by halfword).
* [code:: smlawt] Signed Multiply Accumulate (word by halfword).
* [code:: smlsd] Signed Multiply Subtract Dual.
* [code:: smlsdx] Signed Multiply Subtract Dual.
* [code:: smlsld] Signed Multiply Subtract Long Dual.
* [code:: smlsldx] Signed Multiply Subtract Long Dual.
* [code:: smmla] Signed Most Significant Word Multiply Accumulate.
* [code:: smmlar] Signed Most Significant Word Multiply Accumulate.
* [code:: smmls] Signed Most Significant Word Multiply Subtract.
* [code:: smmlsr] Signed Most Significant Word Multiply Subtract.
* [code:: smmul] Signed Most Significant Word Multiply.
* [code:: smmulr] Signed Most Significant Word Multiply.
* [code:: smuad] Signed Dual Multiply Add.
* [code:: smuadx] Signed Dual Multiply Add.
* [code:: smulbb] Signed Multiply (halfwords).
* [code:: smulbt] Signed Multiply (halfwords).
* [code:: smultb] Signed Multiply (halfwords).
* [code:: smultt] Signed Multiply (halfwords).
* [code:: smull] Signed Multiply Long.
* [code:: smulls] Signed Multiply Long.
* [code:: smulwb] Signed Multiply (word by halfword).
* [code:: smulwt] Signed Multiply (word by halfword).
* [code:: smusd] Signed Multiply Subtract Dual.
* [code:: smusdx] Signed Multiply Subtract Dual.
* [code:: srs] Store Return State.
* [code:: srsda] Store Return State.
* [code:: srsdb] Store Return State.
* [code:: srsia] Store Return State.
* [code:: srsib] Store Return State.
* [code:: ssat] Signed Saturate.
* [code:: ssat16] Signed Saturate 16.
* [code:: ssax] Signed Subtract and Add with Exchange.
* [code:: ssbb] Speculative Store Bypass Barrier.
* [code:: ssub16] Signed Subtract 16.
* [code:: ssub8] Signed Subtract 8.
* [code:: stc] Store data to System register.
* [code:: stl] Store-Release Word.
* [code:: stlb] Store-Release Byte.
* [code:: stlex] Store-Release Exclusive Word.
* [code:: stlexb] Store-Release Exclusive Byte.
* [code:: stlexd] Store-Release Exclusive Doubleword.
* [code:: stlexh] Store-Release Exclusive Halfword.
* [code:: stlh] Store-Release Halfword.
* [code:: stm] Store Multiple (User registers).
* [code:: stm] Store Multiple (Increment After, Empty Ascending).
* [code:: stmia] Store Multiple (Increment After, Empty Ascending).
* [code:: stmea] Store Multiple (Increment After, Empty Ascending).
* [code:: stmda] Store Multiple Decrement After (Empty Descending).
* [code:: stmed] Store Multiple Decrement After (Empty Descending).
* [code:: stmdb] Store Multiple Decrement Before (Full Descending).
* [code:: stmfd] Store Multiple Decrement Before (Full Descending).
* [code:: stmib] Store Multiple Increment Before (Full Ascending).
* [code:: stmfa] Store Multiple Increment Before (Full Ascending).
* [code:: str] Store Register (immediate).
* [code:: str] Store Register (register).
* [code:: strb] Store Register Byte (immediate).
* [code:: strb] Store Register Byte (register).
* [code:: strbt] Store Register Byte Unprivileged.
* [code:: strd] Store Register Dual (immediate).
* [code:: strd] Store Register Dual (register).
* [code:: strex] Store Register Exclusive.
* [code:: strexb] Store Register Exclusive Byte.
* [code:: strexd] Store Register Exclusive Doubleword.
* [code:: strexh] Store Register Exclusive Halfword.
* [code:: strh] Store Register Halfword (immediate).
* [code:: strh] Store Register Halfword (register).
* [code:: strht] Store Register Halfword Unprivileged.
* [code:: strt] Store Register Unprivileged.
* [code:: sub] Subtract from PC: an alias of ADR.
* [code:: sub] Subtract (immediate).
* [code:: subs] Subtract (immediate).
* [code:: sub] Subtract (register).
* [code:: subs] Subtract (register).
* [code:: sub] Subtract (register-shifted register).
* [code:: subs] Subtract (register-shifted register).
* [code:: sub] Subtract from SP (immediate).
* [code:: subs] Subtract from SP (immediate).
* [code:: sub] Subtract from SP (register).
* [code:: subs] Subtract from SP (register).
* [code:: svc] Supervisor Call.
* [code:: sxtab] Signed Extend and Add Byte.
* [code:: sxtab16] Signed Extend and Add Byte 16.
* [code:: sxtah] Signed Extend and Add Halfword.
* [code:: sxtb] Signed Extend Byte.
* [code:: sxtb16] Signed Extend Byte 16.
* [code:: sxth] Signed Extend Halfword.
* [code:: tbb] Table Branch Byte or Halfword.
* [code:: tbh] Table Branch Byte or Halfword.
* [code:: teq] Test Equivalence (immediate).
* [code:: teq] Test Equivalence (register).
* [code:: teq] Test Equivalence (register-shifted register).
* [code:: tsbcsync] Trace Synchronization Barrier.
* [code:: tst] Test (immediate).
* [code:: tst] Test (register).
* [code:: tst] Test (register-shifted register).
* [code:: uadd16] Unsigned Add 16.
* [code:: uadd8] Unsigned Add 8.
* [code:: uasx] Unsigned Add and Subtract with Exchange.
* [code:: ubfx] Unsigned Bit Field Extract.
* [code:: udf] Permanently Undefined.
* [code:: udiv] Unsigned Divide.
* [code:: uhadd16] Unsigned Halving Add 16.
* [code:: uhadd8] Unsigned Halving Add 8.
* [code:: uhasx] Unsigned Halving Add and Subtract with Exchange.
* [code:: uhsax] Unsigned Halving Subtract and Add with Exchange.
* [code:: uhsub16] Unsigned Halving Subtract 16.
* [code:: uhsub8] Unsigned Halving Subtract 8.
* [code:: umaal] Unsigned Multiply Accumulate Accumulate Long.
* [code:: umlal] Unsigned Multiply Accumulate Long.
* [code:: umlals] Unsigned Multiply Accumulate Long.
* [code:: umull] Unsigned Multiply Long.
* [code:: umulls] Unsigned Multiply Long.
* [code:: uqadd16] Unsigned Saturating Add 16.
* [code:: uqadd8] Unsigned Saturating Add 8.
* [code:: uqasx] Unsigned Saturating Add and Subtract with Exchange.
* [code:: uqsax] Unsigned Saturating Subtract and Add with Exchange.
* [code:: uqsub16] Unsigned Saturating Subtract 16.
* [code:: uqsub8] Unsigned Saturating Subtract 8.
* [code:: usad8] Unsigned Sum of Absolute Differences.
* [code:: usada8] Unsigned Sum of Absolute Differences and Accumulate.
* [code:: usat] Unsigned Saturate.
* [code:: usat16] Unsigned Saturate 16.
* [code:: usax] Unsigned Subtract and Add with Exchange.
* [code:: usub16] Unsigned Subtract 16.
* [code:: usub8] Unsigned Subtract 8.
* [code:: uxtab] Unsigned Extend and Add Byte.
* [code:: uxtab16] Unsigned Extend and Add Byte 16.
* [code:: uxtah] Unsigned Extend and Add Halfword.
* [code:: uxtb] Unsigned Extend Byte.
* [code:: uxtb16] Unsigned Extend Byte 16.
* [code:: uxth] Unsigned Extend Halfword.
* [code:: wfe] Wait For Event.
* [code:: wfi] Wait For Interrupt.
* [code:: yield] Yield hint.