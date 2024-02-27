struct RoundVM {
    context: Context,
    status: VMStatus,
    export_stack: Vec<Export>,
    isolated: bool,
}