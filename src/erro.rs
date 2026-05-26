use std::result;

type Result<T> = result::Result<T, Err>;

// Not sure how to do this yet, but all err enum
#[derive]
enum Err<E: String> {
    ValueError(E),
    TaskFailed(E),
}
