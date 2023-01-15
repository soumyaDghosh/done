use relm4::adw::prelude::ActionRowExt;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{DynamicIndex, FactoryView};
use relm4::{view, AsyncFactorySender};
use std::str::FromStr;

use crate::gtk::prelude::{
	ButtonExt, EditableExt, EntryBufferExtManual, EntryExt, WidgetExt,
};
use crate::widgets::factory::provider::ProviderInput;
use crate::application::plugin::Plugin;
use proto_rust::provider::List;
use relm4::loading_widgets::LoadingWidgets;

use crate::{adw, gtk};

#[derive(Debug)]
pub enum ListInput {
	Select,
	Delete(DynamicIndex),
	Rename(String),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum ListOutput {
    Select(ListData),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	Notify(String),
}

#[derive(Debug, Clone)]
pub struct ListData {
	pub data: List,
    pub tasks: Vec<String>
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ListData {
	type ParentInput = ProviderInput;
	type ParentWidget = adw::ExpanderRow;
	type CommandOutput = ();
	type Input = ListInput;
	type Output = ListOutput;
    type Init = List;
	type Widgets = ListWidgets;

	view! {
		#[root]
		gtk::ListBoxRow {
			adw::ActionRow {
				add_prefix = &gtk::Entry {
					set_hexpand: false,
					add_css_class: "flat",
					add_css_class: "no-border",
					#[watch]
					set_text: self.data.name.as_str(),
					set_margin_top: 10,
					set_margin_bottom: 10,
					connect_activate[sender] => move |entry| {
						let buffer = entry.buffer();
						sender.input(ListInput::Rename(buffer.text()));
					},
					// This crashes the program.
					// connect_changed[sender] => move |entry| {
					// 	let buffer = entry.buffer();
					// 	sender.input(ListInput::Rename(buffer.text()));
					// }
				},
				add_prefix = &gtk::MenuButton {
					#[watch]
					set_label: self.data.icon.as_ref().unwrap().as_str(),
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					#[wrap(Some)]
					set_popover = &gtk::EmojiChooser{
						connect_emoji_picked[sender] => move |_, emoji| {
							sender.input(ListInput::ChangeIcon(emoji.to_string()))
						}
					}
				},
				add_suffix = &gtk::Label {
					set_halign: gtk::Align::End,
					set_css_classes: &["dim-label", "caption"],
					// #[watch]
					// set_text: self.count.to_string().as_str(),
				},
				add_suffix = &gtk::Button {
					set_icon_name: "user-trash-full-symbolic",
					set_css_classes: &["circular", "image-button", "destructive-action"],
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
						sender.input(ListInput::Delete(index.clone()))
					}
				},
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input(ListInput::Select);
					sender.output(ListOutput::Forward)
				}
			}
		}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		view! {
			#[local_ref]
			root {
				#[name(spinner)]
				adw::ActionRow {
					add_prefix = &gtk::Spinner {
						start: (),
						set_hexpand: false,
					}
				}
			}
		}
		Some(LoadingWidgets::new(root, spinner))
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	async fn init_model(
		params: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
        let mut tasks = vec![];
        if let Ok(provider) = Plugin::from_str(&params.provider) {
            match provider.connect().await {
                Ok(mut service) => match service.read_task_ids_from_list(params.id.clone()).await {
                    Ok(response) => tasks = response.into_inner().tasks,
                    Err(e) => error!("Failed to find tasks. {:?}", e)
                },
                Err(e) => error!("Failed to connect to service. {:?}", e)
            }
        };
        Self { data: params, tasks }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		if let Ok(provider) = Plugin::from_str(&self.data.provider) {
			match message {
				ListInput::Rename(name) => {
					let mut list = self.data.clone();
					list.name = name.clone();
					match provider.connect().await {
						Ok(mut service) => match service.update_list(list).await {
							Ok(response) => {
								let response = response.into_inner();
								if response.successful {
									self.data.name = name;
								}
								sender.output(ListOutput::Notify(response.message))
							},
							Err(err) => sender.output(ListOutput::Notify(err.to_string())),
						},
						Err(err) => sender.output(ListOutput::Notify(err.to_string())),
					}
				},
				ListInput::Delete(index) => match provider.connect().await {
					Ok(mut service) => {
						match service.delete_list(self.clone().data.id).await {
							Ok(response) => {
								let response = response.into_inner();
								if response.successful {
									sender.output(ListOutput::DeleteTaskList(
										index,
										self.data.id.clone(),
									));
								}
								sender.output(ListOutput::Notify(response.message))
							},
							Err(err) => sender.output(ListOutput::Notify(err.to_string())),
						}
					},
					Err(err) => sender.output(ListOutput::Notify(err.to_string())),
				},
				ListInput::ChangeIcon(icon) => {
					let mut list = self.data.clone();
					list.icon = Some(icon.clone());
					match provider.connect().await {
						Ok(mut service) => match service.update_list(list).await {
							Ok(response) => {
								let response = response.into_inner();
								if response.successful {
									self.data.icon = Some(icon);
								}
								sender.output(ListOutput::Notify(response.message))
							},
							Err(err) => sender.output(ListOutput::Notify(err.to_string())),
						},
						Err(err) => sender.output(ListOutput::Notify(err.to_string())),
					}
				},
				ListInput::Select => {
					sender.output(ListOutput::Select(self.clone()))
				},
			}
		} else if let ListInput::Select = message {
			sender.output(ListOutput::Select(self.clone()))
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
            ListOutput::Select(data) => Some(ProviderInput::ListSelected(data)),
			ListOutput::DeleteTaskList(index, list_id) => {
				Some(ProviderInput::DeleteTaskList(index, list_id))
			},
			ListOutput::Forward => Some(ProviderInput::Forward),
			ListOutput::Notify(msg) => Some(ProviderInput::Notify(msg)),
		}
	}
}
