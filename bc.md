# Backpack Bytecode

The Backpack Bytecode is a different bytecode, not like JVM Bytecode Format or WASM
alls of opcodes are 5 bit
## List

NOP
NOP Opcode Format: 00000
NOP Don't do anything, just plus 1 to IP (Instruction Pointer)
Push
SetLen
SetLen Opcode Format: 10100 + 5 Bit Operand
SetLen what's do? SetLen Set The Length of all Operand of Instructions
Push
Push Opcode Format: 00001 + Length From SetLen Operand
Push work is pushed bit or bits to stack
POP
POP Opcore Format: 00010
POP What's do? POP, Delete The Last Bit of Stack
Plus
Plus Opcode Format: 00011
Plus what's do? Plus, Plus 2 Bit of Last Stack Bit or Stack Home
Minus
Minus Opcode Format: 00100
Minus what's do? Minus, Minu 2 Bit of Last Stack Bit or Stack Home
Swap
Swap Opcode Format: 00101
Swap what's do? Swap, Swap The 2 Last Bit or Home of Stack
Copy
Copy Opcode Format: 00110
Copy what's do? Copy, Duplicate The Last Home of Stack and pushed to stack
Compare
Compare Format: 00111
Compare what do? compare, Comapre the 2 Last Bit of Stack and Save The Result with 2 bit: 00 mean 2 of are Stack Home Input are Equal, with them,
01 Mean The Last Stack Home [-1] are smaller than [-2] of Stack or:
a < b
and 10 mean a is Bigger Than b:
a > b