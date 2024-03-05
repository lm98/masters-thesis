mod lang {
    pub fn nbr<A, F>(vm: &mut RoundVM, expr: F) -> A 
    where
        // F is a generic type bound that represents a function that takes a mutable reference to a RoundVM and returns a value of type A
        F: Fn(&mut RoundVM) -> A,
    {
        //call vm.nest with the Nbr slot, passing also expr to it and then return the result
    }

    pub fn rep<A, F, G>(vm: &mut RoundVM, init: F, fun: G) -> A 
    where
        F: Fn(&mut RoundVM) -> A,
        // G is a generic type bound that represents a function that takes a mutable reference to a RoundVM and a value of type A and returns a value of type A
        G: Fn(&mut RoundVM, A) -> A,
    {
        //...
    }

    pub fn foldhood<A, F, G, H>(vm: &mut RoundVM, init: F, aggr: G, expr: H) -> A
    where
        F: Fn(&mut RoundVM) -> A,
        // G is a generic type bound that represents an aggregator function that takes two values of type A and returns a value of type A
        G: Fn(A, A) -> A,
        H: Fn(&mut RoundVM) -> A,
    {
        //...
    }

    pub fn branch<A, B, TH, EL>(vm: &mut RoundVM, cond: B, thn: TH, els: EL) -> A
    where
        // B is a generic type bound that represents a function that takes no arguments and returns a boolean
        B: Fn() -> bool,
        // TH and EL are generic type bounds that represent functions that take a mutable reference to a RoundVM and return a value of type A. 
        // TH is the type of the function that will be called if the condition is true, and EL is the type of the function that will be called if the condition is false.
        // We need two different type bounds here because in Rust, closures have unique types, even if they have the same signature.
        TH: Fn(&mut RoundVM) -> A,
        EL: Fn(&mut RoundVM) -> A,
    {
        //...
    }
}