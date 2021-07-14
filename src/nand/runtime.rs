#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Bit {
    Zero,
    One
}
use Bit::*;


pub trait NAnd where Self:Sized+Copy {
    //only the primary nand() function is abstract. the rest are implemented in terms of nand()
    //therefore something that is nandlike just needs to implement a truth table in
    // a single function in order to take advantage of any nand-like behavior
    fn nand(self, other:Self) -> Self;

    //An implementer gets these for free
    fn not(self) -> Self {
        self.nand(self)
    }
    fn and(self, other: Self) -> Self {
        self.nand(other).not()
    }
    fn or(self, other: Self) -> Self {
        self.not().nand(other.not())
    }
    fn xnor(self, other: Self) -> Self { self.xor(other).not() }
    fn xor(self, other: Self) -> Self { self.nand(other).and(self.or(other)) }
}


impl NAnd for Bit {
    fn nand(self, other: Self) -> Self {
        match (self, other) {
            (Zero, Zero) => One,
            (Zero, One) => One,
            (One, Zero) => One,
            (One, One) => Zero,
        }
    }
}

pub trait Binary {
    fn full_sum(self, c: Bit) -> Bit;
    fn full_add(self, c: Bit) -> (Bit,Bit);
    fn half_adder(self) -> (Bit,Bit) ;
    fn half_carry(self) -> Bit;
    fn half_sum(self) -> Bit;
    fn fc(self, c:Bit) -> Bit;
}

impl Binary for (Bit,Bit) {
    fn full_sum(self, c: Bit) -> Bit {
        (self.half_sum(), c).half_sum()
    }

    fn full_add(self, c: Bit) -> (Bit, Bit) {
        (
            (self.full_sum(Zero), c).full_sum(Zero),
            self.fc(c)
        )
    }

    fn half_adder(self) -> (Bit, Bit) {
        (
            self.half_sum(),
            self.half_carry()
        )
    }
    fn half_carry(self) -> Bit {
        self.0.and(self.1)
    }
    fn half_sum(self) -> Bit {
        self.0.xor(self.1)
    }
    //this is a full carry. using shorthand as it's used so often in adders
    fn fc(self, c: Bit) -> Bit {
        (
            self.half_carry(),
            (self.half_sum(), c).half_carry()
        ).half_sum()
            }
}

pub trait Add where Self:Sized {
    fn add(self,other:Self) -> Self;

}

//2 bit adder
impl Add for (Bit,Bit) {
    fn add(self,other:Self) -> Self {(
        (self.0,other.0).full_sum((self.1,other.1).fc(Zero)),
        (self.1,other.1).half_sum(),
        // FullSum<A0,B0,FullCarry<A1,B1,Zero>>, //expands to Xor<And<A7,B0>,Xor<A6,B1>>
        // HalfSum<A1,B1>,
    )}
}

//4 bit adder
impl Add for (Bit,Bit,Bit,Bit) {
    fn add(self,other:Self) -> Self {(
        (self.0,other.0).full_sum((self.1,other.1).fc((self.2, other.2).fc((self.3, other.3).fc(Zero)))),
        (self.1,other.1).full_sum((self.2,other.2).fc((self.3, other.3).fc(Zero))),
        (self.2,other.2).full_sum((self.3,other.3).fc(Zero)),
        (self.3,other.3).half_sum(),

    )}
}

//8 bit adder
impl Add for (Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit) {
    fn add(self,b:Self) -> Self {
        let a = self;
        (
        (a.0,b.0).full_sum((a.1,b.1).fc((a.2, b.2).fc((a.3, b.3).fc((a.4, b.4).fc((a.5, b.5).fc((a.6, b.6).fc((a.6, b.6).fc(Zero)))))))),
        (a.1,b.1).full_sum((a.2,b.2).fc((a.3, b.3).fc((a.4, b.4).fc((a.5, b.5).fc((a.6, b.6).fc((a.6, b.6).fc(Zero))))))),
        (a.2,b.2).full_sum((a.3,b.3).fc((a.4, b.4).fc((a.5, b.5).fc((a.6, b.6).fc((a.6, b.6).fc(Zero)))))),
        (a.3,b.3).full_sum((a.4,b.4).fc((a.5, b.5).fc((a.6, b.6).fc((a.6, b.6).fc(Zero))))),
        (a.4,b.4).full_sum((a.5,b.5).fc((a.6, b.6).fc((a.6, b.6).fc(Zero)))),
        (a.5,b.5).full_sum((a.6,b.6).fc((a.7, b.7).fc(Zero))),
        (a.6,b.6).full_sum((a.7,b.7).fc(Zero)),
        (a.7,b.7).half_sum(),
    )}
}

