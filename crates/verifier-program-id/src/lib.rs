#[macro_export]
macro_rules! inject_verifier_program_id {
    ($macro:ident) => {
        $macro! {"7KhNwmEmfhBjQwnD6QTi5ek8jFZ8WrP8KMYWmPQgYUn9"}
    };
}

macro_rules! identity {
    ($x:literal) => {
        $x
    };
}

pub const VERIFIER_PROGRAM_ID: &str = inject_verifier_program_id!(identity);

pub mod solana {
    pub const VERIFIER_PROGRAM_ID: &str = "Gt9S41PtjR58CbG9JhJ3J6vxesqrNAswbWYbLNTMZA3c";
}
