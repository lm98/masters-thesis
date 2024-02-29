#[test]
fn export_should_compose() {
    fn ctx() -> Context {
        Context::new(
            0,
            HashMap::from([(sensor("sensor"), Rc::new(Box::new(5) as Box<dyn Any>))]),
            Default::default(),
            Default::default(),
        )
    }

    let expr_1 = |_vm: &mut RoundVM| 1;
    let expr_2 = |vm: &mut RoundVM| rep(vm, |_vm1| 7, |_vm2, val| val + 1);
    let expr_3 = |vm: &mut RoundVM| {
        foldhood(
            vm,
            |_vm1| 0,
            |a, b| (a + b),
            |vm2| {
                nbr(vm2, |vm3| {
                    *vm3.local_sense::<i32>(&sensor("sensor")).unwrap()
                })
            },
        )
    };

    let mut vm = init_vm();
    let _ = round(&mut vm, combine(expr_1, expr_1.clone(), |a, b| a + b));
    assert_eq!(2, vm.export_data().root::<i32>().clone());
}