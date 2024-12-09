---
tags: assembly, powerpc
endianness: [big, little]
instruction:
  len: fixed
  bits: [32, 64]
---
* [code:: tdi] Trap Doubleword Immediate D-form
* [code:: twi] Trap Word Immediate D-form
* [code:: vaddubm] Vector Add Unsigned Byte Modulo VX-form
* [code:: vadduhm] Vector Add Unsigned Halfword Modulo VX-form
* [code:: vadduwm] Vector Add Unsigned Word Modulo VX-form
* [code:: vaddudm] Vector Add Unsigned Doubleword Modulo VX-form
* [code:: vadduqm] Vector Add Unsigned Quadword Modulo VX-form
* [code:: vaddcuq] Vector Add & write Carry-out Unsigned Quadword VX-form
* [code:: vaddcuw] Vector Add & write Carry-out Unsigned Word VX-form
* [code:: vaddubs] Vector Add Unsigned Byte Saturate VX-form
* [code:: vadduhs] Vector Add Unsigned Halfword Saturate VX-form
* [code:: vadduws] Vector Add Unsigned Word Saturate VX-form
* [code:: vaddsbs] Vector Add Signed Byte Saturate VX-form
* [code:: vaddshs] Vector Add Signed Halfword Saturate VX-form
* [code:: vaddsws] Vector Add Signed Word Saturate VX-form
* [code:: vsububm] Vector Subtract Unsigned Byte Modulo VX-form
* [code:: vsubuhm] Vector Subtract Unsigned Halfword Modulo VX-form
* [code:: vsubuwm] Vector Subtract Unsigned Word Modulo VX-form
* [code:: vsubudm] Vector Subtract Unsigned Doubleword Modulo VX-form
* [code:: vsubuqm] Vector Subtract Unsigned Quadword Modulo VX-form
* [code:: vsubcuq] Vector Subtract & write Carry-out Unsigned Quadword VX-form
* [code:: vsubcuw] Vector Subtract & Write Carry-out Unsigned Word VX-form
* [code:: vsububs] Vector Subtract Unsigned Byte Saturate VX-form
* [code:: vsubuhs] Vector Subtract Unsigned Halfword Saturate VX-form
* [code:: vsubuws] Vector Subtract Unsigned Word Saturate VX-form
* [code:: vsubsbs] Vector Subtract Signed Byte Saturate VX-form
* [code:: vsubshs] Vector Subtract Signed Halfword Saturate VX-form
* [code:: vsubsws] Vector Subtract Signed Word Saturate VX-form
* [code:: vmul10cuq] Vector Multiply-by-10 & write Carry-out Unsigned Quadword VX-form
* [code:: vmul10ecuq] Vector Multiply-by-10 Extended & write Carry-out Unsigned Quadword VX-form
* [code:: vcmpuq] Vector Compare Unsigned Quadword VX-form
* [code:: vcmpsq] Vector Compare Signed Quadword VX-form
* [code:: vmul10uq] Vector Multiply-by-10 Unsigned Quadword VX-form
* [code:: vmul10euq] Vector Multiply-by-10 Extended Unsigned Quadword VX-form
* [code:: bcdcpsgn.] Decimal Copy Sign VX-form
* [code:: bcdadd.] Decimal Add Modulo VX-form
* [code:: bcdsub.] Decimal Subtract Modulo VX-form
* [code:: bcdus.] Decimal Unsigned Shift VX-form
* [code:: bcds.] Decimal Shift VX-form
* [code:: bcdtrunc.] Decimal Truncate VX-form
* [code:: bcdutrunc.] Decimal Unsigned Truncate VX-form
* [code:: bcdctsq.] Decimal Convert To Signed Quadword VX-form
* [code:: bcdcfsq.] Decimal Convert From Signed Quadword VX-form
* [code:: bcdctz.] Decimal Convert To Zoned VX-form
* [code:: bcdctn.] Decimal Convert To National VX-form
* [code:: bcdcfz.] Decimal Convert From Zoned VX-form
* [code:: bcdcfn.] Decimal Convert From National VX-form
* [code:: bcdsetsgn.] Decimal Set Sign VX-form
* [code:: bcdsr.] Decimal Shift & Round VX-form
* [code:: vmaxub] Vector Maximum Unsigned Byte VX-form
* [code:: vmaxuh] Vector Maximum Unsigned Halfword VX-form
* [code:: vmaxuw] Vector Maximum Unsigned Word VX-form
* [code:: vmaxud] Vector Maximum Unsigned Doubleword VX-form
* [code:: vmaxsb] Vector Maximum Signed Byte VX-form
* [code:: vmaxsh] Vector Maximum Signed Halfword VX-form
* [code:: vmaxsw] Vector Maximum Signed Word VX-form
* [code:: vmaxsd] Vector Maximum Signed Doubleword VX-form
* [code:: vminub] Vector Minimum Unsigned Byte VX-form
* [code:: vminuh] Vector Minimum Unsigned Halfword VX-form
* [code:: vminuw] Vector Minimum Unsigned Word VX-form
* [code:: vminud] Vector Minimum Unsigned Doubleword VX-form
* [code:: vminsb] Vector Minimum Signed Byte VX-form
* [code:: vminsh] Vector Minimum Signed Halfword VX-form
* [code:: vminsw] Vector Minimum Signed Word VX-form
* [code:: vminsd] Vector Minimum Signed Doubleword VX-form
* [code:: vavgub] Vector Average Unsigned Byte VX-form
* [code:: vavguh] Vector Average Unsigned Halfword VX-form
* [code:: vavguw] Vector Average Unsigned Word VX-form
* [code:: vavgsb] Vector Average Signed Byte VX-form
* [code:: vavgsh] Vector Average Signed Halfword VX-form
* [code:: vavgsw] Vector Average Signed Word VX-form
* [code:: vclzlsbb] Vector Count Leading Zero Least-Significant Bits Byte VX-form
* [code:: vctzlsbb] Vector Count Trailing Zero Least-Significant Bits Byte VX-form
* [code:: vnegw] Vector Negate Word VX-form
* [code:: vnegd] Vector Negate Doubleword VX-form
* [code:: vprtybw] Vector Parity Byte Word VX-form
* [code:: vprtybd] Vector Parity Byte Doubleword VX-form
* [code:: vprtybq] Vector Parity Byte Quadword VX-form
* [code:: vextsb2w] Vector Extend Sign Byte To Word VX-form
* [code:: vextsh2w] Vector Extend Sign Halfword To Word VX-form
* [code:: vextsb2d] Vector Extend Sign Byte To Doubleword VX-form
* [code:: vextsh2d] Vector Extend Sign Halfword To Doubleword VX-form
* [code:: vextsw2d] Vector Extend Sign Word To Doubleword VX-form
* [code:: vextsd2q] Vector Extend Sign Doubleword to Quadword VX-form
* [code:: vctzb] Vector Count Trailing Zeros Byte VX-form
* [code:: vctzh] Vector Count Trailing Zeros Halfword VX-form
* [code:: vctzw] Vector Count Trailing Zeros Word VX-form
* [code:: vctzd] Vector Count Trailing Zeros Doubleword VX-form
* [code:: vexpandbm] Vector Expand Byte Mask VX-form
* [code:: vexpandhm] Vector Expand Halfword Mask VX-form
* [code:: vexpandwm] Vector Expand Word Mask VX-form
* [code:: vexpanddm] Vector Expand Doubleword Mask VX-form
* [code:: vexpandqm] Vector Expand Quadword Mask VX-form
* [code:: vextractbm] Vector Extract Byte Mask VX-form
* [code:: vextracthm] Vector Extract Halfword Mask VX-form
* [code:: vextractwm] Vector Extract Word Mask VX-form
* [code:: vextractdm] Vector Extract Doubleword Mask VX-form
* [code:: vextractqm] Vector Extract Quadword Mask VX-form
* [code:: mtvsrbm] Move to VSR Byte Mask VX-form
* [code:: mtvsrhm] Move to VSR Halfword Mask VX-form
* [code:: mtvsrwm] Move to VSR Word Mask VX-form
* [code:: mtvsrdm] Move to VSR Doubleword Mask VX-form
* [code:: mtvsrqm] Move to VSR Quadword Mask VX-form
* [code:: vcntmbb] Vector Count Mask Bits Byte VX-form
* [code:: vcntmbh] Vector Count Mask Bits Halfword VX-form
* [code:: vcntmbw] Vector Count Mask Bits Word VX-form
* [code:: vcntmbd] Vector Count Mask Bits Doubleword VX-form
* [code:: vshasigmaw] Vector SHA-256 Sigma Word VX-form
* [code:: vshasigmad] Vector SHA-512 Sigma Doubleword VX-form
* [code:: vclzb] Vector Count Leading Zeros Byte VX-form
* [code:: vclzh] Vector Count Leading Zeros Halfword VX-form
* [code:: vclzw] Vector Count Leading Zeros Word VX-form
* [code:: vclzd] Vector Count Leading Zeros Doubleword VX-form
* [code:: vabsdub] Vector Absolute Difference Unsigned Byte VX-form
* [code:: vabsduh] Vector Absolute Difference Unsigned Halfword VX-form
* [code:: vabsduw] Vector Absolute Difference Unsigned Word VX-form
* [code:: vpopcntb] Vector Population Count Byte VX-form
* [code:: vpopcnth] Vector Population Count Halfword VX-form
* [code:: vpopcntw] Vector Population Count Word VX-form
* [code:: vpopcntd] Vector Population Count Doubleword VX-form
* [code:: vrlb] Vector Rotate Left Byte VX-form
* [code:: vrlh] Vector Rotate Left Halfword VX-form
* [code:: vrlw] Vector Rotate Left Word VX-form
* [code:: vrld] Vector Rotate Left Doubleword VX-form
* [code:: vslb] Vector Shift Left Byte VX-form
* [code:: vslh] Vector Shift Left Halfword VX-form
* [code:: vslw] Vector Shift Left Word VX-form
* [code:: vsl] Vector Shift Left VX-form
* [code:: vsrb] Vector Shift Right Byte VX-form
* [code:: vsrh] Vector Shift Right Halfword VX-form
* [code:: vsrw] Vector Shift Right Word VX-form
* [code:: vsr] Vector Shift Right VX-form
* [code:: vsrab] Vector Shift Right Algebraic Byte VX-form
* [code:: vsrah] Vector Shift Right Algebraic Halfword VX-form
* [code:: vsraw] Vector Shift Right Algebraic Word VX-form
* [code:: vsrad] Vector Shift Right Algebraic Doubleword VX-form
* [code:: vand] Vector Logical AND VX-form
* [code:: vandc] Vector Logical AND with Complement VX-form
* [code:: vor] Vector Logical OR VX-form
* [code:: vxor] Vector Logical XOR VX-form
* [code:: vnor] Vector Logical NOR VX-form
* [code:: vorc] Vector Logical OR with Complement VX-form
* [code:: vnand] Vector Logical NAND VX-form
* [code:: vsld] Vector Shift Left Doubleword VX-form
* [code:: mfvscr] Move From Vector Status and Control Register VX-form
* [code:: mtvscr] Move To Vector Status and Control Register VX-form
* [code:: veqv] Vector Logical Equivalence VX-form
* [code:: vsrd] Vector Shift Right Doubleword VX-form
* [code:: vsrv] Vector Shift Right Variable VX-form
* [code:: vslv] Vector Shift Left Variable VX-form
* [code:: vclzdm] Vector Count Leading Zeros Doubleword under bit Mask VX-form
* [code:: vctzdm] Vector Count Trailing Zeros Doubleword under bit Mask VX-form
* [code:: vrlq] Vector Rotate Left Quadword VX-form
* [code:: vrlqmi] Vector Rotate Left Quadword then Mask Insert VX-form
* [code:: vrlwmi] Vector Rotate Left Word then Mask Insert VX-form
* [code:: vrldmi] Vector Rotate Left Doubleword then Mask Insert VX-form
* [code:: vslq] Vector Shift Left Quadword VX-form
* [code:: vrlqnm] Vector Rotate Left Quadword then AND with Mask VX-form
* [code:: vrlwnm] Vector Rotate Left Word then AND with Mask VX-form
* [code:: vrldnm] Vector Rotate Left Doubleword then AND with Mask VX-form
* [code:: vsrq] Vector Shift Right Quadword VX-form
* [code:: vsraq] Vector Shift Right Algebraic Quadword VX-form
* [code:: vcmpequb.] Vector Compare Equal Unsigned Byte VC-form
* [code:: vcmpequb] Vector Compare Equal Unsigned Byte VC-form
* [code:: vcmpequb.] Vector Compare Equal Unsigned Byte VC-form
* [code:: vcmpequb] Vector Compare Equal Unsigned Byte VC-form
* [code:: vcmpequh.] Vector Compare Equal Unsigned Halfword VC-form
* [code:: vcmpequh] Vector Compare Equal Unsigned Halfword VC-form
* [code:: vcmpequw.] Vector Compare Equal Unsigned Word VC-form
* [code:: vcmpequw] Vector Compare Equal Unsigned Word VC-form
* [code:: vcmpeqfp.] Vector Compare Equal Floating-Point VC-form
* [code:: vcmpeqfp] Vector Compare Equal Floating-Point VC-form
* [code:: vcmpgefp.] Vector Compare Greater Than or Equal Floating-Point VC-form
* [code:: vcmpgefp] Vector Compare Greater Than or Equal Floating-Point VC-form
* [code:: vcmpgtub.] Vector Compare Greater Than Unsigned Byte VC-form
* [code:: vcmpgtub] Vector Compare Greater Than Unsigned Byte VC-form
* [code:: vcmpgtuh.] Vector Compare Greater Than Unsigned Halfword VC-form
* [code:: vcmpgtuh] Vector Compare Greater Than Unsigned Halfword VC-form
* [code:: vcmpgtuw.] Vector Compare Greater Than Unsigned Word VC-form
* [code:: vcmpgtuw] Vector Compare Greater Than Unsigned Word VC-form
* [code:: vcmpgtfp.] Vector Compare Greater Than Floating-Point VC-form
* [code:: vcmpgtfp] Vector Compare Greater Than Floating-Point VC-form
* [code:: vcmpgtsb.] Vector Compare Greater Than Signed Byte VC-form
* [code:: vcmpgtsb] Vector Compare Greater Than Signed Byte VC-form
* [code:: vcmpgtsh.] Vector Compare Greater Than Signed Halfword VC-form
* [code:: vcmpgtsh] Vector Compare Greater Than Signed Halfword VC-form
* [code:: vcmpgtsw.] Vector Compare Greater Than Signed Word VC-form
* [code:: vcmpgtsw] Vector Compare Greater Than Signed Word VC-form
* [code:: vcmpbfp.] Vector Compare Bounds Floating-Point VC-form
* [code:: vcmpbfp] Vector Compare Bounds Floating-Point VC-form
* [code:: vcmpneb.] Vector Compare Not Equal Byte VC-form
* [code:: vcmpneb] Vector Compare Not Equal Byte VC-form
* [code:: vcmpneh.] Vector Compare Not Equal Halfword VC-form
* [code:: vcmpneh] Vector Compare Not Equal Halfword VC-form
* [code:: vcmpnew.] Vector Compare Not Equal Word VC-form
* [code:: vcmpnew] Vector Compare Not Equal Word VC-form
* [code:: vcmpequd.] Vector Compare Equal Unsigned Doubleword VC-form
* [code:: vcmpequd] Vector Compare Equal Unsigned Doubleword VC-form
* [code:: vcmpnezb.] Vector Compare Not Equal or Zero Byte VC-form
* [code:: vcmpnezb] Vector Compare Not Equal or Zero Byte VC-form
* [code:: vcmpnezh.] Vector Compare Not Equal or Zero Halfword VC-form
* [code:: vcmpnezh] Vector Compare Not Equal or Zero Halfword VC-form
* [code:: vcmpnezw.] Vector Compare Not Equal or Zero Word VC-form
* [code:: vcmpnezw] Vector Compare Not Equal or Zero Word VC-form
* [code:: vcmpequq.] Vector Compare Equal Quadword VC-form
* [code:: vcmpequq] Vector Compare Equal Quadword VC-form
* [code:: vcmpgtuq.] Vector Compare Greater Than Unsigned Quadword VC-form
* [code:: vcmpgtuq] Vector Compare Greater Than Unsigned Quadword VC-form
* [code:: vcmpgtud.] Vector Compare Greater Than Unsigned Doubleword VC-form
* [code:: vcmpgtud] Vector Compare Greater Than Unsigned Doubleword VC-form
* [code:: vcmpgtsq.] Vector Compare Greater Than Signed Quadword VC-form
* [code:: vcmpgtsq] Vector Compare Greater Than Signed Quadword VC-form
* [code:: vcmpgtsd.] Vector Compare Greater Than Signed Doubleword VC-form
* [code:: vcmpgtsd] Vector Compare Greater Than Signed Doubleword VC-form
* [code:: vmuloub] Vector Multiply Odd Unsigned Byte VX-form
* [code:: vmulouh] Vector Multiply Odd Unsigned Halfword VX-form
* [code:: vmulouw] Vector Multiply Odd Unsigned Word VX-form
* [code:: vmuloud] Vector Multiply Odd Unsigned Doubleword VX-form
* [code:: vmulosb] Vector Multiply Odd Signed Byte VX-form
* [code:: vmulosh] Vector Multiply Odd Signed Halfword VX-form
* [code:: vmulosw] Vector Multiply Odd Signed Word VX-form
* [code:: vmulosd] Vector Multiply Odd Signed Doubleword VX-form
* [code:: vmuleub] Vector Multiply Even Unsigned Byte VX-form
* [code:: vmuleuh] Vector Multiply Even Unsigned Halfword VX-form
* [code:: vmuleuw] Vector Multiply Even Unsigned Word VX-form
* [code:: vmuleud] Vector Multiply Even Unsigned Doubleword VX-form
* [code:: vmulesb] Vector Multiply Even Signed Byte VX-form
* [code:: vmulesh] Vector Multiply Even Signed Halfword VX-form
* [code:: vmulesw] Vector Multiply Even Signed Word VX-form
* [code:: vmulesd] Vector Multiply Even Signed Doubleword VX-form
* [code:: vpmsumb] Vector Polynomial Multiply-Sum Byte VX-form
* [code:: vpmsumh] Vector Polynomial Multiply-Sum Halfword VX-form
* [code:: vpmsumw] Vector Polynomial Multiply-Sum Word VX-form
* [code:: vpmsumd] Vector Polynomial Multiply-Sum Doubleword VX-form
* [code:: vcipher] Vector AES Cipher VX-form
* [code:: vncipher] Vector AES Inverse Cipher VX-form
* [code:: vsbox] Vector AES SubBytes VX-form
* [code:: vsum4ubs] Vector Sum across Quarter Unsigned Byte Saturate VX-form
* [code:: vsum4shs] Vector Sum across Quarter Signed Halfword Saturate VX-form
* [code:: vsum2sws] Vector Sum across Half Signed Word Saturate VX-form
* [code:: vsum4sbs] Vector Sum across Quarter Signed Byte Saturate VX-form
* [code:: vsumsws] Vector Sum across Signed Word Saturate VX-form
* [code:: vmuluwm] Vector Multiply Unsigned Word Modulo VX-form
* [code:: vmulld] Vector Multiply Low Doubleword VX-form
* [code:: vmulhuw] Vector Multiply High Unsigned Word VX-form
* [code:: vmulhud] Vector Multiply High Unsigned Doubleword VX-form
* [code:: vmulhsw] Vector Multiply High Signed Word VX-form
* [code:: vmulhsd] Vector Multiply High Signed Doubleword VX-form
* [code:: vcipherlast] Vector AES Cipher Last VX-form
* [code:: vncipherlast] Vector AES Inverse Cipher Last VX-form
* [code:: vaddfp] Vector Add Floating-Point VX-form
* [code:: vsubfp] Vector Subtract Floating-Point VX-form
* [code:: vrefp] Vector Reciprocal Estimate Floating-Point VX-form
* [code:: vrsqrtefp] Vector Reciprocal Square Root Estimate Floating-Point VX-form
* [code:: vexptefp] Vector 2 Raised to the Exponent Estimate Floating-Point VX-form
* [code:: vlogefp] Vector Log Base 2 Estimate Floating-Point VX-form
* [code:: vrfin] Vector Round to Floating-Point Integer Nearest VX-form
* [code:: vrfiz] Vector Round to Floating-Point Integer toward Zero VX-form
* [code:: vrfip] Vector Round to Floating-Point Integer toward +Infinity VX-form
* [code:: vrfim] Vector Round to Floating-Point Integer toward -Infinity VX-form
* [code:: vcfux] Vector Convert with round to nearest From Unsigned Word to floating-point format VX-form
* [code:: vcfsx] Vector Convert with round to nearest From Signed Word to floating-point format VX-form
* [code:: vctuxs] Vector Convert with round to zero from floating-point To Unsigned Word format Saturate VX-form
* [code:: vctsxs] Vector Convert with round to zero from floating-point To Signed Word format Saturate VX-form
* [code:: vmaxfp] Vector Maximum Floating-Point VX-form
* [code:: vminfp] Vector Minimum Floating-Point VX-form
* [code:: vdivuq] Vector Divide Unsigned Quadword VX-form
* [code:: vdivuw] Vector Divide Unsigned Word VX-form
* [code:: vdivud] Vector Divide Unsigned Doubleword VX-form
* [code:: vdivsq] Vector Divide Signed Quadword VX-form
* [code:: vdivsw] Vector Divide Signed Word VX-form
* [code:: vdivsd] Vector Divide Signed Doubleword VX-form
* [code:: vdiveuq] Vector Divide Extended Unsigned Quadword VX-form
* [code:: vdiveuw] Vector Divide Extended Unsigned Word VX-form
* [code:: vdiveud] Vector Divide Extended Unsigned Doubleword VX-form
* [code:: vdivesq] Vector Divide Extended Signed Quadword VX-form
* [code:: vdivesw] Vector Divide Extended Signed Word VX-form
* [code:: vdivesd] Vector Divide Extended Signed Doubleword VX-form
* [code:: vmoduq] Vector Modulo Unsigned Quadword VX-form
* [code:: vmoduw] Vector Modulo Unsigned Word VX-form
* [code:: vmodud] Vector Modulo Unsigned Doubleword VX-form
* [code:: vmodsq] Vector Modulo Signed Quadword VX-form
* [code:: vmodsw] Vector Modulo Signed Word VX-form
* [code:: vmodsd] Vector Modulo Signed Doubleword VX-form
* [code:: vmrghb] Vector Merge High Byte VX-form
* [code:: vmrghh] Vector Merge High Halfword VX-form
* [code:: vmrghw] Vector Merge High Word VX-form
* [code:: vmrglb] Vector Merge Low Byte VX-form
* [code:: vmrglh] Vector Merge Low Halfword VX-form
* [code:: vmrglw] Vector Merge Low Word VX-form
* [code:: vspltb] Vector Splat Byte VX-form
* [code:: vsplth] Vector Splat Halfword VX-form
* [code:: vspltw] Vector Splat Word VX-form
* [code:: vspltisb] Vector Splat Immediate Signed Byte VX-form
* [code:: vspltish] Vector Splat Immediate Signed Halfword VX-form
* [code:: vspltisw] Vector Splat Immediate Signed Word VX-form
* [code:: vslo] Vector Shift Left by Octet VX-form
* [code:: vsro] Vector Shift Right by Octet VX-form
* [code:: vgnb] Vector Gather every Nth Bit VX-form
* [code:: vgbbd] Vector Gather Bits by Bytes by Doubleword VX-form
* [code:: vbpermq] Vector Bit Permute Quadword VX-form
* [code:: vbpermd] Vector Bit Permute Doubleword VX-form
* [code:: vmrgow] Vector Merge Odd Word VX-form
* [code:: vmrgew] Vector Merge Even Word VX-form
* [code:: vstribl.] Vector String Isolate Byte Left-justified VC-form
* [code:: vstribl] Vector String Isolate Byte Left-justified VC-form
* [code:: vstribr.] Vector String Isolate Byte Right-justified VC-form
* [code:: vstribr] Vector String Isolate Byte Right-justified VC-form
* [code:: vstrihl.] Vector String Isolate Halfword Left-justified VC-form
* [code:: vstrihl] Vector String Isolate Halfword Left-justified VC-form
* [code:: vstrihr.] Vector String Isolate Halfword Right-justified VC-form
* [code:: vstrihr] Vector String Isolate Halfword Right-justified VC-form
* [code:: vclrlb] Vector Clear Leftmost Bytes VX-form
* [code:: vclrrb] Vector Clear Rightmost Bytes VX-form
* [code:: vextractub] Vector Extract Unsigned Byte to VSR using immediate-specified index VX-form
* [code:: vextractuh] Vector Extract Unsigned Halfword to VSR using immediate-specified index VX-form
* [code:: vextractuw] Vector Extract Unsigned Word to VSR using immediate-specified index VX-form
* [code:: vextractd] Vector Extract Doubleword to VSR using immediate-specified index VX-form
* [code:: vinsertb] Vector Insert Byte from VSR using immediate-specified index VX-form
* [code:: vinserth] Vector Insert Halfword from VSR using immediate-specified index VX-form
* [code:: vinsertw] Vector Insert Word from VSR using immediate-specified index VX-form
* [code:: vinsertd] Vector Insert Doubleword from VSR using immediate-specified index VX-form
* [code:: vcfuged] Vector Centrifuge Doubleword VX-form
* [code:: vpextd] Vector Parallel Bits Extract Doubleword VX-form
* [code:: vpdepd] Vector Parallel Bits Deposit Doubleword VX-form
* [code:: vextublx] Vector Extract Unsigned Byte to GPR using GPR-specified Left-Index VX-form
* [code:: vextuhlx] Vector Extract Unsigned Halfword to GPR using GPR-specified Left-Index VX-form
* [code:: vextuwlx] Vector Extract Unsigned Word to GPR using GPR-specified Left-Index VX-form
* [code:: vextubrx] Vector Extract Unsigned Byte to GPR using GPR-specified Right-Index VX-form
* [code:: vextuhrx] Vector Extract Unsigned Halfword to GPR using GPR-specified Right-Index VX-form
* [code:: vextuwrx] Vector Extract Unsigned Word to GPR using GPR-specified Right-Index VX-form
* [code:: vpkuhum] Vector Pack Unsigned Halfword Unsigned Modulo VX-form
* [code:: vpkuwum] Vector Pack Unsigned Word Unsigned Modulo VX-form
* [code:: vpkuhus] Vector Pack Unsigned Halfword Unsigned Saturate VX-form
* [code:: vpkuwus] Vector Pack Unsigned Word Unsigned Saturate VX-form
* [code:: vpkshus] Vector Pack Signed Halfword Unsigned Saturate VX-form
* [code:: vpkswus] Vector Pack Signed Word Unsigned Saturate VX-form
* [code:: vpkshss] Vector Pack Signed Halfword Signed Saturate VX-form
* [code:: vpkswss] Vector Pack Signed Word Signed Saturate VX-form
* [code:: vupkhsb] Vector Unpack High Signed Byte VX-form
* [code:: vupkhsh] Vector Unpack High Signed Halfword VX-form
* [code:: vupklsb] Vector Unpack Low Signed Byte VX-form
* [code:: vupklsh] Vector Unpack Low Signed Halfword VX-form
* [code:: vpkpx] Vector Pack Pixel VX-form
* [code:: vupkhpx] Vector Unpack High Pixel VX-form
* [code:: vupklpx] Vector Unpack Low Pixel VX-form
* [code:: vpkudum] Vector Pack Unsigned Doubleword Unsigned Modulo VX-form
* [code:: vpkudus] Vector Pack Unsigned Doubleword Unsigned Saturate VX-form
* [code:: vpksdus] Vector Pack Signed Doubleword Unsigned Saturate VX-form
* [code:: vpksdss] Vector Pack Signed Doubleword Signed Saturate VX-form
* [code:: vupkhsw] Vector Unpack High Signed Word VX-form
* [code:: vupklsw] Vector Unpack Low Signed Word VX-form
* [code:: vinsbvlx] Vector Insert Byte from VSR using GPR-specified Left-Index VX-form
* [code:: vinshvlx] Vector Insert Halfword from VSR using GPR-specified Left-Index VX-form
* [code:: vinswvlx] Vector Insert Word from VSR using GPR-specified Left-Index VX-form
* [code:: vinsw] Vector Insert Word from GPR using immediate-specified index VX-form
* [code:: vinsbvrx] Vector Insert Byte from VSR using GPR-specified Right-Index VX-form
* [code:: vinshvrx] Vector Insert Halfword from VSR using GPR-specified Right-Index VX-form
* [code:: vinswvrx] Vector Insert Word from VSR using GPR-specified Right-Index VX-form
* [code:: vinsd] Vector Insert Doubleword from GPR using immediate-specified index VX-form
* [code:: vinsblx] Vector Insert Byte from GPR using GPR-specified Left-Index VX-form
* [code:: vinshlx] Vector Insert Halfword from GPR using GPR-specified Left-Index VX-form
* [code:: vinswlx] Vector Insert Word from GPR using GPR-specified Left-Index VX-form
* [code:: vinsdlx] Vector Insert Doubleword from GPR using GPR-specified Left-Index VX-form
* [code:: vinsbrx] Vector Insert Byte from GPR using GPR-specified Right-Index VX-form
* [code:: vinshrx] Vector Insert Halfword from GPR using GPR-specified Right-Index VX-form
* [code:: vinswrx] Vector Insert Word from GPR using GPR-specified Right-Index VX-form
* [code:: vinsdrx] Vector Insert Doubleword from GPR using GPR-specified Right-Index VX-form
* [code:: mtvsrbmi] Move To VSR Byte Mask Immediate DX-form
* [code:: vsldbi] Vector Shift Left Double by Bit Immediate VN-form
* [code:: vsrdbi] Vector Shift Right Double by Bit Immediate VN-form
* [code:: vmsumcud] Vector Multiply-Sum & write Carry-out Unsigned Doubleword VA-form
* [code:: vextdubvlx] Vector Extract Double Unsigned Byte to VSR using GPR-specified Left-Index VA-form
* [code:: vextdubvrx] Vector Extract Double Unsigned Byte to VSR using GPR-specified Right-Index VA-form
* [code:: vextduhvlx] Vector Extract Double Unsigned Halfword to VSR using GPR-specified Left-Index VA-form
* [code:: vextduhvrx] Vector Extract Double Unsigned Halfword to VSR using GPR-specified Right-Index VA-form
* [code:: vextduwvlx] Vector Extract Double Unsigned Word to VSR using GPR-specified Left-Index VA-form
* [code:: vextduwvrx] Vector Extract Double Unsigned Word to VSR using GPR-specified Right-Index VA-form
* [code:: vextddvlx] Vector Extract Double Doubleword to VSR using GPR-specified Left-Index VA-form
* [code:: vextddvrx] Vector Extract Double Doubleword to VSR using GPR-specified Right-Index VA-form
* [code:: vmhaddshs] Vector Multiply-High-Add Signed Halfword Saturate VA-form
* [code:: vmhraddshs] Vector Multiply-High-Round-Add Signed Halfword Saturate VA-form
* [code:: vmladduhm] Vector Multiply-Low-Add Unsigned Halfword Modulo VA-form
* [code:: vmsumudm] Vector Multiply-Sum Unsigned Doubleword Modulo VA-form
* [code:: vmsumubm] Vector Multiply-Sum Unsigned Byte Modulo VA-form
* [code:: vmsummbm] Vector Multiply-Sum Mixed Byte Modulo VA-form
* [code:: vmsumuhm] Vector Multiply-Sum Unsigned Halfword Modulo VA-form
* [code:: vmsumuhs] Vector Multiply-Sum Unsigned Halfword Saturate VA-form
* [code:: vmsumshm] Vector Multiply-Sum Signed Halfword Modulo VA-form
* [code:: vmsumshs] Vector Multiply-Sum Signed Halfword Saturate VA-form
* [code:: vsel] Vector Select VA-form
* [code:: vperm] Vector Permute VA-form
* [code:: vsldoi] Vector Shift Left Double by Octet Immediate VA-form
* [code:: vpermxor] Vector Permute & Exclusive-OR VA-form
* [code:: vmaddfp] Vector Multiply-Add Floating-Point VA-form
* [code:: vnmsubfp] Vector Negative Multiply-Subtract Floating-Point VA-form
* [code:: maddhd] Multiply-Add High Doubleword VA-form
* [code:: maddhdu] Multiply-Add High Doubleword Unsigned VA-form
* [code:: maddld] Multiply-Add Low Doubleword VA-form
* [code:: vpermr] Vector Permute Right-indexed VA-form
* [code:: vaddeuqm] Vector Add Extended Unsigned Quadword Modulo VA-form
* [code:: vaddecuq] Vector Add Extended & write Carry-out Unsigned Quadword VA-form
* [code:: vsubeuqm] Vector Subtract Extended Unsigned Quadword Modulo VA-form
* [code:: vsubecuq] Vector Subtract Extended & write Carry-out Unsigned Quadword VA-form
* [code:: lxvp] Load VSX Vector Paired DQ-form
* [code:: stxvp] Store VSX Vector Paired DQ-form
* [code:: mulli] Multiply Low Immediate D-form
* [code:: subfic] Subtract From Immediate Carrying D-form
* [code:: cmpli] Compare Logical Immediate D-form
* [code:: cmpi] Compare Immediate D-form
* [code:: addic] Add Immediate Carrying D-form
* [code:: addic.] Add Immediate Carrying and Record D-form
* [code:: addi] Add Immediate D-form
* [code:: paddi] Prefixed Add Immediate MLS:D-form
* [code:: addis] Add Immediate Shifted D-form
* [code:: bcla] Branch Conditional B-form
* [code:: bcl] Branch Conditional B-form
* [code:: bca] Branch Conditional B-form
* [code:: bc] Branch Conditional B-form
* [code:: scv] System Call Vectored SC-form
* [code:: sc] System Call SC-form
* [code:: bla] Branch I-form
* [code:: bl] Branch I-form
* [code:: ba] Branch I-form
* [code:: b] Branch I-form
* [code:: mcrf] Move Condition Register Field XL-form
* [code:: crnor] Condition Register NOR XL-form
* [code:: crandc] Condition Register AND with Complement XL-form
* [code:: crxor] Condition Register XOR XL-form
* [code:: crnand] Condition Register NAND XL-form
* [code:: crand] Condition Register AND XL-form
* [code:: creqv] Condition Register Equivalent XL-form
* [code:: crorc] Condition Register OR with Complement XL-form
* [code:: cror] Condition Register OR XL-form
* [code:: addpcis] Add PC Immediate Shifted DX-form
* [code:: bclrl] Branch Conditional to Link Register XL-form
* [code:: bclr] Branch Conditional to Link Register XL-form
* [code:: bcctrl] Branch Conditional to Count Register XL-form
* [code:: bcctr] Branch Conditional to Count Register XL-form
* [code:: bctarl] Branch Conditional to Branch Target Address Register XL-form
* [code:: bctar] Branch Conditional to Branch Target Address Register XL-form
* [code:: rfid] Return From Interrupt Doubleword XL-form
* [code:: rfscv] Return From System Call Vectored XL-form
* [code:: rfebb] Return from Event-Based Branch XL-form
* [code:: hrfid] Hypervisor Return From Interrupt Doubleword XL-form
* [code:: urfid] Ultravisor Return From Interrupt Doubleword XL-form
* [code:: stop] Stop XL-form
* [code:: isync] Instruction Synchronize XL-form
* [code:: rlwimi.] Rotate Left Word Immediate then Mask Insert M-form
* [code:: rlwimi] Rotate Left Word Immediate then Mask Insert M-form
* [code:: rlwinm.] Rotate Left Word Immediate then AND with Mask M-form
* [code:: rlwinm] Rotate Left Word Immediate then AND with Mask M-form
* [code:: rlwnm.] Rotate Left Word then AND with Mask M-form
* [code:: rlwnm] Rotate Left Word then AND with Mask M-form
* [code:: ori] OR Immediate D-form
* [code:: oris] OR Immediate Shifted D-form
* [code:: xori] XOR Immediate D-form
* [code:: xoris] XOR Immediate Shifted D-form
* [code:: andi.] AND Immediate D-form
* [code:: andis.] AND Immediate Shifted D-form
* [code:: rldicl.] Rotate Left Doubleword Immediate then Clear Left MD-form
* [code:: rldicl] Rotate Left Doubleword Immediate then Clear Left MD-form
* [code:: rldicr.] Rotate Left Doubleword Immediate then Clear Right MD-form
* [code:: rldicr] Rotate Left Doubleword Immediate then Clear Right MD-form
* [code:: rldic.] Rotate Left Doubleword Immediate then Clear MD-form
* [code:: rldic] Rotate Left Doubleword Immediate then Clear MD-form
* [code:: rldimi.] Rotate Left Doubleword Immediate then Mask Insert MD-form
* [code:: rldimi] Rotate Left Doubleword Immediate then Mask Insert MD-form
* [code:: rldcl.] Rotate Left Doubleword then Clear Left MDS-form
* [code:: rldcl] Rotate Left Doubleword then Clear Left MDS-form
* [code:: rldcr.] Rotate Left Doubleword then Clear Right MDS-form
* [code:: rldcr] Rotate Left Doubleword then Clear Right MDS-form
* [code:: cmp] Compare X-form
* [code:: cmpl] Compare Logical X-form
* [code:: setb] Set Boolean X-form
* [code:: cmprb] Compare Ranged Byte X-form
* [code:: cmpeqb] Compare Equal Byte X-form
* [code:: setbc] Set Boolean Condition X-form
* [code:: setbcr] Set Boolean Condition Reverse X-form
* [code:: setnbc] Set Negative Boolean Condition X-form
* [code:: setnbcr] Set Negative Boolean Condition Reverse X-form
* [code:: mcrxrx] Move to CR from XER Extended X-form
* [code:: tw] Trap Word X-form
* [code:: td] Trap Doubleword X-form
* [code:: lvsl] Load Vector for Shift Left Indexed X-form
* [code:: lvsr] Load Vector for Shift Right Indexed X-form
* [code:: lwat] Load Word Atomic X-form
* [code:: ldat] Load Doubleword Atomic X-form
* [code:: stwat] Store Word Atomic X-form
* [code:: stdat] Store Doubleword Atomic X-form
* [code:: copy] Copy X-form
* [code:: cpabort] Copy-Paste Abort X-form
* [code:: paste.] Paste X-form
* [code:: lvebx] Load Vector Element Byte Indexed X-form
* [code:: lvehx] Load Vector Element Halfword Indexed X-form
* [code:: lvewx] Load Vector Element Word Indexed X-form
* [code:: lvx] Load Vector Indexed X-form
* [code:: stvebx] Store Vector Element Byte Indexed X-form
* [code:: stvehx] Store Vector Element Halfword Indexed X-form
* [code:: stvewx] Store Vector Element Word Indexed X-form
* [code:: stvx] Store Vector Indexed X-form
* [code:: lvxl] Load Vector Indexed Last X-form
* [code:: stvxl] Store Vector Indexed Last X-form
* [code:: subfc.] Subtract From Carrying XO-form
* [code:: subfc] Subtract From Carrying XO-form
* [code:: subf.] Subtract From XO-form
* [code:: subf] Subtract From XO-form
* [code:: neg.] Negate XO-form
* [code:: neg] Negate XO-form
* [code:: subfe.] Subtract From Extended XO-form
* [code:: subfe] Subtract From Extended XO-form
* [code:: subfze.] Subtract From Zero Extended XO-form
* [code:: subfze] Subtract From Zero Extended XO-form
* [code:: subfme.] Subtract From Minus One Extended XO-form
* [code:: subfme] Subtract From Minus One Extended XO-form
* [code:: subfco.] Subtract From Carrying & record OV XO-form
* [code:: subfco] Subtract From Carrying & record OV XO-form
* [code:: subfo.] Subtract From & record OV XO-form
* [code:: subfo] Subtract From & record OV XO-form
* [code:: nego.] Negate & record OV XO-form
* [code:: nego] Negate & record OV XO-form
* [code:: subfeo.] Subtract From Extended & record OV XO-form
* [code:: subfeo] Subtract From Extended & record OV XO-form
* [code:: subfzeo.] Subtract From Zero Extended & record OV XO-form
* [code:: subfzeo] Subtract From Zero Extended & record OV XO-form
* [code:: subfmeo.] Subtract From Minus One Extended & record OV XO-form
* [code:: subfmeo] Subtract From Minus One Extended & record OV XO-form
* [code:: mulhdu.] Multiply High Doubleword Unsigned XO-form
* [code:: mulhdu] Multiply High Doubleword Unsigned XO-form
* [code:: mulhd.] Multiply High Doubleword XO-form
* [code:: mulhd] Multiply High Doubleword XO-form
* [code:: mulld.] Multiply Low Doubleword XO-form
* [code:: mulld] Multiply Low Doubleword XO-form
* [code:: modud] Modulo Unsigned Doubleword X-form
* [code:: divdeu.] Divide Doubleword Extended Unsigned XO-form
* [code:: divdeu] Divide Doubleword Extended Unsigned XO-form
* [code:: divde.] Divide Doubleword Extended XO-form
* [code:: divde] Divide Doubleword Extended XO-form
* [code:: divdu.] Divide Doubleword Unsigned XO-form
* [code:: divdu] Divide Doubleword Unsigned XO-form
* [code:: divd.] Divide Doubleword XO-form
* [code:: divd] Divide Doubleword XO-form
* [code:: mulldo.] Multiply Low Doubleword & record OV XO-form
* [code:: mulldo] Multiply Low Doubleword & record OV XO-form
* [code:: modsd] Modulo Signed Doubleword X-form
* [code:: divdeuo.] Divide Doubleword Extended Unsigned & record OV XO-form
* [code:: divdeuo] Divide Doubleword Extended Unsigned & record OV XO-form
* [code:: divdeo.] Divide Doubleword Extended & record OV XO-form
* [code:: divdeo] Divide Doubleword Extended & record OV XO-form
* [code:: divduo.] Divide Doubleword Unsigned & record OV XO-form
* [code:: divduo] Divide Doubleword Unsigned & record OV XO-form
* [code:: divdo.] Divide Doubleword & record OV XO-form
* [code:: divdo] Divide Doubleword & record OV XO-form
* [code:: addex] Add Extended using alternate carry bit Z23-form
* [code:: addg6s] Add and Generate Sixes XO-form
* [code:: addc.] Add Carrying XO-form
* [code:: addc] Add Carrying XO-form
* [code:: adde.] Add Extended XO-form
* [code:: adde] Add Extended XO-form
* [code:: addze.] Add to Zero Extended XO-form
* [code:: addze] Add to Zero Extended XO-form
* [code:: addme.] Add to Minus One Extended XO-form
* [code:: addme] Add to Minus One Extended XO-form
* [code:: add.] Add XO-form
* [code:: add] Add XO-form
* [code:: addco.] Add Carrying & record OV XO-form
* [code:: addco] Add Carrying & record OV XO-form
* [code:: addeo.] Add Extended & record OV XO-form
* [code:: addeo] Add Extended & record OV XO-form
* [code:: addzeo.] Add to Zero Extended & record OV XO-form
* [code:: addzeo] Add to Zero Extended & record OV XO-form
* [code:: addmeo.] Add to Minus One Extended & record OV XO-form
* [code:: addmeo] Add to Minus One Extended & record OV XO-form
* [code:: addo.] Add & record OV XO-form
* [code:: addo] Add & record OV XO-form
* [code:: mulhwu.] Multiply High Word Unsigned XO-form
* [code:: mulhwu] Multiply High Word Unsigned XO-form
* [code:: mulhw.] Multiply High Word XO-form
* [code:: mulhw] Multiply High Word XO-form
* [code:: mullw.] Multiply Low Word XO-form
* [code:: mullw] Multiply Low Word XO-form
* [code:: moduw] Modulo Unsigned Word X-form
* [code:: divweu.] Divide Word Extended Unsigned XO-form
* [code:: divweu] Divide Word Extended Unsigned XO-form
* [code:: divwe.] Divide Word Extended XO-form
* [code:: divwe] Divide Word Extended XO-form
* [code:: divwu.] Divide Word Unsigned XO-form
* [code:: divwu] Divide Word Unsigned XO-form
* [code:: divw.] Divide Word XO-form
* [code:: divw] Divide Word XO-form
* [code:: mullwo.] Multiply Low Word & record OV XO-form
* [code:: mullwo] Multiply Low Word & record OV XO-form
* [code:: modsw] Modulo Signed Word X-form
* [code:: divweuo.] Divide Word Extended Unsigned & record OV XO-form
* [code:: divweuo] Divide Word Extended Unsigned & record OV XO-form
* [code:: divweo.] Divide Word Extended & record OV XO-form
* [code:: divweo] Divide Word Extended & record OV XO-form
* [code:: divwuo.] Divide Word Unsigned & record OV XO-form
* [code:: divwuo] Divide Word Unsigned & record OV XO-form
* [code:: divwo.] Divide Word & record OV XO-form
* [code:: divwo] Divide Word & record OV XO-form
* [code:: lxsiwzx] Load VSX Scalar as Integer Word & Zero Indexed X-form
* [code:: lxsiwax] Load VSX Scalar as Integer Word Algebraic Indexed X-form
* [code:: stxsiwx] Store VSX Scalar as Integer Word Indexed X-form
* [code:: lxvx] Load VSX Vector Indexed X-form
* [code:: lxvdsx] Load VSX Vector Doubleword & Splat Indexed X-form
* [code:: lxvwsx] Load VSX Vector Word & Splat Indexed X-form
* [code:: stxvx] Store VSX Vector Indexed X-form
* [code:: lxsspx] Load VSX Scalar Single-Precision Indexed X-form
* [code:: lxsdx] Load VSX Scalar Doubleword Indexed X-form
* [code:: stxsspx] Store VSX Scalar Single-Precision Indexed X-form
* [code:: stxsdx] Store VSX Scalar Doubleword Indexed X-form
* [code:: lxvw4x] Load VSX Vector Word*4 Indexed X-form
* [code:: lxvh8x] Load VSX Vector Halfword*8 Indexed X-form
* [code:: lxvd2x] Load VSX Vector Doubleword*2 Indexed X-form
* [code:: lxvb16x] Load VSX Vector Byte*16 Indexed X-form
* [code:: stxvw4x] Store VSX Vector Word*4 Indexed X-form
* [code:: stxvh8x] Store VSX Vector Halfword*8 Indexed X-form
* [code:: stxvd2x] Store VSX Vector Doubleword*2 Indexed X-form
* [code:: stxvb16x] Store VSX Vector Byte*16 Indexed X-form
* [code:: lxvrbx] Load VSX Vector Rightmost Byte Indexed X-form
* [code:: lxvrhx] Load VSX Vector Rightmost Halfword Indexed X-form
* [code:: lxvrwx] Load VSX Vector Rightmost Word Indexed X-form
* [code:: lxvrdx] Load VSX Vector Rightmost Doubleword Indexed X-form
* [code:: stxvrbx] Store VSX Vector Rightmost Byte Indexed X-form
* [code:: stxvrhx] Store VSX Vector Rightmost Halfword Indexed X-form
* [code:: stxvrwx] Store VSX Vector Rightmost Word Indexed X-form
* [code:: stxvrdx] Store VSX Vector Rightmost Doubleword Indexed X-form
* [code:: lxvl] Load VSX Vector with Length X-form
* [code:: lxvll] Load VSX Vector with Length Left-justified X-form
* [code:: lxvpx] Load VSX Vector Paired Indexed X-form
* [code:: stxvl] Store VSX Vector with Length X-form
* [code:: stxvll] Store VSX Vector with Length Left-justified X-form
* [code:: stxvpx] Store VSX Vector Paired Indexed X-form
* [code:: lxsibzx] Load VSX Scalar as Integer Byte & Zero Indexed X-form
* [code:: lxsihzx] Load VSX Scalar as Integer Halfword & Zero Indexed X-form
* [code:: stxsibx] Store VSX Scalar as Integer Byte Indexed X-form
* [code:: stxsihx] Store VSX Scalar as Integer Halfword Indexed X-form
* [code:: msgsndu] Message Send Ultravisor X-form
* [code:: msgclru] Message Clear Ultravisor X-form
* [code:: msgsndp] Message Send Privileged X-form
* [code:: msgclrp] Message Clear Privileged X-form
* [code:: msgsnd] Message Send X-form
* [code:: msgclr] Message Clear X-form
* [code:: mfbhrbe] Move From Branch History Rolling Buffer Entry XFX-form
* [code:: clrbhrb] Clear BHRB X-form
* [code:: isel] Integer Select A-form
* [code:: mtcrf] Move To Condition Register Fields XFX-form
* [code:: mtocrf] Move To One Condition Register Field XFX-form
* [code:: xxmfacc] VSX Move From Accumulator X-form
* [code:: xxmtacc] VSX Move To Accumulator X-form
* [code:: xxsetaccz] VSX Set Accumulator to Zero X-form
* [code:: mtmsr] Move To Machine State Register X-form
* [code:: mtmsrd] Move To Machine State Register Doubleword X-form
* [code:: tlbiel] TLB Invalidate Entry Local X-form
* [code:: tlbie] TLB Invalidate Entry X-form
* [code:: slbsync] SLB Synchronize X-form
* [code:: slbmte] SLB Move To Entry X-form
* [code:: slbie] SLB Invalidate Entry X-form
* [code:: slbieg] SLB Invalidate Entry Global X-form
* [code:: slbia] SLB Invalidate All X-form
* [code:: hashstp] Hash Store Privileged X-form
* [code:: hashchkp] Hash Check Privileged X-form
* [code:: hashst] Hash Store X-form
* [code:: hashchk] Hash Check X-form
* [code:: slbiag] SLB Invalidate All Global X-form
* [code:: mfcr] Move From Condition Register XFX-form
* [code:: mfocrf] Move From One Condition Register Field XFX-form
* [code:: mfvsrd] Move From VSR Doubleword X-form
* [code:: mfmsr] Move From Machine State Register X-form
* [code:: mfvsrwz] Move From VSR Word and Zero X-form
* [code:: mtvsrd] Move To VSR Doubleword X-form
* [code:: mtvsrwa] Move To VSR Word Algebraic X-form
* [code:: mtvsrwz] Move To VSR Word and Zero X-form
* [code:: mfvsrld] Move From VSR Lower Doubleword X-form
* [code:: mfspr] Move From Special Purpose Register XFX-form
* [code:: mftb] Move From Time Base XFX-form
* [code:: mtvsrws] Move To VSR Word & Splat X-form
* [code:: mtvsrdd] Move To VSR Double Doubleword X-form
* [code:: mtspr] Move To Special Purpose Register XFX-form
* [code:: darn] Deliver A Random Number X-form
* [code:: slbmfev] SLB Move From Entry VSID X-form
* [code:: slbmfee] SLB Move From Entry ESID X-form
* [code:: slbfee.] SLB Find Entry ESID X-form
* [code:: lwarx] Load Word And Reserve Indexed X-form
* [code:: lbarx] Load Byte And Reserve Indexed X-form
* [code:: ldarx] Load Doubleword And Reserve Indexed X-form
* [code:: lharx] Load Halfword And Reserve Indexed X-form
* [code:: lqarx] Load Quadword And Reserve Indexed X-form
* [code:: ldbrx] Load Doubleword Byte-Reverse Indexed X-form
* [code:: stdbrx] Store Doubleword Byte-Reverse Indexed X-form
* [code:: ldx] Load Doubleword Indexed X-form
* [code:: ldux] Load Doubleword with Update Indexed X-form
* [code:: stdx] Store Doubleword Indexed X-form
* [code:: stdux] Store Doubleword with Update Indexed X-form
* [code:: lwax] Load Word Algebraic Indexed X-form
* [code:: lwaux] Load Word Algebraic with Update Indexed X-form
* [code:: lswx] Load String Word Indexed X-form
* [code:: lswi] Load String Word Immediate X-form
* [code:: stswx] Store String Word Indexed X-form
* [code:: stswi] Store String Word Immediate X-form
* [code:: lwzcix] Load Word and Zero Caching Inhibited Indexed X-form
* [code:: lhzcix] Load Halfword and Zero Caching Inhibited Indexed X-form
* [code:: lbzcix] Load Byte and Zero Caching Inhibited Indexed X-form
* [code:: ldcix] Load Doubleword Caching Inhibited Indexed X-form
* [code:: stwcix] Store Word Caching Inhibited Indexed X-form
* [code:: sthcix] Store Halfword Caching Inhibited Indexed X-form
* [code:: stbcix] Store Byte Caching Inhibited Indexed X-form
* [code:: stdcix] Store Doubleword Caching Inhibited Indexed X-form
* [code:: icbt] Instruction Cache Block Touch X-form
* [code:: dcbst] Data Cache Block Store X-form
* [code:: dcbf] Data Cache Block Flush X-form
* [code:: dcbtst] Data Cache Block Touch for Store X-form
* [code:: dcbt] Data Cache Block Touch X-form
* [code:: lwbrx] Load Word Byte-Reverse Indexed X-form
* [code:: tlbsync] TLB Synchronize X-form
* [code:: sync] Synchronize X-form
* [code:: stwbrx] Store Word Byte-Reverse Indexed X-form
* [code:: lhbrx] Load Halfword Byte-Reverse Indexed X-form
* [code:: eieio] Enforce In-order Execution of I/O X-form
* [code:: msgsync] Message Synchronize X-form
* [code:: sthbrx] Store Halfword Byte-Reverse Indexed X-form
* [code:: icbi] Instruction Cache Block Invalidate X-form
* [code:: dcbz] Data Cache Block set to Zero X-form
* [code:: stwcx.] Store Word Conditional Indexed X-form
* [code:: stqcx.] Store Quadword Conditional Indexed X-form
* [code:: stdcx.] Store Doubleword Conditional Indexed X-form
* [code:: stbcx.] Store Byte Conditional Indexed X-form
* [code:: sthcx.] Store Halfword Conditional Indexed X-form
* [code:: lwzx] Load Word and Zero Indexed X-form
* [code:: lwzux] Load Word and Zero with Update Indexed X-form
* [code:: lbzx] Load Byte and Zero Indexed X-form
* [code:: lbzux] Load Byte and Zero with Update Indexed X-form
* [code:: stwx] Store Word Indexed X-form
* [code:: stwux] Store Word with Update Indexed X-form
* [code:: stbx] Store Byte Indexed X-form
* [code:: stbux] Store Byte with Update Indexed X-form
* [code:: lhzx] Load Halfword and Zero Indexed X-form
* [code:: lhzux] Load Halfword and Zero with Update Indexed X-form
* [code:: lhax] Load Halfword Algebraic Indexed X-form
* [code:: lhaux] Load Halfword Algebraic with Update Indexed X-form
* [code:: sthx] Store Halfword Indexed X-form
* [code:: sthux] Store Halfword with Update Indexed X-form
* [code:: lfsx] Load Floating-Point Single Indexed X-form
* [code:: lfsux] Load Floating-Point Single with Update Indexed X-form
* [code:: lfdx] Load Floating-Point Double Indexed X-form
* [code:: lfdux] Load Floating-Point Double with Update Indexed X-form
* [code:: stfsx] Store Floating-Point Single Indexed X-form
* [code:: stfsux] Store Floating-Point Single with Update Indexed X-form
* [code:: stfdx] Store Floating-Point Double Indexed X-form
* [code:: stfdux] Store Floating-Point Double with Update Indexed X-form
* [code:: lfdpx] Load Floating-Point Double Pair Indexed X-form
* [code:: lfiwax] Load Floating-Point as Integer Word Algebraic Indexed X-form
* [code:: lfiwzx] Load Floating-Point as Integer Word & Zero Indexed X-form
* [code:: stfdpx] Store Floating-Point Double Pair Indexed X-form
* [code:: stfiwx] Store Floating-Point as Integer Word Indexed X-form
* [code:: slw.] Shift Left Word X-form
* [code:: slw] Shift Left Word X-form
* [code:: srw.] Shift Right Word X-form
* [code:: srw] Shift Right Word X-form
* [code:: sraw.] Shift Right Algebraic Word X-form
* [code:: sraw] Shift Right Algebraic Word X-form
* [code:: srawi.] Shift Right Algebraic Word Immediate X-form
* [code:: srawi] Shift Right Algebraic Word Immediate X-form
* [code:: sradi.] Shift Right Algebraic Doubleword Immediate XS-form
* [code:: sradi] Shift Right Algebraic Doubleword Immediate XS-form
* [code:: extswsli.] Extend Sign Word and Shift Left Immediate XS-form
* [code:: extswsli] Extend Sign Word and Shift Left Immediate XS-form
* [code:: cntlzw.] Count Leading Zeros Word X-form
* [code:: cntlzw] Count Leading Zeros Word X-form
* [code:: cntlzd.] Count Leading Zeros Doubleword X-form
* [code:: cntlzd] Count Leading Zeros Doubleword X-form
* [code:: popcntb] Population Count Bytes X-form
* [code:: prtyw] Parity Word X-form
* [code:: prtyd] Parity Doubleword X-form
* [code:: cdtbcd] Convert Declets To Binary Coded Decimal X-form
* [code:: cbcdtd] Convert Binary Coded Decimal To Declets X-form
* [code:: popcntw] Population Count Words X-form
* [code:: popcntd] Population Count Doubleword X-form
* [code:: cnttzw.] Count Trailing Zeros Word X-form
* [code:: cnttzw] Count Trailing Zeros Word X-form
* [code:: cnttzd.] Count Trailing Zeros Doubleword X-form
* [code:: cnttzd] Count Trailing Zeros Doubleword X-form
* [code:: srad.] Shift Right Algebraic Doubleword X-form
* [code:: srad] Shift Right Algebraic Doubleword X-form
* [code:: extsh.] Extend Sign Halfword X-form
* [code:: extsh] Extend Sign Halfword X-form
* [code:: extsb.] Extend Sign Byte X-form
* [code:: extsb] Extend Sign Byte X-form
* [code:: extsw.] Extend Sign Word X-form
* [code:: extsw] Extend Sign Word X-form
* [code:: sld.] Shift Left Doubleword X-form
* [code:: sld] Shift Left Doubleword X-form
* [code:: cntlzdm] Count Leading Zeros Doubleword under bit Mask X-form
* [code:: brw] Byte-Reverse Word X-form
* [code:: brd] Byte-Reverse Doubleword X-form
* [code:: brh] Byte-Reverse Halfword X-form
* [code:: srd.] Shift Right Doubleword X-form
* [code:: srd] Shift Right Doubleword X-form
* [code:: cnttzdm] Count Trailing Zeros Doubleword under bit Mask X-form
* [code:: and.] AND X-form
* [code:: and] AND X-form
* [code:: andc.] AND with Complement X-form
* [code:: andc] AND with Complement X-form
* [code:: nor.] NOR X-form
* [code:: nor] NOR X-form
* [code:: pdepd] Parallel Bits Deposit Doubleword X-form
* [code:: pextd] Parallel Bits Extract Doubleword X-form
* [code:: cfuged] Centrifuge Doubleword X-form
* [code:: bpermd] Bit Permute Doubleword X-form
* [code:: eqv.] Equivalent X-form
* [code:: eqv] Equivalent X-form
* [code:: xor.] XOR X-form
* [code:: xor] XOR X-form
* [code:: orc.] OR with Complement X-form
* [code:: orc] OR with Complement X-form
* [code:: or.] OR X-form
* [code:: or] OR X-form
* [code:: nand.] NAND X-form
* [code:: nand] NAND X-form
* [code:: cmpb] Compare Bytes X-form
* [code:: wait] Wait X-form
* [code:: lwz] Load Word and Zero D-form
* [code:: plwz] Prefixed Load Word and Zero MLS:D-form
* [code:: xxsplti32dx] VSX Vector Splat Immediate32 Doubleword Indexed 8RR:D-form
* [code:: xxspltidp] VSX Vector Splat Immediate Double-Precision 8RR:D-form
* [code:: xxspltiw] VSX Vector Splat Immediate Word 8RR:D-form
* [code:: lwzu] Load Word and Zero with Update D-form
* [code:: xxblendvb] VSX Vector Blend Variable Byte 8RR:XX4-form
* [code:: xxblendvh] VSX Vector Blend Variable Halfword 8RR:XX4-form
* [code:: xxblendvw] VSX Vector Blend Variable Word 8RR:XX4-form
* [code:: xxblendvd] VSX Vector Blend Variable Doubleword 8RR:XX4-form
* [code:: lbz] Load Byte and Zero D-form
* [code:: plbz] Prefixed Load Byte and Zero MLS:D-form
* [code:: xxpermx] VSX Vector Permute Extended 8RR:XX4-form
* [code:: xxeval] VSX Vector Evaluate 8RR:XX4-form
* [code:: lbzu] Load Byte and Zero with Update D-form
* [code:: stw] Store Word D-form
* [code:: pstw] Prefixed Store Word MLS:D-form
* [code:: stwu] Store Word with Update D-form
* [code:: stb] Store Byte D-form
* [code:: pstb] Prefixed Store Byte MLS:D-form
* [code:: stbu] Store Byte with Update D-form
* [code:: lhz] Load Halfword and Zero D-form
* [code:: plhz] Prefixed Load Halfword and Zero MLS:D-form
* [code:: lhzu] Load Halfword and Zero with Update D-form
* [code:: plwa] Prefixed Load Word Algebraic 8LS:D-form
* [code:: lha] Load Halfword Algebraic D-form
* [code:: plxsd] Prefixed Load VSX Scalar Doubleword 8LS:D-form
* [code:: plha] Prefixed Load Halfword Algebraic MLS:D-form
* [code:: lhau] Load Halfword Algebraic with Update D-form
* [code:: plxssp] Prefixed Load VSX Scalar Single-Precision 8LS:D-form
* [code:: sth] Store Halfword D-form
* [code:: psth] Prefixed Store Halfword MLS:D-form
* [code:: sthu] Store Halfword with Update D-form
* [code:: lmw] Load Multiple Word D-form
* [code:: pstxsd] Prefixed Store VSX Scalar Doubleword 8LS:D-form
* [code:: stmw] Store Multiple Word D-form
* [code:: pstxssp] Prefixed Store VSX Scalar Single-Precision 8LS:D-form
* [code:: lfs] Load Floating-Point Single D-form
* [code:: plfs] Prefixed Load Floating-Point Single MLS:D-form
* [code:: lfsu] Load Floating-Point Single with Update D-form
* [code:: plxv] Prefixed Load VSX Vector 8LS:D-form
* [code:: lfd] Load Floating-Point Double D-form
* [code:: plfd] Prefixed Load Floating-Point Double MLS:D-form
* [code:: lfdu] Load Floating-Point Double with Update D-form
* [code:: stfs] Store Floating-Point Single D-form
* [code:: pstfs] Prefixed Store Floating-Point Single MLS:D-form
* [code:: stfsu] Store Floating-Point Single with Update D-form
* [code:: pstxv] Prefixed Store VSX Vector 8LS:D-form
* [code:: stfd] Store Floating-Point Double D-form
* [code:: pstfd] Prefixed Store Floating-Point Double MLS:D-form
* [code:: stfdu] Store Floating-Point Double with Update D-form
* [code:: lq] Load Quadword DQ-form
* [code:: plq] Prefixed Load Quadword 8LS:D-form
* [code:: pld] Prefixed Load Doubleword 8LS:D-form
* [code:: lfdp] Load Floating-Point Double Pair DS-form
* [code:: lxsd] Load VSX Scalar Doubleword DS-form
* [code:: lxssp] Load VSX Scalar Single-Precision DS-form
* [code:: plxvp] Prefixed Load VSX Vector Paired 8LS:D-form
* [code:: ld] Load Doubleword DS-form
* [code:: ldu] Load Doubleword with Update DS-form
* [code:: lwa] Load Word Algebraic DS-form
* [code:: dscli.] DFP Shift Significand Left Immediate Z22-form
* [code:: dscli] DFP Shift Significand Left Immediate Z22-form
* [code:: dscri.] DFP Shift Significand Right Immediate Z22-form
* [code:: dscri] DFP Shift Significand Right Immediate Z22-form
* [code:: dtstdc] DFP Test Data Class Z22-form
* [code:: dtstdg] DFP Test Data Group Z22-form
* [code:: dadd.] DFP Add X-form
* [code:: dadd] DFP Add X-form
* [code:: dmul.] DFP Multiply X-form
* [code:: dmul] DFP Multiply X-form
* [code:: dcmpo] DFP Compare Ordered X-form
* [code:: dtstex] DFP Test Exponent X-form
* [code:: dctdp.] DFP Convert To DFP Long X-form
* [code:: dctdp] DFP Convert To DFP Long X-form
* [code:: dctfix.] DFP Convert To Fixed X-form
* [code:: dctfix] DFP Convert To Fixed X-form
* [code:: ddedpd.] DFP Decode DPD To BCD X-form
* [code:: ddedpd] DFP Decode DPD To BCD X-form
* [code:: dxex.] DFP Extract Biased Exponent X-form
* [code:: dxex] DFP Extract Biased Exponent X-form
* [code:: dsub.] DFP Subtract X-form
* [code:: dsub] DFP Subtract X-form
* [code:: ddiv.] DFP Divide X-form
* [code:: ddiv] DFP Divide X-form
* [code:: dcmpu] DFP Compare Unordered X-form
* [code:: dtstsf] DFP Test Significance X-form
* [code:: drsp.] DFP Round To DFP Short X-form
* [code:: drsp] DFP Round To DFP Short X-form
* [code:: dcffix.] DFP Convert From Fixed X-form
* [code:: dcffix] DFP Convert From Fixed X-form
* [code:: denbcd.] DFP Encode BCD To DPD X-form
* [code:: denbcd] DFP Encode BCD To DPD X-form
* [code:: diex.] DFP Insert Biased Exponent X-form
* [code:: diex] DFP Insert Biased Exponent X-form
* [code:: dqua.] DFP Quantize Z23-form
* [code:: dqua] DFP Quantize Z23-form
* [code:: drrnd.] DFP Reround Z23-form
* [code:: drrnd] DFP Reround Z23-form
* [code:: dquai.] DFP Quantize Immediate Z23-form
* [code:: dquai] DFP Quantize Immediate Z23-form
* [code:: drintx.] DFP Round To FP Integer With Inexact Z23-form
* [code:: drintx] DFP Round To FP Integer With Inexact Z23-form
* [code:: drintn.] DFP Round To FP Integer Without Inexact Z23-form
* [code:: drintn] DFP Round To FP Integer Without Inexact Z23-form
* [code:: dtstsfi] DFP Test Significance Immediate X-form
* [code:: xvi8ger4pp] VSX Vector 8-bit Signed/Unsigned Integer GER (rank-4 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvi8ger4pp] Prefixed Masked VSX Vector 8-bit Signed/Unsigned Integer GER (rank-4 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf16ger2pp] VSX Vector 16-bit Floating-Point GER (rank-2 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvf16ger2pp] Prefixed Masked VSX Vector 16-bit Floating-Point GER (rank-2 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf32gerpp] VSX Vector 32-bit Floating-Point GER (rank-1 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvf32gerpp] Prefixed Masked VSX Vector 32-bit Floating-Point GER (rank-1 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvi4ger8pp] VSX Vector 4-bit Signed Integer GER (rank-8 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvi4ger8pp] Prefixed Masked VSX Vector 4-bit Signed Integer GER (rank-8 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvi16ger2spp] VSX Vector 16-bit Signed Integer GER (rank-2 update) with Saturation Positive multiply, Positive accumulate XX3-form
* [code:: pmxvi16ger2spp] Prefixed Masked VSX Vector 16-bit Signed Integer GER (rank-2 update) with Saturation Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvbf16ger2pp] VSX Vector bfloat16 GER (rank-2 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvbf16ger2pp] Prefixed Masked VSX Vector bfloat16 GER (rank-2 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf64gerpp] VSX Vector 64-bit Floating-Point GER (rank-1 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvf64gerpp] Prefixed Masked VSX Vector 64-bit Floating-Point GER (rank-1 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf16ger2np] VSX Vector 16-bit Floating-Point GER (rank-2 update) Negative multiply, Positive accumulate XX3-form
* [code:: pmxvf16ger2np] Prefixed Masked VSX Vector 16-bit Floating-Point GER (rank-2 update) Negative multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf32gernp] VSX Vector 32-bit Floating-Point GER (rank-1 update) Negative multiply, Positive accumulate XX3-form
* [code:: pmxvf32gernp] Prefixed Masked VSX Vector 32-bit Floating-Point GER (rank-1 update) Negative multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvbf16ger2np] VSX Vector bfloat16 GER (rank-2 update) Negative multiply, Positive accumulate XX3-form
* [code:: pmxvbf16ger2np] Prefixed Masked VSX Vector bfloat16 GER (rank-2 update) Negative multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf64gernp] VSX Vector 64-bit Floating-Point GER (rank-1 update) Negative multiply, Positive accumulate XX3-form
* [code:: pmxvf64gernp] Prefixed Masked VSX Vector 64-bit Floating-Point GER (rank-1 update) Negative multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvf16ger2pn] VSX Vector 16-bit Floating-Point GER (rank-2 update) Positive multiply, Negative accumulate XX3-form
* [code:: pmxvf16ger2pn] Prefixed Masked VSX Vector 16-bit Floating-Point GER (rank-2 update) Positive multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvf32gerpn] VSX Vector 32-bit Floating-Point GER (rank-1 update) Positive multiply, Negative accumulate XX3-form
* [code:: pmxvf32gerpn] Prefixed Masked VSX Vector 32-bit Floating-Point GER (rank-1 update) Positive multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvbf16ger2pn] VSX Vector bfloat16 GER (rank-2 update) Positive multiply, Negative accumulate XX3-form
* [code:: pmxvbf16ger2pn] Prefixed Masked VSX Vector bfloat16 GER (rank-2 update) Positive multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvf64gerpn] VSX Vector 64-bit Floating-Point GER (rank-1 update) Positive multiply, Negative accumulate XX3-form
* [code:: pmxvf64gerpn] Prefixed Masked VSX Vector 64-bit Floating-Point GER (rank-1 update) Positive multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvf16ger2nn] VSX Vector 16-bit Floating-Point GER (rank-2 update) Negative multiply, Negative accumulate XX3-form
* [code:: pmxvf16ger2nn] Prefixed Masked VSX Vector 16-bit Floating-Point GER (rank-2 update) Negative multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvf32gernn] VSX Vector 32-bit Floating-Point GER (rank-1 update) Negative multiply, Negative accumulate XX3-form
* [code:: pmxvf32gernn] Prefixed Masked VSX Vector 32-bit Floating-Point GER (rank-1 update) Negative multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvbf16ger2nn] VSX Vector bfloat16 GER (rank-2 update) Negative multiply, Negative accumulate XX3-form
* [code:: pmxvbf16ger2nn] Prefixed Masked VSX Vector bfloat16 GER (rank-2 update) Negative multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvf64gernn] VSX Vector 64-bit Floating-Point GER (rank-1 update) Negative multiply, Negative accumulate XX3-form
* [code:: pmxvf64gernn] Prefixed Masked VSX Vector 64-bit Floating-Point GER (rank-1 update) Negative multiply, Negative accumulate MMIRR:XX3-form
* [code:: xvi8ger4] VSX Vector 8-bit Signed/Unsigned Integer GER (rank-4 update) XX3-form
* [code:: pmxvi8ger4] Prefixed Masked VSX Vector 8-bit Signed/Unsigned Integer GER (rank-4 update) MMIRR:XX3-form
* [code:: xvf16ger2] VSX Vector 16-bit Floating-Point GER (rank-2 update) XX3-form
* [code:: pmxvf16ger2] Prefixed Masked VSX Vector 16-bit Floating-Point GER (rank-2 update) MMIRR:XX3-form
* [code:: xvf32ger] VSX Vector 32-bit Floating-Point GER (rank-1 update) XX3-form
* [code:: pmxvf32ger] Prefixed Masked VSX Vector 32-bit Floating-Point GER (rank-1 update) MMIRR:XX3-form
* [code:: xvi4ger8] VSX Vector 4-bit Signed Integer GER (rank-8 update) XX3-form
* [code:: pmxvi4ger8] Prefixed Masked VSX Vector 4-bit Signed Integer GER (rank-8 update) MMIRR:XX3-form
* [code:: xvi16ger2s] VSX Vector 16-bit Signed Integer GER (rank-2 update) with Saturation XX3-form
* [code:: pmxvi16ger2s] Prefixed Masked VSX Vector 16-bit Signed Integer GER (rank-2 update) with Saturation MMIRR:XX3-form
* [code:: xvbf16ger2] VSX Vector bfloat16 GER (rank-2 update) XX3-form
* [code:: pmxvbf16ger2] Prefixed Masked VSX Vector bfloat16 GER (rank-2 update) MMIRR:XX3-form
* [code:: xvf64ger] VSX Vector 64-bit Floating-Point GER (rank-1 update) XX3-form
* [code:: pmxvf64ger] Prefixed Masked VSX Vector 64-bit Floating-Point GER (rank-1 update) MMIRR:XX3-form
* [code:: xvi16ger2] VSX Vector 16-bit Signed Integer GER (rank-2 update) XX3-form
* [code:: pmxvi16ger2] Prefixed Masked VSX Vector 16-bit Signed Integer GER (rank-2 update) MMIRR:XX3-form
* [code:: xvi8ger4spp] VSX Vector 8-bit Signed/Unsigned Integer GER (rank-4 update) with Saturation Positive multiply, Positive accumulate XX3-form
* [code:: pmxvi8ger4spp] Prefixed Masked VSX Vector 8-bit Signed/Unsigned Integer GER (rank-4 update) with Saturation Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: xvi16ger2pp] VSX Vector 16-bit Signed Integer GER (rank-2 update) Positive multiply, Positive accumulate XX3-form
* [code:: pmxvi16ger2pp] Prefixed Masked VSX Vector 16-bit Signed Integer GER (rank-2 update) Positive multiply, Positive accumulate MMIRR:XX3-form
* [code:: fcfids.] Floating Convert with round Signed Doubleword to Single-Precision format X-form
* [code:: fcfids] Floating Convert with round Signed Doubleword to Single-Precision format X-form
* [code:: fcfidus.] Floating Convert with round Unsigned Doubleword to Single-Precision format X-form
* [code:: fcfidus] Floating Convert with round Unsigned Doubleword to Single-Precision format X-form
* [code:: fdivs.] Floating Divide Single A-form
* [code:: fdivs] Floating Divide Single A-form
* [code:: fsubs.] Floating Subtract Single A-form
* [code:: fsubs] Floating Subtract Single A-form
* [code:: fadds.] Floating Add Single A-form
* [code:: fadds] Floating Add Single A-form
* [code:: fsqrts.] Floating Square Root Single A-form
* [code:: fsqrts] Floating Square Root Single A-form
* [code:: fres.] Floating Reciprocal Estimate Single A-form
* [code:: fres] Floating Reciprocal Estimate Single A-form
* [code:: fmuls.] Floating Multiply Single A-form
* [code:: fmuls] Floating Multiply Single A-form
* [code:: frsqrtes.] Floating Reciprocal Square Root Estimate Single A-form
* [code:: frsqrtes] Floating Reciprocal Square Root Estimate Single A-form
* [code:: fmsubs.] Floating Multiply-Subtract Single A-form
* [code:: fmsubs] Floating Multiply-Subtract Single A-form
* [code:: fmadds.] Floating Multiply-Add Single A-form
* [code:: fmadds] Floating Multiply-Add Single A-form
* [code:: fnmsubs.] Floating Negative Multiply-Subtract Single A-form
* [code:: fnmsubs] Floating Negative Multiply-Subtract Single A-form
* [code:: fnmadds.] Floating Negative Multiply-Add Single A-form
* [code:: fnmadds] Floating Negative Multiply-Add Single A-form
* [code:: pstq] Prefixed Store Quadword 8LS:D-form
* [code:: xsaddsp] VSX Scalar Add Single-Precision XX3-form
* [code:: xssubsp] VSX Scalar Subtract Single-Precision XX3-form
* [code:: xsmulsp] VSX Scalar Multiply Single-Precision XX3-form
* [code:: xsdivsp] VSX Scalar Divide Single-Precision XX3-form
* [code:: xsadddp] VSX Scalar Add Double-Precision XX3-form
* [code:: xssubdp] VSX Scalar Subtract Double-Precision XX3-form
* [code:: xsmuldp] VSX Scalar Multiply Double-Precision XX3-form
* [code:: xsdivdp] VSX Scalar Divide Double-Precision XX3-form
* [code:: xvaddsp] VSX Vector Add Single-Precision XX3-form
* [code:: xvsubsp] VSX Vector Subtract Single-Precision XX3-form
* [code:: xvmulsp] VSX Vector Multiply Single-Precision XX3-form
* [code:: xvdivsp] VSX Vector Divide Single-Precision XX3-form
* [code:: xvadddp] VSX Vector Add Double-Precision XX3-form
* [code:: xvsubdp] VSX Vector Subtract Double-Precision XX3-form
* [code:: xvmuldp] VSX Vector Multiply Double-Precision XX3-form
* [code:: xvdivdp] VSX Vector Divide Double-Precision XX3-form
* [code:: xsmaxcdp] VSX Scalar Maximum Type-C Double-Precision XX3-form
* [code:: xsmincdp] VSX Scalar Minimum Type-C Double-Precision XX3-form
* [code:: xsmaxjdp] VSX Scalar Maximum Type-J Double-Precision XX3-form
* [code:: xsminjdp] VSX Scalar Minimum Type-J Double-Precision XX3-form
* [code:: xsmaxdp] VSX Scalar Maximum Double-Precision XX3-form
* [code:: xsmindp] VSX Scalar Minimum Double-Precision XX3-form
* [code:: xscpsgndp] VSX Scalar Copy Sign Double-Precision XX3-form
* [code:: xvmaxsp] VSX Vector Maximum Single-Precision XX3-form
* [code:: xvminsp] VSX Vector Minimum Single-Precision XX3-form
* [code:: xvcpsgnsp] VSX Vector Copy Sign Single-Precision XX3-form
* [code:: xviexpsp] VSX Vector Insert Exponent Single-Precision XX3-form
* [code:: xvmaxdp] VSX Vector Maximum Double-Precision XX3-form
* [code:: xvmindp] VSX Vector Minimum Double-Precision XX3-form
* [code:: xvcpsgndp] VSX Vector Copy Sign Double-Precision XX3-form
* [code:: xviexpdp] VSX Vector Insert Exponent Double-Precision XX3-form
* [code:: xsmaddasp] VSX Scalar Multiply-Add Type-A Single-Precision XX3-form
* [code:: xsmaddmsp] VSX Scalar Multiply-Add Type-M Single-Precision XX3-form
* [code:: xsmsubasp] VSX Scalar Multiply-Subtract Type-A Single-Precision XX3-form
* [code:: xsmsubmsp] VSX Scalar Multiply-Subtract Type-M Single-Precision XX3-form
* [code:: xsmaddadp] VSX Scalar Multiply-Add Type-A Double-Precision XX3-form
* [code:: xsmaddmdp] VSX Scalar Multiply-Add Type-M Double-Precision XX3-form
* [code:: xsmsubadp] VSX Scalar Multiply-Subtract Type-A Double-Precision XX3-form
* [code:: xsmsubmdp] VSX Scalar Multiply-Subtract Type-M Double-Precision XX3-form
* [code:: xvmaddasp] VSX Vector Multiply-Add Type-A Single-Precision XX3-form
* [code:: xvmaddmsp] VSX Vector Multiply-Add Type-M Single-Precision XX3-form
* [code:: xvmsubasp] VSX Vector Multiply-Subtract Type-A Single-Precision XX3-form
* [code:: xvmsubmsp] VSX Vector Multiply-Subtract Type-M Single-Precision XX3-form
* [code:: xvmaddadp] VSX Vector Multiply-Add Type-A Double-Precision XX3-form
* [code:: xvmaddmdp] VSX Vector Multiply-Add Type-M Double-Precision XX3-form
* [code:: xvmsubadp] VSX Vector Multiply-Subtract Type-A Double-Precision XX3-form
* [code:: xvmsubmdp] VSX Vector Multiply-Subtract Type-M Double-Precision XX3-form
* [code:: xsnmaddasp] VSX Scalar Negative Multiply-Add Type-A Single-Precision XX3-form
* [code:: xsnmaddmsp] VSX Scalar Negative Multiply-Add Type-M Single-Precision XX3-form
* [code:: xsnmsubasp] VSX Scalar Negative Multiply-Subtract Type-A Single-Precision XX3-form
* [code:: xsnmsubmsp] VSX Scalar Negative Multiply-Subtract Type-M Single-Precision XX3-form
* [code:: xsnmaddadp] VSX Scalar Negative Multiply-Add Type-A Double-Precision XX3-form
* [code:: xsnmaddmdp] VSX Scalar Negative Multiply-Add Type-M Double-Precision XX3-form
* [code:: xsnmsubadp] VSX Scalar Negative Multiply-Subtract Type-A Double-Precision XX3-form
* [code:: xsnmsubmdp] VSX Scalar Negative Multiply-Subtract Type-M Double-Precision XX3-form
* [code:: xvnmaddasp] VSX Vector Negative Multiply-Add Type-A Single-Precision XX3-form
* [code:: xvnmaddmsp] VSX Vector Negative Multiply-Add Type-M Single-Precision XX3-form
* [code:: xvnmsubasp] VSX Vector Negative Multiply-Subtract Type-A Single-Precision XX3-form
* [code:: xvnmsubmsp] VSX Vector Negative Multiply-Subtract Type-M Single-Precision XX3-form
* [code:: xvnmaddadp] VSX Vector Negative Multiply-Add Type-A Double-Precision XX3-form
* [code:: xvnmaddmdp] VSX Vector Negative Multiply-Add Type-M Double-Precision XX3-form
* [code:: xvnmsubadp] VSX Vector Negative Multiply-Subtract Type-A Double-Precision XX3-form
* [code:: xvnmsubmdp] VSX Vector Negative Multiply-Subtract Type-M Double-Precision XX3-form
* [code:: xxsldwi] VSX Vector Shift Left Double by Word Immediate XX3-form
* [code:: xxpermdi] VSX Vector Permute Doubleword Immediate XX3-form
* [code:: xxmrghw] VSX Vector Merge High Word XX3-form
* [code:: xxperm] VSX Vector Permute XX3-form
* [code:: xxmrglw] VSX Vector Merge Low Word XX3-form
* [code:: xxpermr] VSX Vector Permute Right-indexed XX3-form
* [code:: xxland] VSX Vector Logical AND XX3-form
* [code:: xxlandc] VSX Vector Logical AND with Complement XX3-form
* [code:: xxlor] VSX Vector Logical OR XX3-form
* [code:: xxlxor] VSX Vector Logical XOR XX3-form
* [code:: xxlnor] VSX Vector Logical NOR XX3-form
* [code:: xxlorc] VSX Vector Logical OR with Complement XX3-form
* [code:: xxlnand] VSX Vector Logical NAND XX3-form
* [code:: xxleqv] VSX Vector Logical Equivalence XX3-form
* [code:: xxspltw] VSX Vector Splat Word XX2-form
* [code:: xxspltib] VSX Vector Splat Immediate Byte X-form
* [code:: lxvkq] Load VSX Vector Special Value Quadword X-form
* [code:: xxextractuw] VSX Vector Extract Unsigned Word XX2-form
* [code:: xxinsertw] VSX Vector Insert Word XX2-form
* [code:: xvcmpeqsp.] VSX Vector Compare Equal To Single-Precision XX3-form
* [code:: xvcmpeqsp] VSX Vector Compare Equal To Single-Precision XX3-form
* [code:: xvcmpgtsp.] VSX Vector Compare Greater Than Single-Precision XX3-form
* [code:: xvcmpgtsp] VSX Vector Compare Greater Than Single-Precision XX3-form
* [code:: xvcmpgesp.] VSX Vector Compare Greater Than or Equal To Single-Precision XX3-form
* [code:: xvcmpgesp] VSX Vector Compare Greater Than or Equal To Single-Precision XX3-form
* [code:: xvcmpeqdp.] VSX Vector Compare Equal To Double-Precision XX3-form
* [code:: xvcmpeqdp] VSX Vector Compare Equal To Double-Precision XX3-form
* [code:: xvcmpgtdp.] VSX Vector Compare Greater Than Double-Precision XX3-form
* [code:: xvcmpgtdp] VSX Vector Compare Greater Than Double-Precision XX3-form
* [code:: xvcmpgedp.] VSX Vector Compare Greater Than or Equal To Double-Precision XX3-form
* [code:: xvcmpgedp] VSX Vector Compare Greater Than or Equal To Double-Precision XX3-form
* [code:: xscmpeqdp] VSX Scalar Compare Equal Double-Precision XX3-form
* [code:: xscmpgtdp] VSX Scalar Compare Greater Than Double-Precision XX3-form
* [code:: xscmpgedp] VSX Scalar Compare Greater Than or Equal Double-Precision XX3-form
* [code:: xscmpudp] VSX Scalar Compare Unordered Double-Precision XX3-form
* [code:: xscmpodp] VSX Scalar Compare Ordered Double-Precision XX3-form
* [code:: xscmpexpdp] VSX Scalar Compare Exponents Double-Precision XX3-form
* [code:: xscvdpuxws] VSX Scalar Convert with round to zero Double-Precision to Unsigned Word format XX2-form
* [code:: xscvdpsxws] VSX Scalar Convert with round to zero Double-Precision to Signed Word format XX2-form
* [code:: xvcvspuxws] VSX Vector Convert with round to zero Single-Precision to Unsigned Word format XX2-form
* [code:: xvcvspsxws] VSX Vector Convert with round to zero Single-Precision to Signed Word format XX2-form
* [code:: xvcvuxwsp] VSX Vector Convert with round Unsigned Word to Single-Precision format XX2-form
* [code:: xvcvsxwsp] VSX Vector Convert with round Signed Word to Single-Precision format XX2-form
* [code:: xvcvdpuxws] VSX Vector Convert with round to zero Double-Precision to Unsigned Word format XX2-form
* [code:: xvcvdpsxws] VSX Vector Convert with round to zero Double-Precision to Signed Word format XX2-form
* [code:: xvcvuxwdp] VSX Vector Convert Unsigned Word to Double-Precision format XX2-form
* [code:: xvcvsxwdp] VSX Vector Convert Signed Word to Double-Precision format XX2-form
* [code:: xscvuxdsp] VSX Scalar Convert with round Unsigned Doubleword to Single-Precision format XX2-form
* [code:: xscvsxdsp] VSX Scalar Convert with round Signed Doubleword to Single-Precision format XX2-form
* [code:: xscvdpuxds] VSX Scalar Convert with round to zero Double-Precision to Unsigned Doubleword format XX2-form
* [code:: xscvdpsxds] VSX Scalar Convert with round to zero Double-Precision to Signed Doubleword format XX2-form
* [code:: xscvuxddp] VSX Scalar Convert with round Unsigned Doubleword to Double-Precision format XX2-form
* [code:: xscvsxddp] VSX Scalar Convert with round Signed Doubleword to Double-Precision format XX2-form
* [code:: xvcvspuxds] VSX Vector Convert with round to zero Single-Precision to Unsigned Doubleword format XX2-form
* [code:: xvcvspsxds] VSX Vector Convert with round to zero Single-Precision to Signed Doubleword format XX2-form
* [code:: xvcvuxdsp] VSX Vector Convert with round Unsigned Doubleword to Single-Precision format XX2-form
* [code:: xvcvsxdsp] VSX Vector Convert with round Signed Doubleword to Single-Precision format XX2-form
* [code:: xvcvdpuxds] VSX Vector Convert with round to zero Double-Precision to Unsigned Doubleword format XX2-form
* [code:: xvcvdpsxds] VSX Vector Convert with round to zero Double-Precision to Signed Doubleword format XX2-form
* [code:: xvcvuxddp] VSX Vector Convert with round Unsigned Doubleword to Double-Precision format XX2-form
* [code:: xvcvsxddp] VSX Vector Convert with round Signed Doubleword to Double-Precision format XX2-form
* [code:: xsrdpi] VSX Scalar Round to Double-Precision Integer using round to Nearest Away XX2-form
* [code:: xsrdpiz] VSX Scalar Round to Double-Precision Integer using round toward Zero XX2-form
* [code:: xsrdpip] VSX Scalar Round to Double-Precision Integer using round toward +Infinity XX2-form
* [code:: xsrdpim] VSX Scalar Round to Double-Precision Integer using round toward -Infinity XX2-form
* [code:: xvrspi] VSX Vector Round to Single-Precision Integer using round to Nearest Away XX2-form
* [code:: xvrspiz] VSX Vector Round to Single-Precision Integer using round toward Zero XX2-form
* [code:: xvrspip] VSX Vector Round to Single-Precision Integer using round toward +Infinity XX2-form
* [code:: xvrspim] VSX Vector Round to Single-Precision Integer using round toward -Infinity XX2-form
* [code:: xvrdpi] VSX Vector Round to Double-Precision Integer using round to Nearest Away XX2-form
* [code:: xvrdpiz] VSX Vector Round to Double-Precision Integer using round toward Zero XX2-form
* [code:: xvrdpip] VSX Vector Round to Double-Precision Integer using round toward +Infinity XX2-form
* [code:: xvrdpim] VSX Vector Round to Double-Precision Integer using round toward -Infinity XX2-form
* [code:: xscvdpsp] VSX Scalar Convert with round Double-Precision to Single-Precision format XX2-form
* [code:: xsrsp] VSX Scalar Round to Single-Precision XX2-form
* [code:: xscvspdp] VSX Scalar Convert Single-Precision to Double-Precision format XX2-form
* [code:: xsabsdp] VSX Scalar Absolute Double-Precision XX2-form
* [code:: xsnabsdp] VSX Scalar Negative Absolute Double-Precision XX2-form
* [code:: xsnegdp] VSX Scalar Negate Double-Precision XX2-form
* [code:: xvcvdpsp] VSX Vector Convert with round Double-Precision to Single-Precision format XX2-form
* [code:: xvabssp] VSX Vector Absolute Single-Precision XX2-form
* [code:: xvnabssp] VSX Vector Negative Absolute Single-Precision XX2-form
* [code:: xvnegsp] VSX Vector Negate Single-Precision XX2-form
* [code:: xvcvspdp] VSX Vector Convert Single-Precision to Double-Precision format XX2-form
* [code:: xvabsdp] VSX Vector Absolute Double-Precision XX2-form
* [code:: xvnabsdp] VSX Vector Negative Absolute Double-Precision XX2-form
* [code:: xvnegdp] VSX Vector Negate Double-Precision XX2-form
* [code:: xstdivdp] VSX Scalar Test for software Divide Double-Precision XX3-form
* [code:: xvtdivsp] VSX Vector Test for software Divide Single-Precision XX3-form
* [code:: xvtdivdp] VSX Vector Test for software Divide Double-Precision XX3-form
* [code:: xvtstdcsp] VSX Vector Test Data Class Single-Precision XX2-form
* [code:: xvtstdcdp] VSX Vector Test Data Class Double-Precision XX2-form
* [code:: xsrsqrtesp] VSX Scalar Reciprocal Square Root Estimate Single-Precision XX2-form
* [code:: xsresp] VSX Scalar Reciprocal Estimate Single-Precision XX2-form
* [code:: xsrsqrtedp] VSX Scalar Reciprocal Square Root Estimate Double-Precision XX2-form
* [code:: xsredp] VSX Scalar Reciprocal Estimate Double-Precision XX2-form
* [code:: xstsqrtdp] VSX Scalar Test for software Square Root Double-Precision XX2-form
* [code:: xvrsqrtesp] VSX Vector Reciprocal Square Root Estimate Single-Precision XX2-form
* [code:: xvresp] VSX Vector Reciprocal Estimate Single-Precision XX2-form
* [code:: xvtsqrtsp] VSX Vector Test for software Square Root Single-Precision XX2-form
* [code:: xvrsqrtedp] VSX Vector Reciprocal Square Root Estimate Double-Precision XX2-form
* [code:: xvredp] VSX Vector Reciprocal Estimate Double-Precision XX2-form
* [code:: xvtsqrtdp] VSX Vector Test for software Square Root Double-Precision XX2-form
* [code:: xststdcsp] VSX Scalar Test Data Class Single-Precision XX2-form
* [code:: xststdcdp] VSX Scalar Test Data Class Double-Precision XX2-form
* [code:: xxgenpcvbm] VSX Vector Generate PCV from Byte Mask X-form
* [code:: xxgenpcvwm] VSX Vector Generate PCV from Word Mask X-form
* [code:: xxgenpcvhm] VSX Vector Generate PCV from Halfword Mask X-form
* [code:: xxgenpcvdm] VSX Vector Generate PCV from Doubleword Mask X-form
* [code:: xssqrtsp] VSX Scalar Square Root Single-Precision XX2-form
* [code:: xssqrtdp] VSX Scalar Square Root Double-Precision XX2-form
* [code:: xsrdpic] VSX Scalar Round to Double-Precision Integer exact using Current rounding mode XX2-form
* [code:: xvsqrtsp] VSX Vector Square Root Single-Precision XX2-form
* [code:: xvrspic] VSX Vector Round to Single-Precision Integer Exact using Current rounding mode XX2-form
* [code:: xvsqrtdp] VSX Vector Square Root Double-Precision XX2-form
* [code:: xvrdpic] VSX Vector Round to Double-Precision Integer Exact using Current rounding mode XX2-form
* [code:: xscvdpspn] VSX Scalar Convert Scalar Single-Precision to Vector Single-Precision format Non-signalling XX2-form
* [code:: xscvspdpn] VSX Scalar Convert Single-Precision to Double-Precision format Non-signalling XX2-form
* [code:: xsxexpdp] VSX Scalar Extract Exponent Double-Precision XX2-form
* [code:: xsxsigdp] VSX Scalar Extract Significand Double-Precision XX2-form
* [code:: xscvhpdp] VSX Scalar Convert Half-Precision to Double-Precision format XX2-form
* [code:: xscvdphp] VSX Scalar Convert with round Double-Precision to Half-Precision format XX2-form
* [code:: xvxexpdp] VSX Vector Extract Exponent Double-Precision XX2-form
* [code:: xvxsigdp] VSX Vector Extract Significand Double-Precision XX2-form
* [code:: xvtlsbb] VSX Vector Test Least-Significant Bit by Byte XX2-form
* [code:: xxbrh] VSX Vector Byte-Reverse Halfword XX2-form
* [code:: xvxexpsp] VSX Vector Extract Exponent Single-Precision XX2-form
* [code:: xvxsigsp] VSX Vector Extract Significand Single-Precision XX2-form
* [code:: xxbrw] VSX Vector Byte-Reverse Word XX2-form
* [code:: xvcvbf16spn] VSX Vector Convert bfloat16 to Single-Precision format Non-signaling XX2-form
* [code:: xvcvspbf16] VSX Vector Convert with round Single-Precision to bfloat16 format XX2-form
* [code:: xxbrd] VSX Vector Byte-Reverse Doubleword XX2-form
* [code:: xvcvhpsp] VSX Vector Convert Half-Precision to Single-Precision format XX2-form
* [code:: xvcvsphp] VSX Vector Convert with round Single-Precision to Half-Precision format XX2-form
* [code:: xxbrq] VSX Vector Byte-Reverse Quadword XX2-form
* [code:: xsiexpdp] VSX Scalar Insert Exponent Double-Precision X-form
* [code:: xxsel] VSX Vector Select XX4-form
* [code:: pstd] Prefixed Store Doubleword 8LS:D-form
* [code:: stfdp] Store Floating-Point Double Pair DS-form
* [code:: stxsd] Store VSX Scalar Doubleword DS-form
* [code:: stxssp] Store VSX Scalar Single-Precision DS-form
* [code:: lxv] Load VSX Vector DQ-form
* [code:: stxv] Store VSX Vector DQ-form
* [code:: pstxvp] Prefixed Store VSX Vector Paired 8LS:D-form
* [code:: std] Store Doubleword DS-form
* [code:: stdu] Store Doubleword with Update DS-form
* [code:: stq] Store Quadword DS-form
* [code:: fcmpu] Floating Compare Unordered X-form
* [code:: fcmpo] Floating Compare Ordered X-form
* [code:: mcrfs] Move to Condition Register from FPSCR X-form
* [code:: ftdiv] Floating Test for software Divide X-form
* [code:: ftsqrt] Floating Test for software Square Root X-form
* [code:: dscliq.] DFP Shift Significand Left Immediate Quad Z22-form
* [code:: dscliq] DFP Shift Significand Left Immediate Quad Z22-form
* [code:: dscriq.] DFP Shift Significand Right Immediate Quad Z22-form
* [code:: dscriq] DFP Shift Significand Right Immediate Quad Z22-form
* [code:: dtstdcq] DFP Test Data Class Quad Z22-form
* [code:: dtstdgq] DFP Test Data Group Quad Z22-form
* [code:: daddq.] DFP Add Quad X-form
* [code:: daddq] DFP Add Quad X-form
* [code:: dmulq.] DFP Multiply Quad X-form
* [code:: dmulq] DFP Multiply Quad X-form
* [code:: dcmpoq] DFP Compare Ordered Quad X-form
* [code:: dtstexq] DFP Test Exponent Quad X-form
* [code:: dctqpq.] DFP Convert To DFP Extended X-form
* [code:: dctqpq] DFP Convert To DFP Extended X-form
* [code:: dctfixq.] DFP Convert To Fixed Quad X-form
* [code:: dctfixq] DFP Convert To Fixed Quad X-form
* [code:: ddedpdq.] DFP Decode DPD To BCD Quad X-form
* [code:: ddedpdq] DFP Decode DPD To BCD Quad X-form
* [code:: dxexq.] DFP Extract Biased Exponent Quad X-form
* [code:: dxexq] DFP Extract Biased Exponent Quad X-form
* [code:: dsubq.] DFP Subtract Quad X-form
* [code:: dsubq] DFP Subtract Quad X-form
* [code:: ddivq.] DFP Divide Quad X-form
* [code:: ddivq] DFP Divide Quad X-form
* [code:: dcmpuq] DFP Compare Unordered Quad X-form
* [code:: dtstsfq] DFP Test Significance Quad X-form
* [code:: drdpq.] DFP Round To DFP Long X-form
* [code:: drdpq] DFP Round To DFP Long X-form
* [code:: dcffixq.] DFP Convert From Fixed Quad X-form
* [code:: dcffixq] DFP Convert From Fixed Quad X-form
* [code:: denbcdq.] DFP Encode BCD To DPD Quad X-form
* [code:: denbcdq] DFP Encode BCD To DPD Quad X-form
* [code:: diexq.] DFP Insert Biased Exponent Quad X-form
* [code:: diexq] DFP Insert Biased Exponent Quad X-form
* [code:: dcffixqq] DFP Convert From Fixed Quadword Quad X-form
* [code:: dctfixqq] DFP Convert To Fixed Quadword Quad X-form
* [code:: dquaq.] DFP Quantize Quad Z23-form
* [code:: dquaq] DFP Quantize Quad Z23-form
* [code:: drrndq.] DFP Reround Quad Z23-form
* [code:: drrndq] DFP Reround Quad Z23-form
* [code:: dquaiq.] DFP Quantize Immediate Quad Z23-form
* [code:: dquaiq] DFP Quantize Immediate Quad Z23-form
* [code:: drintxq.] DFP Round To FP Integer With Inexact Quad Z23-form
* [code:: drintxq] DFP Round To FP Integer With Inexact Quad Z23-form
* [code:: drintnq.] DFP Round To FP Integer Without Inexact Quad Z23-form
* [code:: drintnq] DFP Round To FP Integer Without Inexact Quad Z23-form
* [code:: dtstsfiq] DFP Test Significance Immediate Quad X-form
* [code:: xsaddqpo] VSX Scalar Add Quad-Precision [using round to Odd] X-form
* [code:: xsaddqp] VSX Scalar Add Quad-Precision [using round to Odd] X-form
* [code:: xsmulqpo] VSX Scalar Multiply Quad-Precision [using round to Odd] X-form
* [code:: xsmulqp] VSX Scalar Multiply Quad-Precision [using round to Odd] X-form
* [code:: xscmpeqqp] VSX Scalar Compare Equal Quad-Precision X-form
* [code:: xscpsgnqp] VSX Scalar Copy Sign Quad-Precision X-form
* [code:: xscmpoqp] VSX Scalar Compare Ordered Quad-Precision X-form
* [code:: xscmpexpqp] VSX Scalar Compare Exponents Quad-Precision X-form
* [code:: xscmpgeqp] VSX Scalar Compare Greater Than or Equal Quad-Precision X-form
* [code:: xscmpgtqp] VSX Scalar Compare Greater Than Quad-Precision X-form
* [code:: xsmaddqpo] VSX Scalar Multiply-Add Quad-Precision [using round to Odd] X-form
* [code:: xsmaddqp] VSX Scalar Multiply-Add Quad-Precision [using round to Odd] X-form
* [code:: xsmsubqpo] VSX Scalar Multiply-Subtract Quad-Precision [using round to Odd] X-form
* [code:: xsmsubqp] VSX Scalar Multiply-Subtract Quad-Precision [using round to Odd] X-form
* [code:: xsnmaddqpo] VSX Scalar Negative Multiply-Add Quad-Precision [using round to Odd] X-form
* [code:: xsnmaddqp] VSX Scalar Negative Multiply-Add Quad-Precision [using round to Odd] X-form
* [code:: xsnmsubqpo] VSX Scalar Negative Multiply-Subtract Quad-Precision [using round to Odd] X-form
* [code:: xsnmsubqp] VSX Scalar Negative Multiply-Subtract Quad-Precision [using round to Odd] X-form
* [code:: xssubqpo] VSX Scalar Subtract Quad-Precision [using round to Odd] X-form
* [code:: xssubqp] VSX Scalar Subtract Quad-Precision [using round to Odd] X-form
* [code:: xsdivqpo] VSX Scalar Divide Quad-Precision [using round to Odd] X-form
* [code:: xsdivqp] VSX Scalar Divide Quad-Precision [using round to Odd] X-form
* [code:: xscmpuqp] VSX Scalar Compare Unordered Quad-Precision X-form
* [code:: xsmaxcqp] VSX Scalar Maximum Type-C Quad-Precision X-form
* [code:: xststdcqp] VSX Scalar Test Data Class Quad-Precision X-form
* [code:: xsmincqp] VSX Scalar Minimum Type-C Quad-Precision X-form
* [code:: xsabsqp] VSX Scalar Absolute Quad-Precision X-form
* [code:: xsxexpqp] VSX Scalar Extract Exponent Quad-Precision X-form
* [code:: xsnabsqp] VSX Scalar Negative Absolute Quad-Precision X-form
* [code:: xsnegqp] VSX Scalar Negate Quad-Precision X-form
* [code:: xsxsigqp] VSX Scalar Extract Significand Quad-Precision X-form
* [code:: xssqrtqpo] VSX Scalar Square Root Quad-Precision [using round to Odd] X-form
* [code:: xssqrtqp] VSX Scalar Square Root Quad-Precision [using round to Odd] X-form
* [code:: xscvqpuqz] VSX Scalar Convert with round to zero Quad-Precision to Unsigned Quadword X-form
* [code:: xscvqpuwz] VSX Scalar Convert with round to zero Quad-Precision to Unsigned Word format X-form
* [code:: xscvudqp] VSX Scalar Convert Unsigned Doubleword to Quad-Precision format X-form
* [code:: xscvuqqp] VSX Scalar Convert with round Unsigned Quadword to Quad-Precision format X-form
* [code:: xscvqpsqz] VSX Scalar Convert with round to zero Quad-Precision to Signed Quadword X-form
* [code:: xscvqpswz] VSX Scalar Convert with round to zero Quad-Precision to Signed Word format X-form
* [code:: xscvsdqp] VSX Scalar Convert Signed Doubleword to Quad-Precision format X-form
* [code:: xscvsqqp] VSX Scalar Convert with round Signed Quadword to Quad-Precision X-form
* [code:: xscvqpudz] VSX Scalar Convert with round to zero Quad-Precision to Unsigned Doubleword format X-form
* [code:: xscvqpdpo] VSX Scalar Convert with round Quad-Precision to Double-Precision format [using round to Odd] X-form
* [code:: xscvqpdp] VSX Scalar Convert with round Quad-Precision to Double-Precision format [using round to Odd] X-form
* [code:: xscvdpqp] VSX Scalar Convert Double-Precision to Quad-Precision format X-form
* [code:: xscvqpsdz] VSX Scalar Convert with round to zero Quad-Precision to Signed Doubleword format X-form
* [code:: xsiexpqp] VSX Scalar Insert Exponent Quad-Precision X-form
* [code:: xsrqpix] VSX Scalar Round to Quad-Precision Integer [with Inexact] Z23-form
* [code:: xsrqpi] VSX Scalar Round to Quad-Precision Integer [with Inexact] Z23-form
* [code:: xsrqpxp] VSX Scalar Round Quad-Precision to Double-Extended-Precision Z23-form
* [code:: mtfsb1.] Move To FPSCR Bit 1 X-form
* [code:: mtfsb1] Move To FPSCR Bit 1 X-form
* [code:: mtfsb0.] Move To FPSCR Bit 0 X-form
* [code:: mtfsb0] Move To FPSCR Bit 0 X-form
* [code:: mtfsfi.] Move To FPSCR Field Immediate X-form
* [code:: mtfsfi] Move To FPSCR Field Immediate X-form
* [code:: fmrgow] Floating Merge Odd Word X-form
* [code:: fmrgew] Floating Merge Even Word X-form
* [code:: mffs.] Move From FPSCR X-form
* [code:: mffs] Move From FPSCR X-form
* [code:: mffsce] Move From FPSCR & Clear Enables X-form
* [code:: mffscdrn] Move From FPSCR Control & Set DRN X-form
* [code:: mffscdrni] Move From FPSCR Control & Set DRN Immediate X-form
* [code:: mffscrn] Move From FPSCR Control & Set RN X-form
* [code:: mffscrni] Move From FPSCR Control & Set RN Immediate X-form
* [code:: mffsl] Move From FPSCR Lightweight X-form
* [code:: mtfsf.] Move To FPSCR Fields XFL-form
* [code:: mtfsf] Move To FPSCR Fields XFL-form
* [code:: fcpsgn.] Floating Copy Sign X-form
* [code:: fcpsgn] Floating Copy Sign X-form
* [code:: fneg.] Floating Negate X-form
* [code:: fneg] Floating Negate X-form
* [code:: fmr.] Floating Move Register X-form
* [code:: fmr] Floating Move Register X-form
* [code:: fnabs.] Floating Negative Absolute Value X-form
* [code:: fnabs] Floating Negative Absolute Value X-form
* [code:: fabs.] Floating Absolute Value X-form
* [code:: fabs] Floating Absolute Value X-form
* [code:: frin.] Floating Round to Integer Nearest X-form
* [code:: frin] Floating Round to Integer Nearest X-form
* [code:: friz.] Floating Round to Integer Toward Zero X-form
* [code:: friz] Floating Round to Integer Toward Zero X-form
* [code:: frip.] Floating Round to Integer Plus X-form
* [code:: frip] Floating Round to Integer Plus X-form
* [code:: frim.] Floating Round to Integer Minus X-form
* [code:: frim] Floating Round to Integer Minus X-form
* [code:: frsp.] Floating Round to Single-Precision X-form
* [code:: frsp] Floating Round to Single-Precision X-form
* [code:: fctiw.] Floating Convert with round Double-Precision To Signed Word format X-form
* [code:: fctiw] Floating Convert with round Double-Precision To Signed Word format X-form
* [code:: fctiwu.] Floating Convert with round Double-Precision To Unsigned Word format X-form
* [code:: fctiwu] Floating Convert with round Double-Precision To Unsigned Word format X-form
* [code:: fctid.] Floating Convert with round Double-Precision To Signed Doubleword format X-form
* [code:: fctid] Floating Convert with round Double-Precision To Signed Doubleword format X-form
* [code:: fcfid.] Floating Convert with round Signed Doubleword to Double-Precision format X-form
* [code:: fcfid] Floating Convert with round Signed Doubleword to Double-Precision format X-form
* [code:: fctidu.] Floating Convert with round Double-Precision To Unsigned Doubleword format X-form
* [code:: fctidu] Floating Convert with round Double-Precision To Unsigned Doubleword format X-form
* [code:: fcfidu.] Floating Convert with round Unsigned Doubleword to Double-Precision format X-form
* [code:: fcfidu] Floating Convert with round Unsigned Doubleword to Double-Precision format X-form
* [code:: fctiwz.] Floating Convert with truncate Double-Precision To Signed Word format X-form
* [code:: fctiwz] Floating Convert with truncate Double-Precision To Signed Word format X-form
* [code:: fctiwuz.] Floating Convert with truncate Double-Precision To Unsigned Word format X-form
* [code:: fctiwuz] Floating Convert with truncate Double-Precision To Unsigned Word format X-form
* [code:: fctidz.] Floating Convert with truncate Double-Precision To Signed Doubleword format X-form
* [code:: fctidz] Floating Convert with truncate Double-Precision To Signed Doubleword format X-form
* [code:: fctiduz.] Floating Convert with truncate Double-Precision To Unsigned Doubleword format X-form
* [code:: fctiduz] Floating Convert with truncate Double-Precision To Unsigned Doubleword format X-form
* [code:: fdiv.] Floating Divide A-form
* [code:: fdiv] Floating Divide A-form
* [code:: fsub.] Floating Subtract A-form
* [code:: fsub] Floating Subtract A-form
* [code:: fadd.] Floating Add A-form
* [code:: fadd] Floating Add A-form
* [code:: fsqrt.] Floating Square Root A-form
* [code:: fsqrt] Floating Square Root A-form
* [code:: fsel.] Floating Select A-form
* [code:: fsel] Floating Select A-form
* [code:: fre.] Floating Reciprocal Estimate A-form
* [code:: fre] Floating Reciprocal Estimate A-form
* [code:: fmul.] Floating Multiply A-form
* [code:: fmul] Floating Multiply A-form
* [code:: frsqrte.] Floating Reciprocal Square Root Estimate A-form
* [code:: frsqrte] Floating Reciprocal Square Root Estimate A-form
* [code:: fmsub.] Floating Multiply-Subtract A-form
* [code:: fmsub] Floating Multiply-Subtract A-form
* [code:: fmadd.] Floating Multiply-Add A-form
* [code:: fmadd] Floating Multiply-Add A-form
* [code:: fnmsub.] Floating Negative Multiply-Subtract A-form
* [code:: fnmsub] Floating Negative Multiply-Subtract A-form
* [code:: fnmadd.] Floating Negative Multiply-Add A-form
* [code:: fnmadd] Floating Negative Multiply-Add A-form
* [code:: pnop] Prefixed Nop MRR:*-form