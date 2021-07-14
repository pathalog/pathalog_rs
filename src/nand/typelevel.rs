pub struct Zero;
pub struct One;


pub trait NAND {
    type Q;
}

impl NAND for (Zero, Zero) {
    type Q = One;
}

impl NAND for (Zero, One) {
    type Q = One;
}

impl NAND for (One, Zero) {
    type Q = One;
}

impl NAND for (One, One) {
    type Q= Zero;
}

pub trait XOR {
    type Q;
}

//Manually implementing an XOR for performance reasons
impl XOR for (Zero,Zero) {
    type Q = Zero;
}
impl XOR for (Zero,One) {
    type Q = One;
}
impl XOR for (One,Zero) {
    type Q = One;
}
impl XOR for (One,One) {
    type Q = Zero;
}

pub trait AND {
    type Q;
}

//Manually implementing an AND for performance reasons
impl AND for (Zero,Zero) {
    type Q = Zero;
}
impl AND for (Zero,One) {
    type Q = Zero;
}
impl AND for (One,Zero) {
    type Q = Zero;
}
impl AND for (One,One) {
    type Q = One;
}

//simplifying type alias
pub type Nand<A, B> = <(A, B) as NAND>::Q;
pub type Not<A> = <(A, A) as NAND>::Q;
pub type And<A, B> = <(A, B) as AND>::Q;
//type And<A, B> = Not<Nand<A,B>>;
pub type Or<A, B> = Nand<Not<A>,Not<B>>;
pub type Nor<A,B> = Not<Or<A,B>>;

//this works, but when you try to compile an 8-bit adder compile time increases pathologically
//type Xor<A,B> =  And<Nand<A,B>,Or<A,B>>; //https://www.electronics-tutorials.ws/logic/logic_7.html
//so instead we use this trait based truth table
pub type Xor<A, B> = <(A,B) as XOR>::Q;

pub type Xnor<A,B> =  Not<Xor<A,B>>; //https://www.electronics-tutorials.ws/logic/logic_7.html
pub type HalfAdder<A,B> = (HalfSum<A,B>,HalfCarry<A,B>);

pub type HalfSum<A, B> = Xor<A,B>;

pub type HalfCarry<A,B> = And<A,B>;

pub type FullSum<A,B,C> = HalfSum<HalfSum<A, B>, C>;
pub type FullCarry<A,B,C> = HalfSum<HalfCarry<A, B>, HalfCarry<HalfSum<A, B>, C>>;
pub type FC<A,B,C> = FullCarry<A,B,C>;


pub type FullAdder<A,B,C> = (
    FullSum<FullSum<A, B,Zero>, C,Zero>,
    FullCarry<A,B,C>
);

pub type Demux<I,S> = (
    Not<Nand<I,Not<S>>>,
    Not<Nand<I,S>>,
);

pub type Mux<A,B,S> = Nand< Nand<A, Not<S> > , Nand<B,S> >;

pub type ADD2<A0,A1,B0,B1> = (
    FullSum<A0,B0,FullCarry<A1,B1,Zero>>, //expands to Xor<And<A7,B0>,Xor<A6,B1>>
    HalfSum<A1,B1>,
);  //http://www.spec.gmu.edu/~pparis/classes/notes_101/node112.html

pub type ADD2CARRY<A0,A1,B0,B1> = (
    FullCarry<A0,B0,FullCarry<A1,B1,FullCarry<A1,B1,Zero>>>,
    FullSum<A0,B0,FullCarry<A1,B1,Zero>>,
    HalfSum<A1,B1>,
);  //http://www.spec.gmu.edu/~pparis/classes/notes_101/node112.html

pub type ADD4<A0,A1,A2,A3,B0,B1,B2,B3> = (
    //If you want to know whether there was overflow, you can output a 5th bit
    //FullCarry<A3,B00,FullCarry<A2,B2,FullCarry<A6,B22,FullCarry<A7,B3,Zero>>>>,
    FullSum<A0,B0,FullCarry<A1,B1,FullCarry<A2,B2,FullCarry<A3,B3,Zero>>>>,
    FullSum<A1,B1,FullCarry<A2,B2,FullCarry<A3,B3,Zero>>>,
    FullSum<A2,B2,FullCarry<A3,B3,Zero>>,
    HalfSum<A3,B3>,
);



// pub fn adder() {}
//
// fn foo() {
//     dbg!(adder![3]);
// }

