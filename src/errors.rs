#[derive(Debug, thiserror::Error)]
pub enum QPError {
    #[error("generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    TobjError(#[from] tobj::LoadError),

    // #[error(transparent)]
    // GltfError(#[from] gltf::Error),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    FontError(#[from] ft::Error),

    #[error("There was a problem drawing the frame")]
    ProblemSwappingFrameBuffers,

    #[error("there was a problem creating a new component registry")]
    ProblemCreatingNewComponentGlobalRegistry,

    #[error("there was a problem creating a new entity")]
    ProblemCreatingEntity,

    #[error("trying to load a duplicate resource")]
    DuplicateResource,

    #[error("counldn't find resource {0}")]
    ResourceNotFound(String),

    #[error("trying to load an existing storage unit")]
    DuplicateStorage,

    #[error("trying to load as existing asset")]
    DuplicateAsset,

    #[error("counldn't find asset {0}")]
    AssetNotFound(String),

    #[error("failed trying to load asset {0}")]
    FailedLoadingAsset(String),

    #[error("shader not found")]
    ShaderNotFound,

    #[error("storage manager not found")]
    StorageManagerNotFound,

    #[error("camera not found")]
    CameraNotFound,

    #[error("texture doesn't exist")]
    SpriteTextureDoesntExist,

    #[error("file contains nil value")]
    FileContainsNil,

    #[error("there was an error compiling the shader: {}", .0)]
    CompileError(String),

    #[error("there was a problem linking the program")]
    LinkingError,

    #[error("there was a problem adding the texture image")]
    FailedAddingTextureImage,

    #[error("there was a problem adding a parameter to the texture")]
    FailedAddingParameter,

    #[error("the wavefront material file doesn't have a texture path")]
    CouldntFindWavefrontTexture,

    #[error("camera is not loaded")]
    CameraNotLoaded,

    #[error("shader is not loaded")]
    ShaderNotLoaded,

    #[error("failed to upgrade weak reference")]
    SharedReferenceDropped,

    #[error("failed to get a lock: {0}")]
    MutexLockFailed(String),

    /// this is a special error used to quit the application
    #[error("quitting the application")]
    Quit,
}
