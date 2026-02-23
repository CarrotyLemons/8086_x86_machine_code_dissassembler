# Overview
This is a partially implemented 8086 dissassembler. That turns 8086 binary machine code back into the corresponding assembly instructions.

When compiled (using `nasm` in the tests) this will result in a equivalent binary.

It is currently only implemented for the following
- A subset of `mov` instructions
- `add`
- `sub`
- `cmp`