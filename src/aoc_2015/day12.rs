use crate::std_iter;

pub fn part1() {
    let data = std_iter!(Lines).next().unwrap();
    let value: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut sum = 0.0;
    let mut stack = vec![&value];

    while let Some(v) = stack.pop() {
        match v {
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
                ()
            }
            serde_json::Value::Number(n) => sum += n.as_f64().unwrap(),
            serde_json::Value::Array(arr) => arr.iter().for_each(|v| stack.push(v)),
            serde_json::Value::Object(obj) => obj.values().for_each(|v| stack.push(v)),
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let data = std_iter!(Lines).next().unwrap();
    let value: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut sum = 0.0;
    let mut stack = vec![&value];

    while let Some(v) = stack.pop() {
        match v {
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
                ()
            }
            serde_json::Value::Number(n) => sum += n.as_f64().unwrap(),
            serde_json::Value::Array(arr) => arr.iter().for_each(|v| stack.push(v)),
            serde_json::Value::Object(obj) => {
                if !obj.values().filter_map(|v| v.as_str()).any(|v| v == "red") {
                    obj.values().for_each(|v| stack.push(v))
                }
            }
        }
    }
    println!("{}", sum);
}
