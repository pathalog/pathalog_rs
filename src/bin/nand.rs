#![no_std]
fn main() {
    typelevel_tests();

    runtime_tests();
}


fn typelevel_tests() {
    {
        use pathalog::nand::typelevel::*;
        //nothing but test cases
        //first the compile time
        let _: Nand<Zero, Zero> = One;
        let _: Nand<Zero, One> = One;
        let _: Nand<One, Zero> = One;
        let _: Nand<One, One> = Zero;

        let _: Not<One> = Zero;
        let _: Not<Zero> = One;

        let _: And<Zero, Zero> = Zero;
        let _: And<Zero, One> = Zero;
        let _: And<One, Zero> = Zero;
        let _: And<One, One> = One;

        let _: Or<Zero, Zero> = Zero;
        let _: Or<Zero, One> = One;
        let _: Or<One, Zero> = One;
        let _: Or<One, One> = One;

        let _: Xnor<Zero, Zero> = One;
        let _: Xnor<Zero, One> = Zero;
        let _: Xnor<One, Zero> = Zero;
        let _: Xnor<One, One> = One;

        let _: Nor<Zero, Zero> = One;
        let _: Nor<Zero, One> = Zero;
        let _: Nor<One, Zero> = Zero;
        let _: Nor<One, One> = Zero;

        let _: Xor<Zero, Zero> = Zero;
        let _: Xor<Zero, One> = One;
        let _: Xor<One, Zero> = One;
        let _: Xor<One, One> = Zero;

        let _: HalfAdder<Zero, Zero> = (Zero, Zero);
        let _: HalfAdder<Zero, One> = (One, Zero);
        let _: HalfAdder<One, Zero> = (One, Zero);
        let _: HalfAdder<One, One> = (Zero, One);

        let _: FullAdder<Zero, Zero, Zero> = (Zero, Zero); //https://www.electronics-tutorials.ws/combination/comb_7.html
        let _: FullAdder<Zero, Zero, One> = (One, Zero);
        let _: FullAdder<Zero, One, Zero> = (One, Zero);
        let _: FullAdder<Zero, One, One> = (Zero, One);
        let _: FullAdder<One, Zero, Zero> = (One, Zero);
        let _: FullAdder<One, Zero, One> = (Zero, One);
        let _: FullAdder<One, One, Zero> = (Zero, One);
        let _: FullAdder<One, One, One> = (One, One);

        //0+0=0
        let _: ADD2<Zero, Zero, Zero, Zero> = (Zero, Zero);
        let _: ADD2<Zero, Zero, Zero, One> = (Zero, One);
        let _: ADD2<Zero, Zero, One, Zero> = (One, Zero);
        let _: ADD2<Zero, Zero, One, One> = (One, One);
        let _: ADD2<Zero, One, Zero, Zero> = (Zero, One);
        //1+1=2
        let _: ADD2<Zero, One, Zero, One> = (One, Zero);
        let _: ADD2<Zero, One, One, Zero> = (One, One);
        //1+3=4
        let _: ADD2<Zero, One, One, One> = (Zero, Zero);
        //2+0=2
        let _: ADD2<One, Zero, Zero, Zero> = (One, Zero);
        let _: ADD2<One, Zero, Zero, One> = (One, One);
        //2+2=4=>0
        let _: ADD2<One, Zero, One, Zero> = (Zero, Zero);
        let _: ADD2<One, Zero, One, One> = (Zero, One);
        let _: ADD2<One, One, Zero, Zero> = (One, One);
        let _: ADD2<One, One, Zero, One> = (Zero, Zero);
        let _: ADD2<One, One, One, Zero> = (Zero, One);
        //3+3=6=>2
        let _: ADD2<One, One, One, One> = (One, Zero);

        //0+0=0
        let _: ADD2CARRY<Zero, Zero, Zero, Zero> = (Zero, Zero, Zero);
        let _: ADD2CARRY<Zero, Zero, Zero, One> = (Zero, Zero, One);
        let _: ADD2CARRY<Zero, Zero, One, Zero> = (Zero, One, Zero);
        let _: ADD2CARRY<Zero, Zero, One, One> = (Zero, One, One);
        let _: ADD2CARRY<Zero, One, Zero, Zero> = (Zero, Zero, One);
        //1+1=2
        let _: ADD2CARRY<Zero, One, Zero, One> = (Zero, One, Zero);
        let _: ADD2CARRY<Zero, One, One, Zero> = (Zero, One, One);
        //1+3=4
        let _: ADD2CARRY<Zero, One, One, One> = (One, Zero, Zero);
        //2+0=2
        let _: ADD2CARRY<One, Zero, Zero, Zero> = (Zero, One, Zero);
        let _: ADD2CARRY<One, Zero, Zero, One> = (Zero, One, One);
        let _: ADD2CARRY<One, Zero, One, Zero> = (One, Zero, Zero);
        let _: ADD2CARRY<One, Zero, One, One> = (One, Zero, One);
        let _: ADD2CARRY<One, One, Zero, Zero> = (Zero, One, One);
        let _: ADD2CARRY<One, One, Zero, One> = (One, Zero, Zero);
        let _: ADD2CARRY<One, One, One, Zero> = (One, Zero, One);
        let _: ADD2CARRY<One, One, One, One> = (One, One, Zero);

        let _: Demux<Zero, Zero> = (Zero, Zero);
        let _: Demux<Zero, One> = (Zero, Zero);
        let _: Demux<One, Zero> = (One, Zero);
        let _: Demux<One, One> = (Zero, One);

        let _: Mux<Zero, Zero, Zero> = Zero;
        let _: Mux<Zero, One, Zero> = Zero;
        let _: Mux<One, Zero, Zero> = One;
        let _: Mux<One, One, Zero> = One;
        let _: Mux<Zero, Zero, One> = Zero;
        let _: Mux<Zero, One, One> = One;
        let _: Mux<One, Zero, One> = Zero;
        let _: Mux<One, One, One> = One;

        //https://www.quora.com/How-do-I-make-a-truth-table-of-a-4-bit-adder-circuit
        //0+0=0
        let _: ADD4<Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero> = (Zero, Zero, Zero, Zero);
        //0+1=1
        let _: ADD4<Zero, Zero, Zero, Zero, Zero, Zero, Zero, One> = (Zero, Zero, Zero, One);
        //1+1+2
        let _: ADD4<Zero, Zero, Zero, One, Zero, Zero, Zero, One> = (Zero, Zero, One, Zero);
        //0+2=2
        let _: ADD4<Zero, Zero, Zero, Zero, Zero, Zero, One, Zero> = (Zero, Zero, One, Zero);
        //0+3=3
        let _: ADD4<Zero, Zero, Zero, Zero, Zero, Zero, One, One> = (Zero, Zero, One, One);

        //0+4=4
        let _: ADD4<Zero, Zero, Zero, Zero, Zero, One, Zero, Zero> = (Zero, One, Zero, Zero);
        //2+5=7
        let _: ADD4<Zero, Zero, One, Zero, Zero, One, Zero, One> = (Zero, One, One, One);

        let _: ADD4<Zero, Zero, Zero, Zero, Zero, One, One, Zero> = (Zero, One, One, Zero);
        let _: ADD4<Zero, Zero, Zero, Zero, Zero, One, One, One> = (Zero, One, One, One);

        let _: ADD4<Zero, Zero, Zero, Zero, One, Zero, Zero, Zero> = (One, Zero, Zero, Zero);
        let _: ADD4<Zero, Zero, Zero, Zero, One, Zero, Zero, One> = (One, Zero, Zero, One);

        let _: ADD4<Zero, Zero, Zero, Zero, One, Zero, One, Zero> = (One, Zero, One, Zero);
        let _: ADD4<Zero, Zero, Zero, Zero, One, Zero, One, One> = (One, Zero, One, One);

        let _: ADD4<Zero, Zero, Zero, Zero, One, One, Zero, Zero> = (One, One, Zero, Zero);
        let _: ADD4<Zero, Zero, Zero, Zero, One, One, Zero, One> = (One, One, Zero, One);


        let _: ADD4<Zero, Zero, Zero, Zero, One, One, One, Zero> = (One, One, One, Zero);
        let _: ADD4<Zero, Zero, Zero, Zero, One, One, One, One> = (One, One, One, One);

        let _: ADD4<Zero, Zero, Zero, One, One, One, One, Zero> = (One, One, One, One);
        let _: ADD4<Zero, Zero, Zero, One, One, One, One, One> = (Zero, Zero, Zero, Zero);

        let _: ADD4<Zero, Zero, One, One, One, One, One, Zero> = (Zero, Zero, Zero, One);
        let _: ADD4<Zero, Zero, One, One, One, One, One, One> = (Zero, Zero, One, Zero);

        let _: ADD4<Zero, One, One, One, One, One, One, Zero> = (Zero, One, Zero, One);
        let _: ADD4<Zero, One, One, One, One, One, One, One> = (Zero, One, One, Zero);

        //7+6=13
        let _: ADD4<Zero, One, One, One, Zero, One, One, Zero> = (One, One, Zero, One);
        let _: ADD4<Zero, Zero, Zero, One, One, One, One, One> = (Zero, Zero, Zero, Zero);

        //15+15 = 30
        let _: ADD4<One, One, One, One, One, One, One, One> = (One, One, One, Zero); //30 == 0001:1110


        let _: Add8<
            Zero, Zero, Zero, Zero, Zero, Zero, One, Zero, //2
            Zero, Zero, Zero, Zero, Zero, Zero, Zero, One //1
        > = (Zero, Zero, Zero, Zero, Zero, Zero, One, One); //3

        let _: Add8<
            Zero, One, One, One, One, One, One, One,//127
            Zero, One, One, One, One, One, One, One//127
        > = (One, One, One, One, One, One, One, Zero); //254

        let _: Add8<
            Zero, Zero, One, Zero, One, Zero, One, Zero, //42
            Zero, One, Zero, Zero, One, One, One, One //79
        > = (Zero, One, One, One, One, Zero, Zero, One); //121

        let _: Add8<
            One, Zero, Zero, Zero, Zero, Zero, Zero, One,//129
            Zero, One, One, One, One, One, One, One//127
        > = (Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero); //256=>0

        let _: Add16<
            Zero, Zero, One, Zero, One, One, One, Zero, Zero, One, One, One, Zero, Zero, One, One,//11891
            One, Zero, Zero, Zero, One, Zero, One, Zero, One, One, Zero, One, One, Zero, One, Zero//35546
        > = (One, Zero, One, One, One, Zero, Zero, One, Zero, One, Zero, Zero, One, One, Zero, One); //47437

        let _: Add32<
            Zero, Zero, One, Zero, One, One, One, Zero, Zero, One, One, One, Zero, Zero, One, One, Zero, Zero, One, Zero, One, One, One, Zero, Zero, One, One, One, Zero, Zero, One, One,
            One, Zero, Zero, Zero, One, Zero, One, Zero, One, One, Zero, One, One, Zero, One, Zero, One, Zero, Zero, Zero, One, Zero, One, Zero, One, One, Zero, One, One, Zero, One, Zero
        > = (One, Zero, One, One, One, Zero, Zero, One, Zero, One, Zero, Zero, One, One, Zero, One, One, Zero, One, One, One, Zero, Zero, One, Zero, One, Zero, Zero, One, One, Zero, One);
    }
}

