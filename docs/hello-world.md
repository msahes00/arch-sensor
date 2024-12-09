# Hello World Analysis

## Creating the program

The program to analyze is going to be the following hello world:
```c
#include <stdio.h>
#include <string.h>

// Print "Hello, World!" to the console
void print_hello() {
    printf("Hello, World!\n");
}

// Function to open a file, write a message to it, read from it, and close it
void file_operations() {
    const char *filename = "hello.txt";
    const char *content = "Hello from file!\n";
    char buffer[64];

    // Open the file for writing ("w" mode creates or overwrites the file)
    FILE *file = fopen(filename, "w");
    if (file == NULL) {
        perror("Error opening file for writing");
        return;
    }

    // Write content to file
    fwrite(content, sizeof(char), strlen(content), file);

    // Close the file after writing
    fclose(file);

    // Re-open the file for reading ("r" mode)
    file = fopen(filename, "r");
    if (file == NULL) {
        perror("Error opening file for reading");
        return;
    }

    // Read content from file into buffer and print it to stdout
    size_t n = fread(buffer, sizeof(char), sizeof(buffer) - 1, file);
    if (n > 0) {
        buffer[n] = '\0';  // Null-terminate the buffer
        printf("Read from file: %s", buffer);
    }

    // Close the file after reading
    fclose(file);
}

int main() {
    print_hello();      // Print "Hello, World!" to console
    file_operations();  // Perform basic file operations

    return 0;
}
```

## Compiling into assembly

To compile this hello world, this command is used for linux:
```bash
gcc -S -O0 hello.c -o hello_assembly.s
```
which makes use of the following flags:
- `-S`: Tells gcc to output assembly rather than produce an executable binary
- `-O0`: Tells gcc to disable any optimizations

Running the following command on an X86_64 Ubuntu system yields the following output:
```assembly
	.file	"main.c"
	.text
	.section	.rodata
.LC0:
	.string	"Hello, World!"
	.text
	.globl	print_hello
	.type	print_hello, @function
print_hello:
.LFB0:
	.cfi_startproc
	endbr64
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	leaq	.LC0(%rip), %rax
	movq	%rax, %rdi
	call	puts@PLT
	nop
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	print_hello, .-print_hello
	.section	.rodata
.LC1:
	.string	"hello.txt"
.LC2:
	.string	"Hello from file!\n"
.LC3:
	.string	"w"
	.align 8
.LC4:
	.string	"Error opening file for writing"
.LC5:
	.string	"r"
	.align 8
.LC6:
	.string	"Error opening file for reading"
.LC7:
	.string	"Read from file: %s"
	.text
	.globl	file_operations
	.type	file_operations, @function
file_operations:
.LFB1:
	.cfi_startproc
	endbr64
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$112, %rsp
	movq	%fs:40, %rax
	movq	%rax, -8(%rbp)
	xorl	%eax, %eax
	leaq	.LC1(%rip), %rax
	movq	%rax, -112(%rbp)
	leaq	.LC2(%rip), %rax
	movq	%rax, -104(%rbp)
	movq	-112(%rbp), %rax
	leaq	.LC3(%rip), %rdx
	movq	%rdx, %rsi
	movq	%rax, %rdi
	call	fopen@PLT
	movq	%rax, -96(%rbp)
	cmpq	$0, -96(%rbp)
	jne	.L3
	leaq	.LC4(%rip), %rax
	movq	%rax, %rdi
	call	perror@PLT
	jmp	.L2
.L3:
	movq	-104(%rbp), %rax
	movq	%rax, %rdi
	call	strlen@PLT
	movq	%rax, %rsi
	movq	-96(%rbp), %rdx
	movq	-104(%rbp), %rax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movl	$1, %esi
	movq	%rax, %rdi
	call	fwrite@PLT
	movq	-96(%rbp), %rax
	movq	%rax, %rdi
	call	fclose@PLT
	movq	-112(%rbp), %rax
	leaq	.LC5(%rip), %rdx
	movq	%rdx, %rsi
	movq	%rax, %rdi
	call	fopen@PLT
	movq	%rax, -96(%rbp)
	cmpq	$0, -96(%rbp)
	jne	.L5
	leaq	.LC6(%rip), %rax
	movq	%rax, %rdi
	call	perror@PLT
	jmp	.L2
.L5:
	movq	-96(%rbp), %rdx
	leaq	-80(%rbp), %rax
	movq	%rdx, %rcx
	movl	$63, %edx
	movl	$1, %esi
	movq	%rax, %rdi
	call	fread@PLT
	movq	%rax, -88(%rbp)
	cmpq	$0, -88(%rbp)
	je	.L6
	leaq	-80(%rbp), %rdx
	movq	-88(%rbp), %rax
	addq	%rdx, %rax
	movb	$0, (%rax)
	leaq	-80(%rbp), %rax
	movq	%rax, %rsi
	leaq	.LC7(%rip), %rax
	movq	%rax, %rdi
	movl	$0, %eax
	call	printf@PLT
.L6:
	movq	-96(%rbp), %rax
	movq	%rax, %rdi
	call	fclose@PLT
.L2:
	movq	-8(%rbp), %rax
	subq	%fs:40, %rax
	je	.L8
	call	__stack_chk_fail@PLT
.L8:
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.size	file_operations, .-file_operations
	.globl	main
	.type	main, @function
main:
.LFB2:
	.cfi_startproc
	endbr64
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$0, %eax
	call	print_hello
	movl	$0, %eax
	call	file_operations
	movl	$0, %eax
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE2:
	.size	main, .-main
	.ident	"GCC: (Ubuntu 13.2.0-23ubuntu4) 13.2.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	1f - 0f
	.long	4f - 1f
	.long	5
0:
	.string	"GNU"
1:
	.align 8
	.long	0xc0000002
	.long	3f - 2f
2:
	.long	0x3
3:
	.align 8
4:
```

