pub struct LabelManager;

impl LabelManager {
    pub fn sanitize(name: &str) -> String {
        name.replace(".", "_")
    }

    pub fn block_label(name: &str) -> String {
        format!(".{}", Self::sanitize(name))
    }
}
