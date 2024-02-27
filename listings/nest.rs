impl RoundVM {
    // other RoundVM methods
    
    pub fn nest<A: Clone + 'static + FromStr, F>(
        &mut self,
        slot: Slot,
        write: bool,
        inc: bool,
        expr: F,
    ) -> A
    where
        F: Fn(&mut RoundVM) -> A,
    {
        self.status.push();
        self.status.nest(slot);
        let val = expr(self);
        let res = if write {
            let cloned_path = self.status.path().clone();
            self.export_data()
                .get::<A>(&cloned_path)
                .unwrap_or(
                    self.export_data()
                        .put_lazy_and_return(cloned_path, || val.clone()),
                )
                .clone()
        } else {
            val
        };
        if inc {
            self.status.pop();
            self.status.inc_index();
        } else {
            self.status.pop();
        }
        res
    }
}