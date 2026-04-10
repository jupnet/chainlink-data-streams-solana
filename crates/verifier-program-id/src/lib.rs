#[macro_export]
macro_rules! inject_verifier_program_id {
    ($macro:ident) => {
        $macro! {"9hFjbrru29w1WfvvPsNgorDZKAWZRstLXJhG7tQe4bWN"}
    };
}

macro_rules! identity {
    ($x:literal) => {
        $x
    };
}

pub const VERIFIER_PROGRAM_ID: &str = inject_verifier_program_id!(identity);

pub mod solana {
    pub const VERIFIER_PROGRAM_ID: &str = "9hFjbrru29w1WfvvPsNgorDZKAWZRstLXJhG7tQe4bWN";
}
