use core_done::{service::Service, models::list::List};
use relm4::{gtk::{self, traits::{OrientableExt, WidgetExt, ButtonExt, GtkWindowExt}}, adw, component::{SimpleAsyncComponent, AsyncComponentParts}, AsyncComponentSender, factory::AsyncFactoryVecDeque, prelude::DynamicIndex, RelmWidgetExt, Controller, Component, ComponentController};
use relm4_icons::icon_name;

use crate::{app::{models::sidebar_list::SidebarList, factories::task_list::{TaskListFactoryModel, TaskListFactoryInit}, components::list_dialog::ListDialogOutput}, fl};

use super::list_dialog::ListDialogComponent;

pub struct TaskListSidebarModel {
	service: Service,
	status: TaskListSidebarStatus,
	task_list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	list_entry: Controller<ListDialogComponent>,
}

#[derive(Debug)]
pub enum TaskListSidebarInput {
	LoadTaskLists,
	OpenNewTaskListDialog,
	AddTaskListToSidebar(String),
	ServiceSelected(Service),
	SelectList(SidebarList),
	DeleteTaskList(DynamicIndex, String)
}

#[derive(Debug)]
pub enum TaskListSidebarOutput {
	SelectList(SidebarList, Service),
}

#[derive(Debug, PartialEq, Eq)]
enum TaskListSidebarStatus {
	Loading,
	Loaded
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for TaskListSidebarModel {
	type Input = TaskListSidebarInput;
	type Output = TaskListSidebarOutput;
	type Init = Service;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_width_request: 350,
			adw::HeaderBar {
				set_css_classes: &["flat"],
				set_show_end_title_buttons: false,
				set_title_widget: Some(&gtk::Label::new(Some("Lists"))),
				pack_end = &gtk::Button {
					set_tooltip: fl!("add-new-task-list"),
					set_icon_name: icon_name::PLUS,
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					connect_clicked => TaskListSidebarInput::OpenNewTaskListDialog
				},
			},
			gtk::CenterBox {
				set_vexpand: true,
				#[watch]
				set_visible: model.status == TaskListSidebarStatus::Loading,
				#[wrap(Some)]
				set_center_widget = &gtk::Spinner {
					start: ()
				}
			},
			gtk::ScrolledWindow {
				set_vexpand: true,
				#[watch]
				set_visible: model.status == TaskListSidebarStatus::Loaded,
				#[local_ref]
				task_list_widget -> gtk::ListBox {
					set_css_classes: &["navigation-sidebar"],
				},
			}
		}
	}

	async fn init(
		init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = TaskListSidebarModel {
			service: init,
			status: TaskListSidebarStatus::Loaded, 
			task_list_factory: AsyncFactoryVecDeque::new(
				gtk::ListBox::default(),
				sender.input_sender(),
			),
			list_entry: ListDialogComponent::builder()
			.launch(None)
			.forward(sender.input_sender(), |message| match message {
				ListDialogOutput::AddTaskListToSidebar(name) => {
					TaskListSidebarInput::AddTaskListToSidebar(name)
				},
				ListDialogOutput::RenameList(_) => todo!(),
			})
		};
		sender.input(TaskListSidebarInput::ServiceSelected(model.service));
		let task_list_widget = model.task_list_factory.widget();
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
		match message {
			TaskListSidebarInput::AddTaskListToSidebar(name) => {
				let mut guard = self.task_list_factory.guard();
				guard.push_back(TaskListFactoryInit {
					service: self.service,
					list: SidebarList::Custom(List::new(&name, self.service))
				});
			}
			TaskListSidebarInput::OpenNewTaskListDialog => {
				let list_entry = self.list_entry.widget();
				list_entry.present();
			}
			TaskListSidebarInput::ServiceSelected(service) => {
				self.service = service;
				self.status = TaskListSidebarStatus::Loading;
				sender.input(TaskListSidebarInput::LoadTaskLists);
			},
			TaskListSidebarInput::LoadTaskLists => {
				let mut guard = self.task_list_factory.guard();
				guard.clear();
				if matches!(self.service, Service::Smart) {
					for smart_list in SidebarList::list() {
						guard.push_back(TaskListFactoryInit::new(Service::Smart, smart_list));
					}
				} else {
					match self.service.get_service().read_lists().await {
						Ok(lists) => {
							for list in lists {
								guard.push_back(TaskListFactoryInit::new(
									self.service,
									SidebarList::Custom(list),
								));
							}
						},
						Err(err) => tracing::error!("{err}"),
					}
				}
				self.status = TaskListSidebarStatus::Loaded;
			}
			TaskListSidebarInput::SelectList(list) => {
				sender.output(TaskListSidebarOutput::SelectList(list, self.service.clone())).unwrap()
			},
			TaskListSidebarInput::DeleteTaskList(index, list_id) => {
				self.task_list_factory.guard().remove(index.current_index());
				tracing::info!("Deleted task list with id: {}", list_id);
			},
		}
	}
}