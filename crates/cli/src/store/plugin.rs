use crate::actions::Action;

pub(crate) enum PluginView {
	Create,
	Info,
	Main,
}

pub(crate) struct PluginStore<'a> {
	dir: &'a String,
	pub(crate) view: &'a mut PluginView,
}

impl PluginStore {
	pub(crate) fn new() -> Self {
		Self {
			dir: &"undefined".to_string(),
			view: &mut PluginView::Main,
		}
	}

	pub(crate) fn update(&mut self, action: Action) {
		match action {
			Action::ViewCreate => self.view = &mut PluginView::Create,
			Action::ViewInfo => self.view = &mut PluginView::Info,
			Action::ViewMain => self.view = &mut PluginView::Main,
		}
	}

	pub(crate) fn get_state(&self) -> &String {
		self.dir
	}
}
