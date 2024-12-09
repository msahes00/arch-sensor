# PDFs Summary

> **Note**: The obsidian dataview plugin is required for viewing the summary.
## General
```dataview
TABLE 
	join(endianness, ", ")       AS "Endianness", 
	join(instruction.bits, ", ") AS "Bits", 
	instruction.len              AS "Instruction length",
	length(file.lists)           AS "Instruction count"
FROM #assembly
```

## Instructions
```dataviewjs
// Get all pages with the #assembly tag
let pages = dv.pages('#assembly');
  

// Iterate over all mnemonics of each file
let mnemonics = {};
pages.forEach(file => {
	let instructions = file["code"] || [];
	// Use a Set to track mnemonics seen on this page
	let seenMnemonics = new Set();
	
	// For each mnemonic found, increase its counter
	instructions.forEach(mnemonic => {
		// Skip if already counted on this page
		if (seenMnemonics.has(mnemonic)) return;

		seenMnemonics.add(mnemonic);
		
		let count = mnemonics[mnemonic] || 0;
		mnemonics[mnemonic] = count + 1;
	});
});

// Create an array of mnemonics with their counts and common status
let mnemonicDetails = Object.keys(mnemonics).map(mnemonic => ({
	mnemonic: mnemonic,
	count: mnemonics[mnemonic],
	common: mnemonics[mnemonic] === pages.length
}));
  
// Display the mnemonics in a table with count and common status
let tableData = mnemonicDetails
	.map(item => [item.mnemonic, item.count, item.common])
	.sort((a, b) => b[1] - a[1])

dv.table(['Mnemonic', 'Count', 'Common'], tableData);
```
