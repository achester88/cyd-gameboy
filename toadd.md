8-bit arithmetic instructions

DEC r8
DEC [HL]
INC r8
INC [HL]

16-bit arithmetic instructions

DEC r16
INC r16

Bitwise logic instructions

CPL

Bit flag instructions

BIT u3,r8
BIT u3,[HL]
RES u3,r8
RES u3,[HL]
SET u3,r8
SET u3,[HL]

Bit shift instructions

RL r8
RL [HL]
RLA
RLC r8
RLC [HL]
RLCA
RR r8
RR [HL]
RRA
RRC r8
RRC [HL]
RRCA
SLA r8
SLA [HL]
SRA r8
SRA [HL]
SRL r8
SRL [HL]
SWAP r8
SWAP [HL]

Jumps and subroutine instructions

CALL n16
CALL cc,n16
JP HL
JP n16
JP cc,n16
JR n16
JR cc,n16
RET cc
RET
RETI
RST vec

Carry flag instructions

CCF
SCF

Stack manipulation instructions

DEC SP
INC SP
POP AF
POP r16
PUSH AF
PUSH r16

Interrupt-related instructions

DI
EI
HALT

Miscellaneous instructions

DAA
STOP
