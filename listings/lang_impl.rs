// The language is now a public module that exposes pure functions
pub mod lang {

    pub fn nbr<A, F>(vm: &mut RoundVM, expr: F) -> A
    where
        A : Clone + 'static + FromStr,
        F: Fn(&mut RoundVM) -> A,
    {
        vm.nest(
            Nbr(vm.index()),
            vm.unless_folding_on_others(),
            true,
            |vm| match vm.neighbor() {
                Some(nbr) if nbr != vm.self_id() => match vm.neighbor_val::<A>() {
                    Ok(val) => val,
                    _ => expr(vm),
                },
                _ => expr(vm),
            },
        )
    }

    // other constructs omitted
}