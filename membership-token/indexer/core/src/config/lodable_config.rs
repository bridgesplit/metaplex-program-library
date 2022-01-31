pub trait LoadableConfig<'a> {
    fn load(&mut self) -> Result<&Self, &'a str>;

    fn set(&self, section: &str, key: &str, value: &str);

    fn get(&self, section: &str, key: &str) -> Option<&str>;
}
