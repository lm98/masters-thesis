mod lang {
    pub fn nbr<A, F>(vm: &mut RoundVM, expr: F) -> A 
    where
        F: Fn(&mut RoundVM) -> A,
    {
        //call vm.nest with the Nbr slot, passing also expr to it and then return the result
    }

    pub fn rep<A, F, G>(vm: &mut RoundVM, init: F, fun: G) -> A 
    where
        F: Fn(&mut RoundVM) -> A,
        G: Fn(&mut RoundVM, A) -> A,
    {
        //...
    }

    pub fn foldhood<A, F, G, H>(vm: &mut RoundVM, init: F, aggr: G, expr: H) -> A
    where
        F: Fn(&mut RoundVM) -> A,
        G: Fn(A, A) -> A,
        H: Fn(&mut RoundVM) -> A,
    {
        //...
    }

    pub fn branch<A, B, TH, EL>(vm: &mut RoundVM, cond: B, thn: TH, els: EL) -> A
    where
        B: Fn() -> bool,
        TH: Fn(&mut RoundVM) -> A,
        EL: Fn(&mut RoundVM) -> A,
    {
        //...
    }
}