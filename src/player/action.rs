pub enum Action {
    CHECK,
    BET(u32),
    CALL,
    RAISE(u32),
    FOLD
}