## Completing the analysis

After stripping globals and labels, the following instructions list is left:
```assembly
	endbr64
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	.LC0(%rip), %rax
	movq	%rax, %rdi
	call	puts@PLT
	nop
	popq	%rbp
	ret

	endbr64
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$112, %rsp
	movq	%fs:40, %rax
	movq	%rax, -8(%rbp)
	xorl	%eax, %eax
	leaq	.LC1(%rip), %rax
	movq	%rax, -112(%rbp)
	leaq	.LC2(%rip), %rax
	movq	%rax, -104(%rbp)
	movq	-112(%rbp), %rax
	leaq	.LC3(%rip), %rdx
	movq	%rdx, %rsi
	movq	%rax, %rdi
	call	fopen@PLT
	movq	%rax, -96(%rbp)
	cmpq	$0, -96(%rbp)
	jne	.L3
	leaq	.LC4(%rip), %rax
	movq	%rax, %rdi
	call	perror@PLT
	jmp	.L2

	movq	-104(%rbp), %rax
	movq	%rax, %rdi
	call	strlen@PLT
	movq	%rax, %rsi
	movq	-96(%rbp), %rdx
	movq	-104(%rbp), %rax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movl	$1, %esi
	movq	%rax, %rdi
	call	fwrite@PLT
	movq	-96(%rbp), %rax
	movq	%rax, %rdi
	call	fclose@PLT
	movq	-112(%rbp), %rax
	leaq	.LC5(%rip), %rdx
	movq	%rdx, %rsi
	movq	%rax, %rdi
	call	fopen@PLT
	movq	%rax, -96(%rbp)
	cmpq	$0, -96(%rbp)
	jne	.L5
	leaq	.LC6(%rip), %rax
	movq	%rax, %rdi
	call	perror@PLT
	jmp	.L2

	movq	-96(%rbp), %rdx
	leaq	-80(%rbp), %rax
	movq	%rdx, %rcx
	movl	$63, %edx
	movl	$1, %esi
	movq	%rax, %rdi
	call	fread@PLT
	movq	%rax, -88(%rbp)
	cmpq	$0, -88(%rbp)
	je	.L6
	leaq	-80(%rbp), %rdx
	movq	-88(%rbp), %rax
	addq	%rdx, %rax
	movb	$0, (%rax)
	leaq	-80(%rbp), %rax
	movq	%rax, %rsi
	leaq	.LC7(%rip), %rax
	movq	%rax, %rdi
	movl	$0, %eax
	call	printf@PLT

	movq	-96(%rbp), %rax
	movq	%rax, %rdi
	call	fclose@PLT

	movq	-8(%rbp), %rax
	subq	%fs:40, %rax
	je	.L8
	call	__stack_chk_fail@PLT

	leave
	ret

	endbr64
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$0, %eax
	call	print_hello
	movl	$0, %eax
	call	file_operations
	movl	$0, %eax
	popq	%rbp
	ret
```

Finally, after removing all duplicated mnemonics, the following mnemonics are left:
* addq
* call
* cmpq
* endbr64
* je
* jmp
* jne
* leaq
* leave
* movb
* movl
* movq
* nop
* popq
* pushq
* ret
* subq
* xorl