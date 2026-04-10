pub struct Categorized {
    pub errors: Vec<(String, usize)>,
    pub warnings: Vec<(String, usize)>,
    pub infos: Vec<(String, usize)>,
}
