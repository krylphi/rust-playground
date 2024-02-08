fn call_with_one<F: Fn(usize) -> usize>(func: F) -> usize {
    func(1)
}