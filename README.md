# Bits to Compiler

This is the rough code implementation for a hopeful future series of articles documenting the progression of abstractions in software development and system programming basics. The idea is to document how one would write a simple bootloader using nothing but raw 1s and 0s (describing x86 ModR/M instruction encoding), progressing to a simple two pass assembler to make our lives easier with mnemonics, labels, and simple mathematics, and finally iterating on this design until we have a compiler for a toy c-like language.

The repo is currently a hodgepodge of rough tests and examples strewn throughout git history. While work still needs to be done to cleanup the codebase and write the accompanying articles, the code itself is more or less working up to standards.

# Bits

To simulate the error prone tediousness of altering bits manually with something such as a front panel, this chapter focuses on writing a simple program in Rust to reduce a text file of ASCII 1s and 0s into a raw binary that, when run on an x86 machine, will print the words "Hello, World!" to the terminal via BIOS commands.

This state of the project can be found at: https://github.com/cj-dimaggio/bits-to-compiler/tree/1d18cb065a0d56b6e0cf658155243b10c3a40c68
where we have:

* [An example of the assembly we'd like to replicate with out own Binary encoding](https://github.com/cj-dimaggio/bits-to-compiler/blob/1d18cb065a0d56b6e0cf658155243b10c3a40c68/examples/test.asm)

* [An example, in our binary encoding, of printing a simple "!" character to the screen.](https://github.com/cj-dimaggio/bits-to-compiler/blob/1d18cb065a0d56b6e0cf658155243b10c3a40c68/examples/example-bang.bit)

* [An example, in our binary encoding, of printing the string "Hello, World" to the screen](https://github.com/cj-dimaggio/bits-to-compiler/blob/1d18cb065a0d56b6e0cf658155243b10c3a40c68/examples/example-loop.bit)

* [The simple Rust program for compiling the ASCII binary encoding into raw binary](https://github.com/cj-dimaggio/bits-to-compiler/blob/1d18cb065a0d56b6e0cf658155243b10c3a40c68/compiler/src/main.rs)

The compiler can be built and run by `cd`ing into the `compiler` subdirectory and running:
```
$ cargo run ../examples/example-loop.bit
```

The build artifact can then be tested using QEMU using:

```
$ qemu-system-x86_64 -fda ../examples/example-loop.bin
```

(Debugging the produced binary can also be down by examining the disassembly: `bjdump -b binary -mi386 -Maddr16,data16 -D examples/example-loop.bin`)

# Assembler

The finished assembler code can be found at: https://github.com/cj-dimaggio/bits-to-compiler/tree/28ef57c5f0b0252520be87fbd1ff300ab56ead88

Here we have implemented a simple assembler capable of a handful of instructions (really only the ones we need for our toy example), some directives, label references, and simple arithmetic via a shunting yard implementation.

Our testing file is now: [`loop-2.bit`](https://github.com/cj-dimaggio/bits-to-compiler/blob/28ef57c5f0b0252520be87fbd1ff300ab56ead88/examples/loop-2.bit)

which can be compiled the same way from the `compiler` directory with:

```
$ cargo run ../examples/loop-2.bit
```

And then run in QEMU with:

```
$ qemu-system-x86_64 -fda ../examples/loop-2.bin
```

# Compiler

Our assembler is iterated on until we have a working compiler for a C-Like language, which is first completed at commit: https://github.com/cj-dimaggio/bits-to-compiler/tree/d507c64990780ac840afaf645cb8b9ae93cb263c

For code simplicity, our compiler currently doesn't emit actually binary anymore but instead it transpiles to assembly (whose inner workings we should now be well familiar with).

The testing file can be found at: https://github.com/cj-dimaggio/bits-to-compiler/blob/d507c64990780ac840afaf645cb8b9ae93cb263c/examples/c-like.bit


When compiled with `cargo run ../examples/c-like.bit` this outputs an assembly artifact: https://github.com/cj-dimaggio/bits-to-compiler/blob/d507c64990780ac840afaf645cb8b9ae93cb263c/examples/c-like.asm

Which, when compiled with NASM:
```
$ nasm -f bin examples/c-like.asm -o examples/c-like.bin
```

Can then be run with:

```
$ qemu-system-x86_64 -fda examples/c-like.bin
```
