use crate::actions::{Action, ActionComponent};
use crate::store::app::ApplicationStore;
use crate::store::plugin::PluginStore;

pub(crate) struct MainStore<'a> {
	application: ApplicationStore<'a>,
	plugin: PluginStore<'a>,
}

impl MainStore {
	pub(crate) fn new() -> Self {
		Self {
			application: ApplicationStore::new(),
			plugin: PluginStore::new(),
		}
	}

	pub(crate) fn update(&mut self, component: ActionComponent, action: Action) {
		match component {
			ActionComponent::Application => self.application.update(action),
			ActionComponent::Plugin => self.plugin.update(action),
		}
	}
}
