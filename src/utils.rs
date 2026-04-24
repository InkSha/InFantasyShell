pub fn read_yaml<T: ToString>(path: T) -> Result<serde_yaml::Value, serde_yaml::Error> {
    let yaml_str = std::fs::read_to_string(path.to_string()).expect("Failed to read YAML file");

    serde_yaml::from_str(&yaml_str)
}
