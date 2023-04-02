use group_sum_by::{group_sum_by, Agrupator, KeyValue, ObjectMap};
use std::collections::HashMap;

fn sum_number_by_group<A>(n: i32, result: &mut ObjectMap<i32>, group: &A)
where
    A: Agrupator<i32>,
{
    let group = group.get_group_key(n);
    if let Some(res) = result.get(&group) {
        result.insert(group, res + n);
    } else {
        result.insert(group, n);
    }
}

fn is_odd(n: i32) -> KeyValue {
    if n % 2 != 0 {
        //return KeyValue::String(String::from("Ímpar"));
        return KeyValue::Number(1);
    }
    //return KeyValue::String(String::from("Par"));
    return KeyValue::Number(0);
}

//fn get_group(n: i32, group: impl Agrupator<i32>) -> KeyValue {
//    return group.get_group_key(n);
//}

fn main() {
    let numbers = [1, 2, 3, 4];

    let mut result: ObjectMap<i32> = HashMap::new();
    numbers
        .into_iter()
        .for_each(|n| sum_number_by_group(n, &mut result, &is_odd));

    result.iter().for_each(|(k, v)| println!("{k:?} => {v}"));

    let result = group_sum_by(numbers.to_vec(), &is_odd, sum_number_by_group);
    result.iter().for_each(|(k, v)| println!("{k:?} => {v}"));
    return;
}
