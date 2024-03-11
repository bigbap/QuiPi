use crate::{
    common::resources::{
        Asset, AssetId, AssetLoader, AssetStore, Camera, CameraList, StringInterner,
    },
    prelude::QPError,
    resources::{Resource, ResourceManager},
    QPResult,
};

pub struct Commands {
    commands: Vec<Box<dyn Command>>,
}

impl Commands {
    pub(crate) fn new() -> Self {
        Self { commands: vec![] }
    }

    fn insert(&mut self, command: impl Command) {
        self.commands.push(Box::new(command))
    }

    pub(crate) fn flush(&mut self, resources: &mut ResourceManager) -> QPResult<()> {
        while let Some(mut command) = self.commands.pop() {
            command(resources)?;
        }

        Ok(())
    }

    pub fn add_resource<R: Resource + Copy + 'static>(&mut self, resource: R) -> &mut Self {
        self.insert(move |resources: &mut ResourceManager| {
            resources.add_resource::<R>(resource);

            Ok(())
        });

        self
    }

    /// ///////////
    ///
    /// Assets
    ///
    /// ///////////

    pub fn load_asset<A: Asset + 'static>(
        &mut self,
        identifier: String,
        loader: impl AssetLoader<A> + 'static,
    ) -> &mut Self {
        self.insert(move |resources: &mut ResourceManager| {
            let interner = resources
                .get_mut::<StringInterner>()
                .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

            let id = interner.intern(identifier.clone());

            let store = resources
                .get_mut::<AssetStore<A>>()
                .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

            store.load_asset(loader, id);

            Ok(())
        });

        self
    }

    pub fn unload_asset<A: Asset + 'static>(&mut self, id: AssetId) -> &mut Self {
        self.insert(move |resources: &mut ResourceManager| {
            let store = resources
                .get_mut::<AssetStore<A>>()
                .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

            store.unload_asset(id);
            Ok(())
        });

        self
    }

    /// ///////////
    ///
    /// Cameras
    ///
    /// ///////////

    pub fn add_camera<C: Camera + 'static>(&mut self, identifier: String, camera: C) -> &mut Self {
        self.insert(move |resources: &mut ResourceManager| {
            let interner = resources
                .get_mut::<StringInterner>()
                .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

            let id = interner.intern(identifier.clone());

            let store = resources
                .get_mut::<CameraList>()
                .ok_or(QPError::ResourceNotFound("CameraList".into()))?;

            store.add_camera(id, camera);
            Ok(())
        });

        self
    }

    /// ///////////
    ///
    /// Storage
    ///
    /// ///////////

    pub fn spawn(&mut self) -> &mut Self {
        self
    }
}

pub trait Command: FnMut(&mut ResourceManager) -> QPResult<()> + 'static {}
impl<F> Command for F where F: FnMut(&mut ResourceManager) -> QPResult<()> + 'static {}
