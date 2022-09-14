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


fn main() {
    println!("Hello, world!");
}
