use gtk::*;

use relm::{Component, ContainerWidget, Widget};
use relm_derive::{widget, Msg};

use super::window::CollectionSender;
use crate::collection::Action;

enum ModalState {
    Empty,
    Add(Component<AddDeckWidget>),
    Delete(Component<DeleteDeckWidget>),
    Edit(Component<EditDeckWidget>),
}

// ----- Operations Grid Section -----
// Lists operations available to perform on the collection, click buttons to
// open the appropriate modals
pub struct OpGridModel {
    modal_state: ModalState,
    tx: CollectionSender,
}

#[derive(Msg)]
pub enum OpGridMsg {
    Add,
    Edit,
    Delete,
}

#[widget]
impl Widget for OpGrid {
    fn model(tx: CollectionSender) -> OpGridModel {
        OpGridModel {
            modal_state: ModalState::Empty,
            tx,
        }
    }

    fn update(&mut self, event: OpGridMsg) {
        match (&event, &self.model.modal_state) {
            // If we're updating to the same state as we just were, don't add a new box
            (OpGridMsg::Add, ModalState::Add(_))
            | (OpGridMsg::Delete, ModalState::Delete(_))
            | (OpGridMsg::Edit, ModalState::Edit(_)) => {
                println!("Setting modal state to same val, aborting");
                return;
            }
            _ => {
                println!("Modal state being updated, continuing");
            }
        }

        // Remove old modal state, if applicable
        if let ModalState::Add(widget) = &self.model.modal_state {
            self.mod_box.remove_widget(widget.clone());
        } else if let ModalState::Delete(widget) = &self.model.modal_state {
            self.mod_box.remove_widget(widget.clone());
        } else if let ModalState::Edit(widget) = &self.model.modal_state {
            self.mod_box.remove_widget(widget.clone());
        }

        match event {
            OpGridMsg::Add => {
                println!("op grid add event");
                let widget = self
                    .mod_box
                    .add_widget::<AddDeckWidget>(self.model.tx.clone());
                self.model.modal_state = ModalState::Add(widget);
            }
            OpGridMsg::Edit => {
                println!("op grid edit event");
                let widget = self.mod_box.add_widget::<EditDeckWidget>(());
                self.model.modal_state = ModalState::Edit(widget);
            }
            OpGridMsg::Delete => {
                println!("op grid delete event");
                let widget = self.mod_box.add_widget::<DeleteDeckWidget>(());
                self.model.modal_state = ModalState::Delete(widget);
            }
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
                    clicked => OpGridMsg::Add,
                },
                gtk::Button {
                    label: "Edit",
                    cell: {
                        left_attach: 1,
                        top_attach: 0,
                    },
                    clicked => OpGridMsg::Edit,
                },
                gtk::Button {
                    label: "Delete",
                    cell: {
                        left_attach: 0,
                        top_attach: 1,
                    },
                    clicked => OpGridMsg::Delete,
                },
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
    tx: CollectionSender,
}

#[derive(Msg)]
pub enum AddDeckMsg {
    Add,
    Change,
}

#[widget]
impl Widget for AddDeckWidget {
    fn model(tx: CollectionSender) -> AddDeckModel {
        AddDeckModel {
            text: "".to_string(),
            tx,
        }
    }

    fn update(&mut self, event: AddDeckMsg) {
        match event {
            AddDeckMsg::Add => {
                // Don't add a nameless deck
                if self.model.text == "" {
                    return;
                }
                if let Err(e) = self.model.tx.send(Action::AddDeck(self.model.text.clone())) {
                    println!("error sending add deck msg: {}", e);
                }
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
pub struct DeleteDeckModel {}

#[derive(Msg)]
pub enum DeleteDeckMsg {
    Delete,
}

#[widget]
impl Widget for DeleteDeckWidget {
    fn model() -> DeleteDeckModel {
        DeleteDeckModel {}
    }

    fn update(&mut self, event: DeleteDeckMsg) {
        match event {
            DeleteDeckMsg::Delete => {
                println!("delete msg callback in delete deck modal");
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
                label: "Delete",
                widget_name: "delete_deck_button",
                clicked => DeleteDeckMsg::Delete,
            }
        }
    }
}

// ----- Edit Deck Modal Widget -----
// Edit deck modal widget
pub struct EditDeckModel {}

#[derive(Msg)]
pub enum EditDeckMsg {
    Edit,
}

#[widget]
impl Widget for EditDeckWidget {
    fn model() -> EditDeckModel {
        EditDeckModel {}
    }

    fn update(&mut self, event: EditDeckMsg) {
        match event {
            EditDeckMsg::Edit => {
                println!("edit msg callback in edit deck modal");
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Edit Deck",
                widget_name: "delete_deck_label",
            },
            gtk::Button {
                label: "Edit",
                widget_name: "edit_deck_button",
                clicked => EditDeckMsg::Edit,
            }
        }
    }
}
