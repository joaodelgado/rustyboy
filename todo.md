# Work Work

## general

  - Power up sequence
    - Beep sounds
    - Nintendo logo scroll
    - Developer validation

## cpu

  - read opcodes
  - Implement instructions
    - LD nn,
    - LD r1,r2
    - LD A,n
    - LD n,A
    - LD A,(C)
    - LD A,(HLD)
    - LD A,(HL-)
    - LDD A,(HL)
    - LD (HLD),A
    - LD(HL-1),A
    - LDD (HL),A
    - LD A,(HLI)
    - LD A,(HL+)
    - LD A,(HL)
    - LD (HLI),A
    - LD (HL+),A
    - LDI (HL),A
    - LDH (n),A
    - LDH A, (n)
    - LD n,nn
    - ~~LD SP,HL~~
    - LD HL, SP+n
    - LDHL SP,n
    - ~~LD (nn),SP~~
    - PUSH nn
    - POP nn
    - ADD A,n
    - ADC A,n
    - SUB n
    - SBC A,n
    - AND n
    - OR n
    - XOR n
    - CP n
    - INC n
    - DEC n
    - ADD HL,n
    - ADD SP,n
    - INC nn
    - DEC nn
    - SWAP n
    - DAA
    - CPL
    - CCF
    - SCF
    - ~~NOP~~
    - STOP
    - ~~DI~~
    - EI
    - RLCA
    - RLA
    - RRCA
    - RRA
    - RLC n
    - RL n
    - RRC n
    - RR n
    - SLA n
    - SRA n
    - SRL n
    - BIT b,r
    - SET b,r
    - RES b,r
    - JP nn
    - JP (HL)
    - JR cc, n
    - ~~JP nn~~
    - ~~JP cc,nn~~
    - JP (HL)
    - JR cc,n
    - CALL nn
    - CALL cc,nn
    - RST n
    - RET
    - RET cc
    - RETI

## Utilities

- Macro to simplify print and return on all instruction implementations
- inline functions/macros to convert two u8s in u16 and vice versa
