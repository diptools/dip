#[derive(Default, Clone, Debug)]
pub struct Settings {
    pub filter: Filter,
}

#[derive(Clone, Debug)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}
