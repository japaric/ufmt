macro_rules! assume_unreachable {
    () => {
        if cfg!(debug_assertions) {
            unreachable!()
        } else {
            core::hint::unreachable_unchecked()
        }
    };
}
