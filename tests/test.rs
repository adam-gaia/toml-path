#[cfg(test)]
mod tests {
    #[test]
    fn test_readme_examples() {
        trycmd::TestCases::new().case("README.md");
    }
}
