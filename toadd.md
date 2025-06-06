
Load instructions

LD r8,r8
LD r8,n8
LD r16,n16
LD [HL],r8
LD [HL],n8
LD r8,[HL]
LD [r16],A
LD [n16],A
LDH [n16],A
LDH [C],A
LD A,[r16]
LD A,[n16]
LDH A,[n16]
LDH A,[C]
LD [HLI],A
LD [HLD],A
LD A,[HLI]
LD A,[HLD]

8-bit arithmetic instructions

ADC A,[HL]
ADC A,n8
ADD A,r8
ADD A,[HL]
ADD A,n8
CP A,r8
CP A,[HL]
CP A,n8
DEC r8
DEC [HL]
INC r8
INC [HL]
SBC A,r8
SBC A,[HL]
SBC A,n8
SUB A,r8
SUB A,[HL]
SUB A,n8

16-bit arithmetic instructions

ADD HL,r16
DEC r16
INC r16

Bitwise logic instructions

AND A,r8
AND A,[HL]
AND A,n8
CPL
OR A,r8
OR A,[HL]
OR A,n8
XOR A,r8
XOR A,[HL]
XOR A,n8

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

ADD HL,SP
ADD SP,e8
DEC SP
INC SP
LD SP,n16
LD [n16],SP
LD HL,SP+e8
LD SP,HL
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
NOP
STOP
