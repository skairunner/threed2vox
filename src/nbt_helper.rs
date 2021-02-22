// Adds some Rust to NBT encapsulations

use nbt::Value;

/// Creates a Value::List of Value::Ints, as used in the Structure file spec.
/// Schematic tends to use IntArray instead.
pub(crate) fn list_from_intvec(v: Vec<i32>) -> Value {
    let v = v.into_iter().map(|i| Value::Int(i)).collect();
    Value::List(v)
}
