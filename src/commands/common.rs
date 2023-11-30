#[derive(Debug)]
pub enum ValueType<B, T, N> {
    OneOf(T),
    AllOf(N),
    All(B),
    Last(B),
    Default,
}
