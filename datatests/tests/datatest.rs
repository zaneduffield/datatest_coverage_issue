fn call_fn_to_cover(_: &std::path::Path) -> datatest_stable::Result<()> {
    to_cover::main();
    Ok(())
}

// coverage properly generated when using regular main function

// fn main() {
//     to_cover::main();
// }

// coverage not generated when using datatest_stable::harness macro
datatest_stable::harness!(
    call_fn_to_cover,
    ".",
    r"test$",
);