use serde_json::{self, Value};

#[derive(Debug)]
/// Representing [Schema.org](https://schema.org/) information (currently only via JSON-LD)
pub struct SchemaOrg {
    /// Schema.org type (article, image, event)
    pub schema_type: String,
    /// Schema.org info
    pub value: Value,
}

impl SchemaOrg {
    pub fn from(content: String) -> Vec<Self> {
        let node: Value = serde_json::from_str(&content).unwrap_or(Value::Null);

        let vals: Vec<Value>;
        if let Value::Array(arr) = node {
            vals = arr;
        } else {
            vals = vec![node];
        }

        vals.into_iter()
            .flat_map(|v| {
                let type_opt = v["@type"].clone();
                if let Value::String(ref type_val) = type_opt {
                    return Some(SchemaOrg {
                        schema_type: type_val.to_string(),
                        value: v,
                    });
                }
                None
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SchemaOrg;

    #[test]
    fn test_empty() {
        let schema = SchemaOrg::from("{}".to_string());
        assert!(schema.is_none());
    }

    #[test]
    fn test_type() {
        let schema = SchemaOrg::from("{\"@type\": \"article\"}".to_string());
        assert!(schema.is_some());
        assert_eq!(schema.unwrap().schema_type, "article");
    }
}
