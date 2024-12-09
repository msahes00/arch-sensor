---
tags: assembly, sparc
endianness: [big]
instruction:
  len: fixed
  bits: [32, 64]
---
* [code:: movcc] Move integer register if condition is satisfied
* [code:: movr] Move integer register on contents of integer register
* [code:: fmovs] Floating-point move
* [code:: fmovd] Floating-point move
* [code:: fmovq] Floating-point move
* [code:: fmovscc] Move floating-point register if condition is satisfied
* [code:: fmovdcc] Move floating-point register if condition is satisfied
* [code:: fmovqcc] Move floating-point register if condition is satisfied
* [code:: fmovsr] Move f-p reg. if integer reg. contents satisfy condition
* [code:: fmovdr] Move f-p reg. if integer reg. contents satisfy condition
* [code:: fmovqr] Move f-p reg. if integer reg. contents satisfy condition
* [code:: fsrc2s] Copy source to destination
* [code:: fsrc2d] Copy source to destination
* [code:: fsrc1s] Copy source to destination
* [code:: fsrc1d] Copy source to destination
* [code:: movftoi] Move floating-point register to integer register
* [code:: movitof] Move integer register to floating-point register
* [code:: fitos] Convert 32-bit integer to floating-point
* [code:: fitod] Convert 32-bit integer to floating-point
* [code:: fitoq] Convert 32-bit integer to floating-point
* [code:: fstoi] Convert floating point to integer
* [code:: fdtoi] Convert floating point to integer
* [code:: fqtoi] Convert floating point to integer
* [code:: fstox] Convert floating point to 64-bit integer
* [code:: fdtox] Convert floating point to 64-bit integer
* [code:: fqtox] Convert floating point to 64-bit integer
* [code:: fstos] Convert between floating-point formats
* [code:: fdtod] Convert between floating-point formats
* [code:: fqtoq] Convert between floating-point formats
* [code:: fstos] Convert between floating-point formats
* [code:: fdtod] Convert between floating-point formats
* [code:: fqtoq] Convert between floating-point formats
* [code:: fxtos] Convert 64-bit integer to floating-point
* [code:: fxtod] Convert 64-bit integer to floating-point
* [code:: fxtoq] Convert 64-bit integer to floating-point
* [code:: and] Logical and
* [code:: andcc] Logical and and modify condition codes
* [code:: or] Inclusive-or
* [code:: orcc] Inclusive-or and modify condition codes
* [code:: orncc] Inclusive-or not and modify condition codes
* [code:: xnorcc] Exclusive-nor and modify condition codes
* [code:: xorcc] Exclusive-or and modify condition codes
* [code:: orn] Inclusive-or not
* [code:: xnor] Exclusive-nor
* [code:: xor] Exclusive-or
* [code:: fands] Logical and operation
* [code:: fandd] Logical and operation
* [code:: fandnot1s] Logical and operation with one inverted source
* [code:: fandnot1d] Logical and operation with one inverted source
* [code:: fandnot2s] Logical and operation with one inverted source
* [code:: fandnot2d] Logical and operation with one inverted source
* [code:: fnands] Logical nand operation
* [code:: fnandd] Logical nand operation
* [code:: fnors] Logical nor operation
* [code:: fnord] Logical nor operation
* [code:: fnot1s] Copy negated source
* [code:: fnot1d] Copy negated source
* [code:: fnot2s] Copy negated source
* [code:: fnot2d] Copy negated source
* [code:: fones] One fill
* [code:: fors] Logical or operation
* [code:: foned] One fill
* [code:: ford] Logical or operation
* [code:: fornot1s] Logical or operation with one inverted source
* [code:: fornot1d] Logical or operation with one inverted source
* [code:: fornot2s] Logical or operation with one inverted source
* [code:: fornot2d] Logical or operation with one inverted source
* [code:: fxnors] Logical xnor operation
* [code:: fxors] Logical xor operation
* [code:: fzeros] Zero fill
* [code:: fxnord] Logical xnor operation
* [code:: fxord] Logical xor operation
* [code:: fzerod] Zero fill
* [code:: sll] Shift left logical
* [code:: sllx] Shift left logical, extended
* [code:: sra] Shift right arithmetic
* [code:: srax] Shift right arithmetic, extended
* [code:: srl] Shift right logical
* [code:: srlx] Shift right logical, extended
* [code:: alignaddress_little] Calculate address for misaligned data
* [code:: alignaddress] Calculate address for misaligned data
* [code:: array8] 3-D array addressing instructions
* [code:: array16] 3-D array addressing instructions
* [code:: array32] 3-D array addressing instructions
* [code:: faligndatag] Perform data alignment for misaligned data (using GSR.align)
* [code:: faligndatai] Perform data alignment for misaligned data (using integer register)
* [code:: bicc] Branch on integer condition codes
* [code:: bpcc] Branch on integer condition codes with prediction
* [code:: bpr] Branch on contents of integer register with prediction
* [code:: call] Call and link
* [code:: cbcond] Compare and Branch
* [code:: donep] Return from trap
* [code:: fbfccd] Branch on floating-point condition codes
* [code:: fbpfcc] Branch on floating-point condition codes with prediction
* [code:: illtrap] Illegal instruction
* [code:: jmpl] Jump and link
* [code:: retryp] Return from trap and retry
* [code:: return] Return
* [code:: tcc] Trap on integer condition codes
* [code:: bmask] Set the GSR.mask field
* [code:: bshuffle] Permute bytes as specified by GSR.mask
* [code:: cmask8n] Create GSR.mask from SIMD comparison result
* [code:: cmask16n] Create GSR.mask from SIMD comparison result
* [code:: cmask32n] Create GSR.mask from SIMD comparison result
* [code:: fexpand] Pixel expansion
* [code:: fpack16] Pixel packing
* [code:: fpack32] Pixel packing
* [code:: fpackfix] Pixel packing
* [code:: fpmerge] Pixel merge
* [code:: ldblockfd] Block loads
* [code:: stblockf] Block stores
* [code:: lddf] Load double floating-point
* [code:: lddfapasi] Load double floating-point from alternate space
* [code:: ldf] Load floating-point
* [code:: ldfapasi] Load floating-point from alternate space
* [code:: ldqf] Load quad floating-point
* [code:: ldqfapasi] Load quad floating-point from alternate space
* [code:: ldshortf] Short floating-point loads
* [code:: stdf] Store double floating-point
* [code:: stdfapasi] Store double floating-point into alternate space
* [code:: stf] Store floating-point
* [code:: stfapasi] Store floating-point into alternate space
* [code:: stpartialf] Partial Store instructions
* [code:: stqf] Store quad floating point
* [code:: stqfapasi] Store quad floating-point into alternate space
* [code:: stshortf] Short floating-point stores
* [code:: casapasi] Compare and swap word in alternate space
* [code:: casxapasi] Compare and swap doubleword in alternate space
* [code:: ldstub] Load-store unsigned byte
* [code:: ldstubapasi] Load-store unsigned byte in alternate space
* [code:: swapd] Swap integer register with memory
* [code:: swapad] Swap integer register with memory in alternate space
* [code:: swapapasi] Swap integer register with memory in alternate space
* [code:: ldsb] Load signed byte
* [code:: ldsbapasi] Load signed byte from alternate space
* [code:: ldsh] Load signed halfword
* [code:: ldshapasi] Load signed halfword from alternate space
* [code:: ldsw] Load signed word
* [code:: ldswapasi] Load signed word from alternate space
* [code:: ldtxan] Load integer twin extended word from alternate space
* [code:: ldtwd] Load integer twin word
* [code:: ldtwapasi] Load integer twin word from alternate space
* [code:: ldtwd] Load integer twin word
* [code:: ldtwapasi] Load integer twin word from alternate space
* [code:: ldub] Load unsigned byte
* [code:: ldubapasi] Load unsigned byte from alternate space
* [code:: lduh] Load unsigned halfword
* [code:: lduhapasi] Load unsigned halfword from alternate space
* [code:: lduw] Load unsigned word
* [code:: lduwapasi] Load unsigned word from alternate space
* [code:: ldx] Load extended
* [code:: ldxapasi] Load extended from alternate space
* [code:: stb] Store byte
* [code:: stbapasi] Store byte into alternate space
* [code:: sttwd] Store twin word
* [code:: sttwad] Store twin word into alternate space
* [code:: sttwapasi] Store twin word into alternate space
* [code:: sth] Store halfword
* [code:: sthapasi] Store halfword into alternate space
* [code:: stw] Store word
* [code:: stwapasi] Store word into alternate space
* [code:: stx] Store extended
* [code:: stxapasi] Store extended into alternate space
* [code:: ldfsrd] Load floating-point state register (lower)
* [code:: ldxefsr] Load Entire floating-point state register
* [code:: ldxfsr] Load floating-point state register
* [code:: mwait] Monitor Wait
* [code:: membar] Memory barrier
* [code:: prefetch] Prefetch data
* [code:: prefetchapasi] Prefetch data from alternate space
* [code:: stfsrd] Store floating-point state register (lower)
* [code:: stxfsr] Store floating-point state register
* [code:: fabss] Floating-point absolute value
* [code:: fadds] Floating-point add
* [code:: fdivs] Floating-point divide
* [code:: fabsd] Floating-point absolute value
* [code:: faddd] Floating-point add
* [code:: fdivd] Floating-point divide
* [code:: fabsq] Floating-point absolute value
* [code:: faddq] Floating-point add
* [code:: fdivq] Floating-point divide
* [code:: fdmulq] Floating-point multiply double to quad
* [code:: fhadds] Floating-point add and halve
* [code:: fhsubs] Floating-point subtract and halve
* [code:: fhaddd] Floating-point add and halve
* [code:: fhsubd] Floating-point subtract and halve
* [code:: fmadds] Floating-point multiply-add single/double (fused)
* [code:: fmaddd] Floating-point multiply-add single/double (fused)
* [code:: fmsubs] Floating-point multiply-subtract single/double (fused)
* [code:: fmsubd] Floating-point multiply-subtract single/double (fused)
* [code:: fmuls] Floating-point multiply
* [code:: fmuld] Floating-point multiply
* [code:: fmulq] Floating-point multiply
* [code:: fnadds] Floating-point add and negate
* [code:: fnaddd] Floating-point add and negate
* [code:: fnhadds] Floating-point add, halve, and negate
* [code:: fnhaddd] Floating-point add, halve, and negate
* [code:: fnmadds] Floating-point negative multiply-add single/double (fused)
* [code:: fnmaddd] Floating-point negative multiply-add single/double (fused)
* [code:: fnmuls] Floating-point multiply and negate
* [code:: fnmuld] Floating-point multiply and negate
* [code:: fnegs] Floating-point negate
* [code:: fnegd] Floating-point negate
* [code:: fnegq] Floating-point negate
* [code:: fnmsubs] Floating-point negative multiply-subtract single/double (fused)
* [code:: fnmsubd] Floating-point negative multiply-subtract single/double (fused)
* [code:: fnsmuld] Floating-point multiply single to double, and negate
* [code:: fsmuld] Floating-point multiply single to double
* [code:: fsqrts] Floating-point square root
* [code:: fsubs] Floating-point subtract
* [code:: fcmps] Floating-point compare
* [code:: fcmpes] Floating-point compare (exception if unordered)
* [code:: fsqrtd] Floating-point square root
* [code:: fsubd] Floating-point subtract
* [code:: fcmpd] Floating-point compare
* [code:: fcmped] Floating-point compare (exception if unordered)
* [code:: fsqrtq] Floating-point square root
* [code:: fsubq] Floating-point subtract
* [code:: fcmpq] Floating-point compare
* [code:: fcmpeq] Floating-point compare (exception if unordered)
* [code:: flcmps] Lexicographic compare
* [code:: flcmpd] Lexicographic compare
* [code:: allcleanp] Mark all register window sets as “clean”
* [code:: invalwp] Mark all register window sets as “invalid”
* [code:: flushw] Flush register windows
* [code:: normalwp] “Other” register windows become “normal” register windows
* [code:: otherwp] “Normal” register windows become “other” register windows
* [code:: restore] Restore caller’s window
* [code:: restoredp] Window has been restored
* [code:: save] Save caller’s window
* [code:: savedp] Window has been saved
* [code:: flush] Flush instruction memory
* [code:: nop] No operation
* [code:: add] Add
* [code:: addc] Add with carry
* [code:: addxc] Add with extended carry
* [code:: addcc] Add and modify condition codes
* [code:: addccc] Add with carry and modify condition codes
* [code:: addxccc] Add with extended carry and modify condition codes
* [code:: mulx] Multiply 64-bit integers
* [code:: sdivccd] 32-bit signed integer divide and modify condition codes
* [code:: sdivd] 32-bit signed integer divide
* [code:: sdivx] 64-bit signed integer divide
* [code:: smulccd] Signed integer multiply and modify condition codes
* [code:: smuld] Signed integer multiply
* [code:: subcc] Subtract and modify condition codes
* [code:: sub] Subtract
* [code:: subccc] Subtract with carry and modify condition codes
* [code:: subc] Subtract with carry
* [code:: subxccc] Subtract with extended carry and modify condition codes
* [code:: subxc] Subtract with extended carry
* [code:: taddcc] Tagged add and modify condition codes (trap on overflow)
* [code:: taddcctvd] Tagged add and modify condition codes (trap on overflow)
* [code:: tsubcc] Tagged subtract and modify condition codes (trap on overflow)
* [code:: tsubcctvd] Tagged subtract and modify condition codes (trap on overflow)
* [code:: udivccd] Unsigned integer divide and modify condition codes
* [code:: udivd] Unsigned integer divide
* [code:: udivx] 64-bit unsigned integer divide
* [code:: umulccd] Unsigned integer multiply and modify condition codes
* [code:: umuld] Unsigned integer multiply
* [code:: umulxhi] 64 × 64 multiply yielding upper 64 bits of product
* [code:: xmulx] XOR Multiply
* [code:: xmulxhi] XOR Multiply
* [code:: fmean16] 16-bit partitioned average
* [code:: fpadd] Partitioned integer add
* [code:: fpadds] Partitioned integer add with saturation
* [code:: fpcmp] Partitioned Compare signed integer values
* [code:: fpcmpu] Partitioned Compare unsigned integer values
* [code:: fpmax] Partitioned Integer Maximum
* [code:: fpmaxu] Partitioned Integer Maximum, Unsigned
* [code:: fpmin] Partitioned Integer Minimum
* [code:: fpminu] Partitioned Integer Minimum, Unsigned
* [code:: fpsub16s] Partitioned Integer Subtract
* [code:: fpsub32s] Partitioned Integer Subtract
* [code:: fpsub16] Partitioned Integer Subtract
* [code:: fpsub32] Partitioned Integer Subtract
* [code:: fpsubs] Partitioned Integer Subtract with Saturation
* [code:: fslas16] Partitioned Shift Left Arithmetic, with Saturation
* [code:: fsll16] Partitioned Shift Left Logical, 16- or 32-bit elements
* [code:: fsra16] Partitioned Shift Right Arithmetic, 16- or 32-bit elements
* [code:: fsrl16] Partitioned Shift Right Logical, 16- or 32-bit elements
* [code:: fslas32] Partitioned Shift Left Arithmetic, with Saturation
* [code:: fsll32] Partitioned Shift Left Logical, 16- or 32-bit elements
* [code:: fsra32] Partitioned Shift Right Arithmetic, 16- or 32-bit elements
* [code:: fsrl32] Partitioned Shift Right Logical, 16- or 32-bit elements
* [code:: fsll16] 16- or 32-bit partitioned shift, left or right
* [code:: fsrl16] 16- or 32-bit partitioned shift, left or right
* [code:: fsra16] 16- or 32-bit partitioned shift, left or right
* [code:: fsll32] 16- or 32-bit partitioned shift, left or right
* [code:: fsrl32] 16- or 32-bit partitioned shift, left or right
* [code:: fsra32] 16- or 32-bit partitioned shift, left or right
* [code:: fpadd64] Integer add, 64-bit F registers
* [code:: fmul8x16] 8x16 partitioned product
* [code:: fmul8x16] 8x16 upper/lower α partitioned product
* [code:: fmul8x16] 8x16 upper/lower partitioned product
* [code:: fmuld8x16] 8x16 upper/lower partitioned product
* [code:: fmul8x16au] 8x16 upper/lower α partitioned product
* [code:: fmul8sux16] 8x16 upper/lower partitioned product
* [code:: fmuld8sux16] 8x16 upper/lower partitioned product
* [code:: fmul8x16al] 8x16 upper/lower α partitioned product
* [code:: fmul8ulx16] 8x16 upper/lower partitioned product
* [code:: fmuld8ulx16] 8x16 upper/lower partitioned product
* [code:: fpmaddxhi] 64-bit Integer multiply-add (low and high 64 bit results)
* [code:: fpmaddx] 64-bit Integer multiply-add (low and high 64 bit results)
* [code:: fpsub64] Integer Subtract, 64-bit F registers
* [code:: lzcnt] Leading zeroes count, on 64-bit integer register
* [code:: popc] Population count
* [code:: sethi] Set high 22 bits of low word of integer register
* [code:: edge8lcc] Edge handling instructions (and modify condition codes)
* [code:: edge8ln] Edge handling instructions
* [code:: edge8cc] Edge handling instructions (and modify condition codes)
* [code:: edge8n] Edge handling instructions
* [code:: edge16lcc] Edge handling instructions (and modify condition codes)
* [code:: edge16ln] Edge handling instructions
* [code:: edge16cc] Edge handling instructions (and modify condition codes)
* [code:: edge16n] Edge handling instructions
* [code:: edge32lcc] Edge handling instructions (and modify condition codes)
* [code:: edge32ln] Edge handling instructions
* [code:: edge32cc] Edge handling instructions (and modify condition codes)
* [code:: edge32n] Edge handling instructions
* [code:: fchksm16] 16-bit partitioned checksum
* [code:: pdistd] Pixel component distance
* [code:: pdistn] Distance between eight 8-bit components with no accumulation
* [code:: aes_dround01] AES Decryption, Columns 0&1
* [code:: aes_dround23] AES Decryption, Columns 2&3
* [code:: aes_dround01_last] AES Decryption, Columns 0&1, Last Round
* [code:: aes_dround23_last] AES Decryption, Columns 2&3, Last Round
* [code:: aes_eround01] AES Encryption, Columns 0&1
* [code:: aes_eround23] AES Encryption, Columns 2&3
* [code:: aes_eround01_last] AES Encryption, Columns 0&1, Last Round
* [code:: aes_eround23_last] AES Encryption, Columns 2&3, Last Round
* [code:: aes_kexpand0] AES Key Expansion, without RCON
* [code:: aes_kexpand1] AES Key Expansion, with RCON
* [code:: aes_kexpand2] AES Key Expansion, without SBOX
* [code:: camellia_fn] Camellia “F” operation
* [code:: camellia_fln] Camellia “FL” operation
* [code:: camellia_flin] Camellia “FLI” operation
* [code:: crc32cn] Cyclic Redundancy Check
* [code:: des_ipn] DES Initial Permutation
* [code:: des_iipn] DES Inverse Initial Permutation
* [code:: des_kexpandn] DES Key Expand
* [code:: des_roundn] DES Round Operations
* [code:: md5n] MD5 Hash Operation
* [code:: mpmuln] Multiple-Precision Multiply
* [code:: montmuln] Montgomery Multiplication
* [code:: montsqrn] Montgomery Squaring
* [code:: sha1n] SHA1 Secure Hash operation
* [code:: sha256n] SHA256 Secure Hash operation
* [code:: sha512n] SHA512 Secure Hash operation
* [code:: xmontmuln] Montgomery XOR Multiplication
* [code:: xmontsqrn] Montgomery XOR Squaring
* [code:: xmpmuln] Multiple-Precision XOR Multiply
* [code:: pause] Pause Virtual Processor
* [code:: rdasi] Read ASI register
* [code:: rdasrpasr] Read ancillary state register
* [code:: rdccr] Read Condition Codes register (CCR)
* [code:: rdfprs] Read Floating-Point Registers State register (FPRS)
* [code:: rdgsr] Read General Status register (GSR)
* [code:: rdpc] Read Program Counter register (PC)
* [code:: rdprp] Read privileged register
* [code:: rdsoftintp] Read per-virtual processor Soft Interrupt register (SOFTINT)
* [code:: rdstickpdis] Read System Tick register (STICK)
* [code:: rdstickhdis] Read System Tick register (STICK)
* [code:: rdstick_cmprp] Read System Tick Compare register (STICK_CMPR)
* [code:: rdtickpdis] Read Tick register (TICK)
* [code:: rdtickhdis] Read Tick register (TICK)
* [code:: rdyd] Read Y register
* [code:: siam] Set interval arithmetic mode
* [code:: wrasi] Write ASI register
* [code:: wrasrpasr] Write ancillary state register
* [code:: wrccr] Write Condition Codes register (CCR)
* [code:: wrfprs] Write Floating-Point Registers State register (FPRS)
* [code:: wrgsr] Write General Status register (GSR)
* [code:: wrprp] Write privileged register
* [code:: wrsoftintp] Write per-virtual processor Soft Interrupt register (SOFTINT)
* [code:: wrsoftint_clrp] Clear bits of per-virtual processor Soft Interrupt register (SOFTINT)
* [code:: wrsoftint_setp] Set bits of per-virtual processor Soft Interrupt register (SOFTINT)
* [code:: wrstickp] Write System Tick register (STICK)
* [code:: wrstick_cmprp] Write System Tick Compare register (STICK_CMPR)
* [code:: wryd] Write Y register