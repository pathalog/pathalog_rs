//Based on
//http://www.simplecpudesign.com/simple_cpu_v1/index.html

use crate::nand::runtime::Bit::Zero;
use crate::nand::runtime::{Add, Binary, Bit};

#[derive(Default)]
struct Pair(Bit,Bit);


#[derive(Default)]
struct Nibble(Pair,Pair);

#[derive(Default)]
struct Byte(Nibble, Nibble);

///Program Counter
struct ProgramCounter(Byte);

///Instruction Counter
struct InstructionRegister(Byte, Byte);

///Accumulator
struct Accumulator(Byte);

///Using a Byte for addressing allows us to support up to 256 bytes of storage
struct Addr(Byte);
struct Value(Byte);

#[derive(Default)]
struct TwoByte(Byte, Byte);
#[derive(Default)]
struct FourByte(TwoByte, TwoByte);
#[derive(Default)]
struct EightByte(FourByte, FourByte);
#[derive(Default)]
struct SixteenByte(EightByte, EightByte);
#[derive(Default)]
struct ThirtyTwoByte(SixteenByte, SixteenByte);
#[derive(Default)]
struct SixtyFourByte(ThirtyTwoByte, ThirtyTwoByte);
#[derive(Default)]
struct OneTwentyEightByte(SixtyFourByte,SixtyFourByte);
#[derive(Default)]
struct TwoFiftySixByte(OneTwentyEightByte,OneTwentyEightByte);
#[derive(Default)]
struct FiveOneTwoByte(TwoFiftySixByte,TwoFiftySixByte);
#[derive(Default)]
struct KiloByte(FiveOneTwoByte,FiveOneTwoByte);
#[derive(Default)]
struct TwoKB(KiloByte,KiloByte);
#[derive(Default)]
struct FourKB(TwoKB,TwoKB);
#[derive(Default)]
struct EightKB(FourKB,FourKB);
#[derive(Default)]
struct SixteenKB(EightKB,EightKB);
#[derive(Default)]
struct ThirtyTwoKB(SixteenKB,SixteenKB);
#[derive(Default)]
struct SixtyFourKB(ThirtyTwoKB,ThirtyTwoKB);

struct ROM();

type InterpreterBytes = FiveOneTwoByte;

struct ProgramBytes(
    KiloByte
);
//4096 Bytes of RAM
struct RAM(InterpreterBytes);

// enum Instruction {
//     ///no operation
//     NOP,
//
//     ///loads 8 bit value stored at given address to accumulator
//     LDA(Addr),
//
//     ///adds the value stored at given address to accumulator and stores the result in accumulator
//     ADD(Addr),
//
//     ///subtracts the value stored at given address from accumulator and stores the result in accumulator
//     SUB(Addr),
//
//     ///stores value in accumulater at the address provided in memory
//     STA(Addr),
//
//     ///loads an immidiate value to accumulator
//     LDI(Value),
//
//     ///unconditional jump to specified address
//     JMP(Addr),
//
//     ///jumps to provided address if carry flag is set
//     JC(Addr),
//
//     ///jumps to provided address if zero flag is set
//     JZ(Addr),
//
//     ///moves contents of accumulator to register B
//     MOVAB,
//
//     ///moves contents of register B to accumulator
//     MOVBA,
//
//     ///displays value stored in accumlator on the display
//     OUT,
//
//     ///haults the system by suspending clock
//     HLT,
// }

struct Computer {
    rom: ROM,
    ram: RAM,
    processor: Processor,
}

struct Processor {
    pc: ProgramCounter,
    ir: InstructionRegister,
    id: InstructionDecoder,
    gpr: GeneralPurposeRegisters,
    alu: ArithemticLogicUnit,
    acc: ControlUnit,
}

struct ControlUnit();

struct GeneralPurposeRegisters();

struct InstructionDecoder();

impl Processor {
    fn mov_a_b(&mut self) {}
    fn add_c(&mut self) {}
}

struct ArithemticLogicUnit();

// trait ALU {
//     fn adder(a:Byte,b:Byte) -> (Byte,Byte) {
//
//     }
// }