fn runtime_tests() {

    assert_eq!(Zero.nand(Zero), One);
    assert_eq!(Zero.nand(Zero), One);
    assert_eq!(One.nand(Zero), One);
    assert_eq!(One.nand(One), Zero);

    assert_eq!(One.not(), Zero);
    assert_eq!(Zero.not(), One);

    assert_eq!(Zero.and(Zero), Zero);
    assert_eq!(Zero.and(One), Zero);
    assert_eq!(One.and(Zero), Zero);
    assert_eq!(One.and(One), One);

    assert_eq!(Zero.or(Zero), Zero);
    assert_eq!(Zero.or(One), One);
    assert_eq!(One.or(Zero), One);
    assert_eq!(One.or(One), One);

    assert_eq!(Zero.xor(Zero), Zero);
    assert_eq!(Zero.xor(One), One);
    assert_eq!(One.xor(Zero), One);
    assert_eq!(One.xor(One), Zero);

    assert_eq!(Zero.xnor(Zero), One);
    assert_eq!(Zero.xnor(One), Zero);
    assert_eq!(One.xnor(Zero), Zero);
    assert_eq!(One.xnor(One), One);

    assert_eq!((Zero, Zero).half_carry(), Zero);
    assert_eq!((Zero, One).half_carry(), Zero);
    assert_eq!((One, Zero).half_carry(), Zero);
    assert_eq!((One, One).half_carry(), One);

    assert_eq!((Zero, Zero).half_sum(), Zero);
    assert_eq!((Zero, One).half_sum(), One);
    assert_eq!((One, Zero).half_sum(), One);
    assert_eq!((One, One).half_sum(), Zero);

    assert_eq!((Zero, Zero).half_adder(), (Zero, Zero));
    assert_eq!((Zero, One).half_adder(), (One, Zero));
    assert_eq!((One, Zero).half_adder(), (One, Zero));
    assert_eq!((One, One).half_adder(), (Zero, One));

    assert_eq!((Zero, Zero).fc(Zero), Zero);
    assert_eq!((Zero, One).fc(One), One);
    assert_eq!((One, Zero).fc(Zero), Zero);
    assert_eq!((One, One).fc(One), One);


    assert_eq!((Zero, Zero).full_add(Zero), ((Zero, Zero)));
    assert_eq!((Zero, Zero).full_add(One), ((One, Zero)));
    assert_eq!((Zero, One).full_add(Zero), ((One, Zero)));
    assert_eq!((Zero, One).full_add(One), ((Zero, One)));
    assert_eq!((One, Zero).full_add(Zero), ((One, Zero)));
    assert_eq!((One, Zero).full_add(One), ((Zero, One)));
    assert_eq!((One, One).full_add(Zero), ((Zero, One)));
    assert_eq!((One, One).full_add(One), ((One, One)));

    //Here we move to using the tuple-based definitions
    //Testing two bit adder
    assert_eq!(((Zero, Zero).add((Zero, Zero))), ((Zero, Zero)));
    assert_eq!(((Zero, Zero).add((Zero, One))), ((Zero, One)));
    assert_eq!(((Zero, Zero).add((One, Zero))), ((One, Zero)));
    assert_eq!(((Zero, Zero).add((One, One))), ((One, One)));
    assert_eq!(((Zero, One).add((Zero, Zero))), ((Zero, One)));
    assert_eq!(((Zero, One).add((Zero, One))), ((One, Zero)));
    assert_eq!(((Zero, One).add((One, Zero))), ((One, One)));
    assert_eq!(((Zero, One).add((One, One))), ((Zero, Zero)));
    assert_eq!(((One, Zero).add((Zero, Zero))), ((One, Zero)));
    assert_eq!(((One, Zero).add((Zero, One))), ((One, One)));
    assert_eq!(((One, Zero).add((One, Zero))), ((Zero, Zero)));
    assert_eq!(((One, Zero).add((One, One))), ((Zero, One)));
    assert_eq!(((One, One).add((Zero, Zero))), ((One, One)));
    assert_eq!(((One, One).add((Zero, One))), ((Zero, Zero)));
    assert_eq!(((One, One).add((One, Zero))), ((Zero, One)));
    assert_eq!(((One, One).add((One, One))), ((One, Zero)));


    //testing 4-bit adder
    assert_eq!(((One, One, One, One).add((Zero, Zero, Zero, Zero))), ((One, One, One, One)));
    assert_eq!(((One, One, Zero, Zero).add((Zero, Zero, One, One))), ((One, One, One, One)));
    assert_eq!(((One, One, One, One).add((Zero, Zero, Zero, Zero))), ((One, One, One, One)));
    assert_eq!(((One, Zero, One, Zero).add((Zero, Zero, One, One))), ((One, One, Zero, One)));
    assert_eq!(((One, One, One, One).add((Zero, Zero, Zero, Zero))), ((One, One, One, One)));
    assert_eq!(((One, One, Zero, Zero).add((Zero, Zero, One, One))), ((One, One, One, One)));
    assert_eq!(((One, One, Zero, One).add((Zero, Zero, One, One))), (((Zero, Zero, Zero, Zero))));
    assert_eq!(((One, One, Zero, One).add((One, Zero, One, One))), (((One, Zero, Zero, Zero))));

    //testing 8-bit adder
    //255+0=255
    assert_eq!(((One, One, One, One, One, One, One, One).add(
        (Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero))),
               ((One, One, One, One, One, One, One, One)));

    //183+96=375-256=>23
    assert_eq!(((One, Zero, One, One, Zero, One, One, One).add(
        (Zero, One, One, Zero, Zero, Zero, Zero, Zero))),
               ((Zero, Zero, Zero, One, Zero, One, One, One)));


//16-bit adder
//255+0=255
    assert_eq!((((One, One, One, One, One, One, One, One), (One, One, One, One, One, One, One, One)).add(
        ((Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero), (Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero)))),
               ((One, One, One, One, One, One, One, One), (One, One, One, One, One, One, One, One)));

//16-bit adder
//255+0=255
    assert_eq!(((
        (Zero, Zero, One, Zero, One, One, One, Zero), (Zero, One, One, One, Zero, Zero, One, One)).add(//11891
        ((One, Zero, Zero, Zero, One, Zero, One, Zero), (One, One, Zero, One, One, Zero, One, Zero)))),//35546
               ((One, Zero, One, One, One, Zero, Zero, One), (Zero, One, Zero, Zero, One, One, Zero, One)));//47437

//16-bit adder
//255+0=255
//   32342161 = 00000001 11101101 10000000 10010001
//   532 = 00000000 00000000 00000010 00010100
//    => 323426693 = 00000001 11101101 10000010 10100101
    assert_eq!(((
        (Zero, Zero, Zero, Zero, Zero, Zero, Zero, One), (One, One, One, Zero, One, One, Zero, One), (One, Zero, Zero, Zero, Zero, Zero, Zero, Zero), (One, Zero, Zero, One, Zero, Zero, Zero, One)).add(//32342161
        ((Zero, Zero, Zero, Zero, Zero, Zero, Zero,Zero), (Zero, Zero, Zero, Zero, Zero, Zero, Zero,Zero),(Zero, Zero, Zero, Zero, Zero, Zero, One,Zero), (Zero, Zero, Zero, One, Zero, One, Zero, Zero)))),//532
               ((Zero, Zero, Zero, Zero, Zero, Zero, Zero, One), (One, One, One, Zero, One, One, Zero, One), (One, Zero, Zero, Zero, Zero, Zero, One, Zero), (One, Zero, One, Zero, Zero, One, Zero, One)));//32342693
}
