pub fn gradient() -> fn(&RoundVM) -> f64 {
    fn is_source(vm: &mut RoundVM) -> bool {
        vm.local_sense::<bool>(&sensor("source")).unwrap().clone()
    }

    rep!(|_| f64::INFINITY, |vm1, d| {
        mux(
            vm1,
            is_source,
            |_| 0.0,
            foldhood_plus!(|_| f64::INFINITY, |a, b| a.min(b), |vm2| {
                nbr(vm2, |_| d) + 1.0
            }),
        )
    })
}