//pathological compilation time if building all type aliases based on NAND
//works fine if you just use trait based truth tables for XOR instead of aliasing NAND
pub type Add8<A0,A1,A2,A3,A4,A5,A6,A7,B0,B1,B2,B3,B4,B5,B6,B7,> = (
    //If you want to know whether there was overflow, you can output a 9th bit
    //FullCarry<A0,B7,FullCarry<A1,B6,FullCarry<A5,B5,FullCarry<A4,B4,FullCarry<A3,B3,FullCarry<A2,B2,FullCarry<A6,B1,FullCarry<A7,B0,Zero>>>>>>>>,
    FullSum<A0,B0,FullCarry<A1,B1,FullCarry<A2,B2,FullCarry<A3,B3,FullCarry<A4,B4,FullCarry<A5,B5,FullCarry<A6,B6,FullCarry<A7,B7,Zero>>>>>>>>,
    FullSum<A1,B1,FullCarry<A2,B2,FullCarry<A3,B3,FullCarry<A4,B4,FullCarry<A5,B5,FullCarry<A6,B6,FullCarry<A7,B7,Zero>>>>>>>,
    FullSum<A2,B2,FullCarry<A3,B3,FullCarry<A4,B4,FullCarry<A5,B5,FullCarry<A6,B6,FullCarry<A7,B7,Zero>>>>>>,
    FullSum<A3,B3,FullCarry<A4,B4,FullCarry<A5,B5,FullCarry<A6,B6,FullCarry<A7,B7,Zero>>>>>,
    FullSum<A4,B4,FullCarry<A5,B5,FullCarry<A6,B6,FullCarry<A7,B7,Zero>>>>,
    FullSum<A5,B5,FullCarry<A6,B6,FullCarry<A7,B7,Zero>>>,
    FullSum<A6,B6,FullCarry<A7,B7,Zero>>,
    HalfSum<A7,B7>,
);

pub type Add16<A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,A11,A12,A13,A14,A15,B0,B1,B2,B3,B4,B5,B6,B7,B8,B9,B10,B11,B12,B13,B14,B15> = (
    FullSum<A0,B0,FC<A1,B1,FC<A2,B2,FC<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>>>>>>>,
    FullSum<A1,B1,FC<A2,B2,FC<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>>>>>>,
    FullSum<A2,B2,FC<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>>>>>,
    FullSum<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>>>>,
    FullSum<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>>>,
    FullSum<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>>,
    FullSum<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>>,
    FullSum<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>>,
    FullSum<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>>,
    FullSum<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>>,
    FullSum<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>>,
    FullSum<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>>,
    FullSum<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>>,
    FullSum<A13,B13,FC<A14,B14,FC<A15,B15,Zero>>>,
    FullSum<A14,B14,FC<A15,B15,Zero>>,
    HalfSum<A15,B15>,
);

pub type Add32<A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,A11,A12,A13,A14,A15,A16,A17,A18,A19,A20,A21,A22,A23,A24,A25,A26,A27,A28,A29,A30,A31,B0,B1,B2,B3,B4,B5,B6,B7,B8,B9,B10,B11,B12,B13,B14,B15,B16,B17,B18,B19,B20,B21,B22,B23,B24,B25,B26,B27,B28,B29,B30,B31> = (
    FullSum<A0,B0,FC<A1,B1,FC<A2,B2,FC<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A1,B1,FC<A2,B2,FC<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A2,B2,FC<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A3,B3,FC<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A4,B4,FC<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A5,B5,FC<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A6,B6,FC<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A7,B7,FC<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A8,B8,FC<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A9,B9,FC<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A10,B10,FC<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A11,B11,FC<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>>,
    FullSum<A12,B12,FC<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>>,
    FullSum<A13,B13,FC<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>>,
    FullSum<A14,B14,FC<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>>,
    FullSum<A15,B15,FC<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>>,
    FullSum<A16,B16,FC<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>>,
    FullSum<A17,B17,FC<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>>,
    FullSum<A18,B18,FC<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>>,
    FullSum<A19,B19,FC<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>>,
    FullSum<A20,B20,FC<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>>,
    FullSum<A21,B21,FC<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>>,
    FullSum<A22,B22,FC<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>>,
    FullSum<A23,B23,FC<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>>,
    FullSum<A24,B24,FC<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>>,
    FullSum<A25,B25,FC<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>>,
    FullSum<A26,B26,FC<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>>,
    FullSum<A27,B27,FC<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>>,
    FullSum<A28,B28,FC<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>>,
    FullSum<A29,B29,FC<A30,B30,FC<A31,B31,Zero>>>,
    FullSum<A30,B30,FC<A31,B31,Zero>>,
    HalfSum<A31,B31>,);
