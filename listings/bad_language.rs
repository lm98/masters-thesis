trait Language {
    fn nbr<A, F> (&mut self, f: F) -> A
    where
        F: Fn() -> A + Copy,
        A: Clone + 'static + FromStr;

    // other constructs omitted for brevity
}

impl Language for RoundVM {
    fn nbr<A, F> (&mut self, f: F) -> A
    where
        F: Fn() -> A + Copy,
        A: Clone + 'static + FromStr,
    {
        self.nest(
            Nbr(self.index()),
            self.unless_folding_on_others(),
            true,
            || match self.neighbor() {
                Some(nbr) if nbr != self.self_id() => match self.neighbor_val::<A>() {
                    Ok(val) => val,
                    _ => f(),
                },
                _ => f(),
            },
        )
    }
}