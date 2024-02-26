pub fn gradient(vm: &mut RoundVM) -> f64 {
    fn is_source(vm: &mut RoundVM) -> bool {
        vm.local_sense::<bool>(&sensor("source")).unwrap().clone()
    }

    rep(
        vm,
        |_| 0.0,
        |vm1, d| {
            mux(
                vm1,
                is_source,
                |_vm| 0.0,
                |vm2| {
                    foldhood_plus(
                        vm2,
                        |_vm| f64::INFINITY,
                        |a, b| a.min(b),
                        |vm3| nbr(vm3, |_vm| d) + 1.0,
                    )
                },
            )
        },
    )
}