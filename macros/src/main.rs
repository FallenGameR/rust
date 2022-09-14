use std::collections::HashMap;

enum Json
{
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

/// Construct Json representation for any Rust number type
macro_rules! impl_from_num_to_json
{
    ( $($t:ident)* ) => { $(
        impl From<$t> for Json
        {
            fn from(n: $t) -> Json
            {
                Json::Number(n as f64)
            }
        }
    )* }
}

impl_from_num_to_json!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

/// Construct JSON via macro
/// tt is token tree
macro_rules! json
{
    (null) => {
        Json::Null
    };
    ( [$($element:tt),*] ) => {
        Json::Array(vec![ $( json!($element) ),* ])
    };
    ( {$($key:tt : $value:tt),*} ) => {
        Json::Object(Box::new(vec![
            $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect()))
    };
    ( $other:tt ) => {
        Json::from($other)
    };
}

fn main() {
    todo!("Add macro debugging info ");
    println!("Hello, world!");
}
