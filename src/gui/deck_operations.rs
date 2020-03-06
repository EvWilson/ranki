use gtk::*;

use relm::{Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::deck_view::DeckViewMsg;

// I wish that ModalState(below) could be written like the below snippet, so that
// I could more easily reduce the combinatorial checking below. It seems that
// other people also wish this were the case, as this kind of operation is being
// actively considered for development:
// https://github.com/rust-lang/rust/issues/52662
/*
enum ModalState {
    Empty,
    Full(Component<W: Widget>),
}
*/

enum ModalState {
    Add(Component<AddDeckWidget>),
    Delete(Component<DeleteDeckWidget>),
    Edit(Component<EditDeckWidget>),
}

// ----- Operations Grid Section -----
// Lists operations available to perform on the collection, click buttons to
// open the appropriate modals
pub struct DeckOpModel {
    //modal_state: ModalState,
    modal_state: Option<ModalState>,
    parent_stream: EventStream<DeckViewMsg>,
    stream: EventStream<DeckOpMsg>,
}

#[derive(Msg)]
pub enum DeckOpMsg {
    Add(String),
    AddModal,
    Cancel,
    Delete,
    DeleteModal,
    Edit(String),
    EditModal,
}

#[widget]
impl Widget for DeckOpGrid {
    fn model(relm: &Relm<Self>, parent_stream: EventStream<DeckViewMsg>) -> DeckOpModel {
        DeckOpModel {
            modal_state: None,
            parent_stream,
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: DeckOpMsg) {
        self.remove_modal();

        match event {
            DeckOpMsg::AddModal => {
                let widget = self
                    .mod_box
                    .add_widget::<AddDeckWidget>(self.model.stream.clone());
                self.model.modal_state = Some(ModalState::Add(widget));
            }
            DeckOpMsg::Cancel => {
                self.model.modal_state = None;
                self.model.parent_stream.emit(DeckViewMsg::Cleared);
            }
            DeckOpMsg::Delete => {
                self.model.parent_stream.emit(DeckViewMsg::Delete);
            }
            DeckOpMsg::Add(name) => {
                self.model.parent_stream.emit(DeckViewMsg::Add(name));
            }
            DeckOpMsg::DeleteModal => {
                let widget = self
                    .mod_box
                    .add_widget::<DeleteDeckWidget>(self.model.stream.clone());
                self.model.modal_state = Some(ModalState::Delete(widget));
            }
            DeckOpMsg::Edit(name) => {
                self.model.parent_stream.emit(DeckViewMsg::Edit(name));
            }
            DeckOpMsg::EditModal => {
                let widget = self
                    .mod_box
                    .add_widget::<EditDeckWidget>(self.model.stream.clone());
                self.model.modal_state = Some(ModalState::Edit(widget));
            }
        }
    }

    fn remove_modal(&mut self) {
        use ModalState::*;

        // Remove old modal state, if applicable
        match &self.model.modal_state {
            None => return,
            Some(Add(widget)) => self.mod_box.remove_widget(widget.clone()),
            Some(Delete(widget)) => self.mod_box.remove_widget(widget.clone()),
            Some(Edit(widget)) => self.mod_box.remove_widget(widget.clone()),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Operations",
                widget_name: "op_label",
            },
            gtk::Grid {
                row_homogeneous: true,
                column_homogeneous: true,
                gtk::Button {
                    label: "Add",
                    cell: {
                        left_attach: 0,
                        top_attach: 0,
                    },
                    clicked => DeckOpMsg::AddModal,
                },
                gtk::Button {
                    label: "Edit",
                    cell: {
                        left_attach: 1,
                        top_attach: 0,
                    },
                    clicked => DeckOpMsg::EditModal,
                },
                gtk::Button {
                    label: "Delete",
                    cell: {
                        left_attach: 0,
                        top_attach: 1,
                    },
                    clicked => DeckOpMsg::DeleteModal,
                },
                gtk::Button {
                    label: "Cancel",
                    cell: {
                        left_attach: 1,
                        top_attach: 1,
                    },
                    clicked => DeckOpMsg::Cancel,
                }
            },
            #[name="mod_box"]
            gtk::Box {}
        }
    }
}

// ----- Add Deck Modal Widget -----
// Widget opened when the user enters the add deck state (clicks add deck btn)
pub struct AddDeckModel {
    text: String,
    parent_stream: EventStream<DeckOpMsg>,
}

#[derive(Msg)]
pub enum AddDeckMsg {
    Add,
    Change,
}

#[widget]
impl Widget for AddDeckWidget {
    fn model(parent_stream: EventStream<DeckOpMsg>) -> AddDeckModel {
        AddDeckModel {
            text: "".to_string(),
            parent_stream,
        }
    }

    fn update(&mut self, event: AddDeckMsg) {
        match event {
            AddDeckMsg::Add => {
                // Don't add a nameless deck
                if self.model.text == "" {
                    return;
                }
                self.model
                    .parent_stream
                    .emit(DeckOpMsg::Add(self.model.text.clone()));
                // Clear entry field
                self.entry.set_text("");
            }
            AddDeckMsg::Change => {
                let text = match self.entry.get_text() {
                    None => {
                        println!("error: failed to get text on add deck field change");
                        return;
                    }
                    Some(string) => string,
                };
                self.model.text = text.chars().collect();
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Add Deck",
            },
            #[name="entry"]
            gtk::Entry {
                changed => AddDeckMsg::Change,
            },
            gtk::Button {
                label: "Add",
                clicked => AddDeckMsg::Add,
            }
        }
    }
}

// ----- Delete Deck Modal Widget -----
// Delete deck state widget
pub struct DeleteDeckModel {
    parent_stream: EventStream<DeckOpMsg>,
}

#[derive(Msg)]
pub enum DeleteDeckMsg {
    Delete,
}

#[widget]
impl Widget for DeleteDeckWidget {
    fn model(parent_stream: EventStream<DeckOpMsg>) -> DeleteDeckModel {
        DeleteDeckModel { parent_stream }
    }

    fn update(&mut self, event: DeleteDeckMsg) {
        match event {
            DeleteDeckMsg::Delete => {
                self.model.parent_stream.emit(DeckOpMsg::Delete);
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Delete Deck",
                widget_name: "delete_deck_label",
            },
            gtk::Button {
                label: "Delete Selected Deck (for real)",
                widget_name: "delete_deck_button",
                clicked => DeleteDeckMsg::Delete,
            }
        }
    }
}

// ----- Edit Deck Modal Widget -----
// Edit deck modal widget
pub struct EditDeckModel {
    title: String,
    parent_stream: EventStream<DeckOpMsg>,
}

#[derive(Msg)]
pub enum EditDeckMsg {
    Change,
    Edit,
}

#[widget]
impl Widget for EditDeckWidget {
    fn model(parent_stream: EventStream<DeckOpMsg>) -> EditDeckModel {
        EditDeckModel {
            parent_stream,
            title: "".to_string(),
        }
    }

    fn update(&mut self, event: EditDeckMsg) {
        match event {
            EditDeckMsg::Change => {
                let text = match self.entry.get_text() {
                    None => {
                        println!("error: failed to get text on edit deck field change");
                        return;
                    }
                    Some(string) => string,
                };
                self.model.title = text.chars().collect();
            }
            EditDeckMsg::Edit => {
                self.model
                    .parent_stream
                    .emit(DeckOpMsg::Edit(self.model.title.clone()));
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "New Deck Name:",
                widget_name: "edit_deck_label",
            },
            #[name="entry"]
            gtk::Entry {
                changed => EditDeckMsg::Change,
            },
            gtk::Button {
                label: "Edit",
                widget_name: "edit_deck_button",
                clicked => EditDeckMsg::Edit,
            }
        }
    }
}
