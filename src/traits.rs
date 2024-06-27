/// Converts a type T into a Result<T, E>
pub trait IntoResult<T> {
    type Error;
    fn into_result(self) -> Result<T, Self::Error>;
}
