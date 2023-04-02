enum KeyValue {
    Number(i32),
    String(String),
}

type ObjectMap<T> = HashMap<KeyValue, T>;

trait Agrupator<T> {
    fn get_group_key(&self, object: T) -> KeyValue;
}

impl<T, KeyFunction> Agrupator<T> for KeyFunction
where
    KeyFunction: Fn(T) -> KeyValue,
{
    fn get_group_key(&self, object: T) -> KeyValue {
        return self(object);
    }
}

impl Agrupator<i32> for KeyValue {
    fn get_group_key(&self, _: i32) -> KeyValue {
        return self.clone();
    }
}
