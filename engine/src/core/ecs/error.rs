#[derive(Debug, thiserror::Error)]
pub enum ECSError {
    #[error("there was a problem creating a new component registry")]
    ProblemCreatingNewComponentRegistry
}
