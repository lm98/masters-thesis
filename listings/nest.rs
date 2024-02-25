// Inside VM
fn nest<A, F>(&mut self, slot: Slot, expr: F, inc_index: bool, write: bool) -> A
where
    F: Fn(&mut RoundVM) -> A,
{
    // push the slot onto the current Path in VMStatus

    // compute expr result

    // if write is true, check if the export has a value for the current path
    // if not, write the result to the export

    // if inc_index is true, increment the index of the VMStatus (for ast navigation)

    // return the expr result
}