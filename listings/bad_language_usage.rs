fn main() {
    let ctx = Context::new(0, Default::default(), Default::default(), Default::default());
    let mut vm = RoundVM::new(ctx);
    let result: i32 = vm.nbr(|| 0); // This code compiles without errors and result is 0.

    let nested_result: i32 = vm.nbr(|| vm.nbr(|| 0)); // This code does not compile: `vm` doesn't implement the Copy trait.
}