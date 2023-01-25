
/// Takes a reference or a name and returns the namespace and name.
pub fn resolve(name: &str) -> (String, String) {
    let mut parts: Vec<String> = name.trim_start_matches("#/components/schemas/")
        .split('.')
        .map(|s| s.to_string())
        .collect();
    let name = parts.pop().unwrap();
    (parts.join("/"), name)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_resolve() {
        let (namespace, name) = super::resolve("#/components/schemas/MyObject");
        assert_eq!(namespace, "");
        assert_eq!(name, "MyObject");

        let (namespace, name) = super::resolve("#/components/schemas/MyNamespace.MyObject");
        assert_eq!(namespace, "MyNamespace");
    }

}
