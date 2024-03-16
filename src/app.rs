use crate::assets::Asset;
use crate::assets::Assets;
use crate::common::resources::Clock;
use crate::common::resources::StringInterner;
use crate::plugin::Plugin;
use crate::plugin::Plugins;
use crate::prelude::QPError;
use crate::prelude::Schedule;
use crate::prelude::StartupSchedule;
use crate::prelude::StorageId;
use crate::prelude::StorageManager;
use crate::prelude::System;
use crate::prelude::UpdateSchedule;
use crate::prelude::World;
use crate::resources::Resource;
use crate::schedule::ScheduleManager;
use crate::QPResult;
use egui::TextBuffer;
use std::collections::HashSet;

pub struct App {
    pub world: World,

    runner: Box<dyn FnOnce(App) -> QPResult<()>>,

    plugins: Vec<Box<dyn Plugin>>,
    plugin_names: HashSet<String>,
    plugins_building_count: usize,

    // pub(crate) schedules: ScheduleManager,
    pub(crate) state: AppState,
}

impl App {
    fn empty() -> Self {
        Self {
            world: World::new(),
            runner: Box::new(run_once),
            plugins: vec![],
            plugin_names: HashSet::default(),
            plugins_building_count: 0,
            state: AppState::Created,
            // schedules: ScheduleManager::new(),
        }
    }

    pub fn new() -> Self {
        let mut app = Self::empty();
        app.add_plugins(Manadatory {});

        app
    }

    pub fn add_plugins(&mut self, plugins: impl Plugins) -> &mut Self {
        self.state = AppState::LoadingPlugins;

        plugins.install(self);

        self
    }

    pub(crate) fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().take();
        if self.plugin_names.contains(&name) {
            panic!("trying to add a duplicate plugin");
        }

        self.plugins_building_count += 1;
        if let Err(e) = plugin.build(self) {
            panic!("There was a problem building the plugins: {}", e);
        };
        self.plugins_building_count -= 1;

        self.plugin_names.insert(name);
        self.plugins.push(plugin);
    }

    pub fn add_system<S: Schedule>(&mut self, system: impl System) -> &mut Self {
        if let Some(schedules) = self.world.resource_mut::<ScheduleManager>() {
            schedules.add_system::<S>(system);
        } else {
            panic!("Schedule Manager not found in resources");
        }

        self
    }

    pub fn add_resource(&mut self, resource: impl Resource + 'static) -> &mut Self {
        if let Err(e) = self.world.resources.insert(resource) {
            panic!("there was a problem adding a resource: {}", e);
        }

        self
    }

    pub fn init_asset_store<A: Asset + 'static>(&mut self) -> &mut Self {
        if let Err(e) = self.world.resources.insert(Assets::<A>::default()) {
            panic!("there was a problem initializing asset store: {}", e)
        }

        self
    }

    pub fn set_runner(&mut self, runner: impl FnOnce(App) -> QPResult<()> + 'static) {
        self.runner = Box::new(runner);
    }

    pub fn run(&mut self) -> QPResult<()> {
        self.plugins_cleanup()?;

        let mut app = std::mem::replace(self, App::empty());

        if app.plugins_building_count > 0 {
            panic!("App::run() was called before all plugins were built");
        }

        app.world.execute::<StartupSchedule>()?;

        self.state = AppState::Running;

        let runner = std::mem::replace(&mut app.runner, Box::new(run_once));

        match runner(app) {
            Err(QPError::Quit) => Ok(()),
            Err(e) => Err(e),
            _ => Ok(()),
        }
    }

    fn plugins_cleanup(&mut self) -> QPResult<()> {
        let plugins = std::mem::take(&mut self.plugins);

        for plugin in &plugins {
            plugin.cleanup(self)?;
        }

        self.plugins = plugins;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResult {
    Quit,
    None,
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Created,
    LoadingConfig,
    LoadingPlugins,
    Running,
    Quiting,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

fn run_once(mut app: App) -> QPResult<()> {
    app.world.execute::<StartupSchedule>()
}

struct Manadatory {}
impl Plugin for Manadatory {
    fn build(&self, app: &mut App) -> QPResult<()> {
        let mut manager = StorageManager::new();
        manager.insert(StorageId::Entities)?;

        app.add_resource(Clock::new());
        app.add_resource(StringInterner::new());
        app.add_resource(manager);

        let mut schedules = ScheduleManager::new();

        schedules.add_schedule::<StartupSchedule>();
        schedules.add_schedule::<UpdateSchedule>();

        app.add_resource(schedules);

        Ok(())
    }
}
