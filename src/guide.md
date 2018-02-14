# Specs
4095 addresses in RAM indexed 0 to 4094. Every addressed is initialized to `0xDEADBEEF` and then overwritten if
they are a 'magic address'. Check the Memory Map to see which ones are overwritten.
16bit memory space, 8bit opcode, 12bit addresses. To the VM, the first 8bits are the opcode and the
rest are two 12bit addresses. If an opcode only takes 1 argument then the last 12bits are ignored.

# Magic numbers
- `0xDEADBEEF`: Uninitialized and unclaimed RAM, if this is set, programs and the VM will see the slot as unused
- `0xC001BABE`: Uninitialized, but **claimed** RAM, if this is set, programs and the VM will see this slot as used but not yet usable for checks and the like.

# Memory Map
- `0`: **EIP**, the address of the next instruction to be executed. Initialized to `SYSTEM_OFFSET`.
- `1`: **RET**, the return value of whatever was the last function run (`CMP`, etc). Init to `0xC001BABE`.
- `2`: **EFLAGS**, see EFLAGS section for more info.
- `3`: **STDOUT**, standard output, writes the utf8 char corresponding whatever code it contains to to the terminal or any stdout.
- `4`: **STDIN**, character code(s) of whatever chars have been entered into stdin.
- `5`: **gbuffer**, a single color code that it rendered progressively to the screen. mainly used for debug. but can be used for quickly software rendering a background (kinda janky right now)
- `6`: **spriteflags**, bitflags for which sprite is currently selected and the x and y coord to draw the sprite at.
- `0x0FFE`(4094): **EOF flag**, if this == 1 then program execution stops. The inst `EOF` sets this addr to 1
- `3095-4093`: Space reserved for constant values.

# Tokens
- `$`: Indicates a constant. The preprocessor automatically assigns the constant value to an address and replaces all references to the constant with the address.
- `0x`: Indicates a hexadecimal number. frequently used with `$`.

# Startup Process
1. RAM is created, every address is initialized to `0xDEADBEEF`
2. 'Magic addresses' such as EIP and RET are set to their default values. EIP is set to `SYSTEM_OFFSET`.
3. Program is mapped into RAM starting from `SYSTEM_OFFSET`.
4. Execution starts.

# EFLAGS
EFLAGS is a special address located at RAM[2] that is used to store bitflags for various things.
The following list tells what each bit of EFLAGS is used for. **list is numbered from lowest bit to greatest**
0. ZF (Zero Flag); Set when the result of an operation is zero.
4. OF (Overflow Flag); Set when an operation overflows.
8. SF (Sign Flag); Set when the result of an overflown operation **would be** negative.
12. IF (Input Flag); Set by a program, causes execution to stop until a user enters a value into stdin.
