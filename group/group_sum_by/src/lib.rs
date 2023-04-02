use std::collections::HashMap;
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum KeyValue {
    Number(i32),
    String(String),
}

pub type ObjectMap<T> = HashMap<KeyValue, T>;

pub trait Agrupator<T> {
    fn get_group_key(&self, object: T) -> KeyValue;
}

impl<T> Agrupator<T> for KeyValue {
    fn get_group_key(&self, _: T) -> KeyValue {
        return self.clone();
    }
}

impl<T, KeyFunction> Agrupator<T> for KeyFunction
where
    KeyFunction: Fn(T) -> KeyValue,
{
    fn get_group_key(&self, object: T) -> KeyValue {
        return self(object);
    }
}

pub fn group_sum_by<T, B>(
    to_group: Vec<T>,
    group_by: &B,
    summer: fn(object: T, result: &mut ObjectMap<T>, func: &B),
) -> ObjectMap<T>
where
    B: Agrupator<T>,
{
    let mut result: ObjectMap<T> = HashMap::new();
    for groupee in to_group {
        summer(groupee, &mut result, group_by);
    }
    return result;
}
