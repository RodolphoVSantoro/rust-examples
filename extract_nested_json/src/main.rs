use serde::Deserialize;

#[derive(Deserialize)]
struct ID {
    #[serde(rename = "$oid")]
    oid: String,
}

struct Test {
    id: String,
}

impl<'de> serde::de::Deserialize<'de> for Test {
    fn deserialize<D>(deserializer: D) -> Result<Test, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Test { id: s })
    }
}

fn main() {
    println!("Hello, world!");
}
