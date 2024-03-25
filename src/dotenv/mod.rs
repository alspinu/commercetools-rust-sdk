pub mod types;

pub mod dotenv {
    use crate::dotenv::types::EnvironmentVariable;
    use std::fmt;
    use std::fs;
    use std::path::Path;
    use std::vec::IntoIter;
    impl fmt::Display for EnvironmentVariable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("")
                .field(&self.key)
                .field(&self.value)
                .finish()
        }
    }

    impl fmt::Debug for EnvironmentVariable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(key={}, value={})", self.key, self.value)
        }
    }

    pub trait EnvironmentVariableCollection {
        fn getByKey(&self, key: &str) -> &EnvironmentVariable;
        fn getValueByKey(&self, key: &str) -> &str;
    }

    impl EnvironmentVariableCollection for IntoIter<EnvironmentVariable> {
        fn getByKey(&self, key: &str) -> &EnvironmentVariable {
            return &self
                .as_ref()
                .into_iter()
                .find(|variable| variable.key == key)
                .unwrap();
        }
        fn getValueByKey(&self, key: &str) -> &str {
            return &self.getByKey(key).value.as_str();
        }
    }

    pub fn parse_env() -> IntoIter<EnvironmentVariable> {
        let file_path = "./.env";
        let resolved_path = Path::new(&file_path);
        let contents = fs::read_to_string(resolved_path).unwrap();
        let contents_result = contents.as_str();
        let parsed_env: Vec<EnvironmentVariable> = Vec::from_iter(
            contents_result
                .trim_end()
                .split("\n")
                .map(|variable| {
                    let split_variable: Vec<&str> = Vec::from_iter(variable.split("="));
                    return EnvironmentVariable {
                        key: split_variable[0].to_string(),
                        value: split_variable[1].to_string(),
                    };
                })
                .into_iter(),
        );
        return parsed_env.into_iter();
    }
}