//16 bit adder
impl Add for ((Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit),
              (Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit)) {
    fn add(self, other: Self) -> Self {
        let (a1, a2, b1, b2) = (self.0, self.1, other.0, other.1);
        (
            (
                (a1.0, b1.0).full_sum((a1.1, b1.1).fc((a1.2, b1.2).fc((a1.3, b1.3).fc((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))),
                (a1.1, b1.1).full_sum((a1.2, b1.2).fc((a1.3, b1.3).fc((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))),
                (a1.2, b1.2).full_sum((a1.3, b1.3).fc((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))),
                (a1.3, b1.3).full_sum((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))),
                (a1.4, b1.4).full_sum((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))),
                (a1.5, b1.5).full_sum((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))),
                (a1.6, b1.6).full_sum((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))),
                (a1.7, b1.7).full_sum((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))
            ),
            (
                (a2.0, b2.0).full_sum((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))),
                (a2.1, b2.1).full_sum((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))),
                (a2.2, b2.2).full_sum((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))),
                (a2.3, b2.3).full_sum((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))),
                (a2.4, b2.4).full_sum((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))),
                (a2.5, b2.5).full_sum((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))),
                (a2.6, b2.6).full_sum((a2.7, b2.7).fc(Zero)),
                (a2.7, b2.7).half_sum(),
            )
        )
    }
}
    //32 bit adder
    impl Add for ((Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit),
                  (Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit),
                  (Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit),
                  (Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit)) {
        fn add(self, other: Self) -> Self {
            let (a1, a2, a3,a4) = self;
            let (b1, b2,b3,b4) = other;
            (
                (
                    (a1.0, b1.0).full_sum((a1.1, b1.1).fc((a1.2, b1.2).fc( (a1.3, b1.3).fc((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))))))))))))))),
                    (a1.1, b1.1).full_sum((a1.2, b1.2).fc( (a1.3, b1.3).fc((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))))))))))))))),
                    (a1.2, b1.2).full_sum( (a1.3, b1.3).fc((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))))))))))))),
                    (a1.3, b1.3).full_sum((a1.4, b1.4).fc((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))))))))))))),
                    (a1.4, b1.4).full_sum((a1.5, b1.5).fc((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))))))))))),
                    (a1.5, b1.5).full_sum((a1.6, b1.6).fc((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))))))))))),
                    (a1.6, b1.6).full_sum((a1.7, b1.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))))))))),
                    (a1.7, b1.7).full_sum((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))))))))),
                ),
                (
                    (a2.0, b2.0).full_sum((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))))))),
                    (a2.1, b2.1).full_sum((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))))))),
                    (a2.2, b2.2).full_sum((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))))),
                    (a2.3, b2.3).full_sum((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))))),
                    (a2.4, b2.4).full_sum((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))))),
                    (a2.5, b2.5).full_sum((a2.6, b2.6).fc((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))))),
                    (a2.6, b2.6).full_sum((a2.7, b2.7).fc((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))))),
                    (a2.7, b2.7).full_sum((a3.0, b3.0).fc((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))))),
                ),
                (
                    (a3.0, b3.0).full_sum((a3.1, b3.1).fc((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))))),
                    (a3.1, b3.1).full_sum((a3.2, b3.2).fc((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))))),
                    (a3.2, b3.2).full_sum((a3.3, b3.3).fc((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))))),
                    (a3.3, b3.3).full_sum((a3.4, b3.4).fc((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))))),
                    (a3.4, b3.4).full_sum((a3.5, b3.5).fc((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))))),
                    (a3.5, b3.5).full_sum((a3.6, b3.6).fc((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero))))))))))),
                    (a3.6, b3.6).full_sum((a3.7, b3.7).fc((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))),
                    (a3.7, b3.7).full_sum((a2.0, b2.0).fc((a2.1, b2.1).fc((a2.2, b2.2).fc((a2.3, b2.3).fc((a2.4, b2.4).fc((a2.5, b2.5).fc((a2.6, b2.6).fc((a2.7, b2.7).fc(Zero)))))))))
                ),
                (
                    (a4.0, b4.0).full_sum((a4.1, b4.1).fc((a4.2, b4.2).fc((a4.3, b4.3).fc((a4.4, b4.4).fc((a4.5, b4.5).fc((a4.6, b4.6).fc((a4.7, b4.7).fc(Zero)))))))),
                    (a4.1, b4.1).full_sum((a4.2, b4.2).fc((a4.3, b4.3).fc((a4.4, b4.4).fc((a4.5, b4.5).fc((a4.6, b4.6).fc((a4.7, b4.7).fc(Zero))))))),
                    (a4.2, b4.2).full_sum((a4.3, b4.3).fc((a4.4, b4.4).fc((a4.5, b4.5).fc((a4.6, b4.6).fc((a4.7, b4.7).fc(Zero)))))),
                    (a4.3, b4.3).full_sum((a4.4, b4.4).fc((a4.5, b4.5).fc((a4.6, b4.6).fc((a4.7, b4.7).fc(Zero))))),
                    (a4.4, b4.4).full_sum((a4.5, b4.5).fc((a4.6, b4.6).fc((a4.7, b4.7).fc(Zero)))),
                    (a4.5, b4.5).full_sum((a4.6, b4.6).fc((a4.7, b4.7).fc(Zero))),
                    (a4.6, b4.6).full_sum((a4.7, b4.7).fc(Zero)),
                    (a4.7, b4.7).half_sum(),
                )
            )
        }

}

