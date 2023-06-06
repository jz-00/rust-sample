fn to_json<T>(item: &T) -> serde_json::Value
where
    T: serde::Serialize,
{
    serde_json::to_value(&item).unwrap()
}

fn opt_to_json<T>(item: &Option<T>) -> Option<serde_json::Value>
where
    T: serde::Serialize,
{
    if let Some(value) = item {
        Some(serde_json::to_value(value).unwrap())
    } else {
        None
    }
}

pub trait ToJson: serde::Serialize {
    fn to_json(&self) -> serde_json::Value {
        self::to_json(&self)
    }
}

pub trait ToOptionJson: serde::Serialize {
    fn to_json(&self) -> Option<serde_json::Value>;
}

impl<T: serde::Serialize> ToOptionJson for Option<T> {
    fn to_json(&self) -> Option<serde_json::Value> {
        self::opt_to_json(&self)
    }
}
