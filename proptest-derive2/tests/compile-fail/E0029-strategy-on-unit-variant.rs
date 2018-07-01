#[macro_use]
extern crate proptest_derive;

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0029]
                            //~| [proptest_derive, E0008]
enum NonFatal {
    #[proptest(strategy = "Just(T0::V0)")]
    V0,
    V1 {
        #[proptest(skip)]
        field: usize,
    }
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T0 {
    #[proptest(strategy = "Just(T0::V0)")]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T1 {
    #[proptest(value = "T0::V0")]
    V0,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T2 {
    #[proptest(strategy = "Just(T0::V0)")]
    V0 {},
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T3 {
    #[proptest(value = "T0::V0")]
    V0 {},
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T4 {
    #[proptest(strategy = "Just(T0::V0)")]
    V0(),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0029]
enum T5 {
    #[proptest(value = "T0::V0")]
    V0(),
}
