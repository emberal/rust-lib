pub mod pool;

/// Re-export diesel::result::Error as DieselError
pub type DieselError = diesel::result::Error;
