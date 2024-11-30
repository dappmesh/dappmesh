use crate::actions::Action;

pub(crate) enum ApplicationView {
	Create,
	Info,
	Main,
}

pub(crate) struct ApplicationStore<'a> {
	dir: &'a String,
	pub(crate) view: &'a mut ApplicationView,
}

impl ApplicationStore {
	pub(crate) fn new() -> Self {
		Self {
			dir: &"undefined".to_string(),
			view: &mut ApplicationView::Main,
		}
	}

	pub(crate) fn update(&mut self, action: Action) {
		match action {
			Action::ViewCreate => self.view = &mut ApplicationView::Create,
			Action::ViewInfo => self.view = &mut ApplicationView::Info,
			Action::ViewMain => self.view = &mut ApplicationView::Main,
		}
	}

	pub(crate) fn get_state(&self) -> &String {
		self.dir
	}
}
