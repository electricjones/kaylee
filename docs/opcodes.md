| Operation   | Opcode   | Op 0   | Op 1   | Op 2   | Notes                                   |
|-------------|----------|--------|--------|--------|-----------------------------------------|
| HALT        | 0x06     | -      | -      | -      | This is some cool note about my thing   |
| LOAD        | 0x10     | a-     | -a-    | -a     | This is some cool note about my thing   |
| STORE       | 0x1121   | a      | b-     | -b     |                                         |
| STORE       | 0xEF32   | a      | b      | c      |                                         |
| ----------- | -------- | ------ | ------ | ------ | --------------------------------------- |

# Identifiers

`$` = Register
`#` = Constant
`@` = Program Point
`&` = Memory Address
`> <` = Memory Boundary

# Machine

0 - 29 NOP HALT DIE ERR THREAD x5 STATS SLEEP DEBUG x 5

# Data

30 - 49 LOAD MOVE COPY WRITE READ

# Program

50 - 69 JUMP JUMPF JUMPB JUMPC

# Math

70 - 99 ADD SUB MUL DIV POW SQR CAST x 3 MOD ROUND

# Logical / Compare

100 - 129

# Library / System

129 - 199 GRAPHICS CONSOLE FILES USERS PERMISSIONS TYPES

# Misc / Reserved

200 +
