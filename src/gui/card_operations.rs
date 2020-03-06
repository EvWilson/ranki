use gtk::*;

use relm::{Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::card_view::CardViewMsg;

enum ModalState {
    Add(Component<AddCardWidget>),
    Delete(Component<DeleteCardWidget>),
    Edit(Component<EditCardWidget>),
}

pub struct CardOpModel {
    modal_state: Option<ModalState>,
    parent_stream: EventStream<CardViewMsg>,
    stream: EventStream<CardOpMsg>,
}

#[derive(Msg)]
pub enum CardOpMsg {
    Add(String, String),
    AddModal,
    Delete,
    DeleteModal,
    Edit(String, String),
    EditModal,
    Cancel,
}

#[widget]
impl Widget for CardOpGrid {
    fn model(relm: &Relm<Self>, parent_stream: EventStream<CardViewMsg>) -> CardOpModel {
        CardOpModel {
            modal_state: None,
            parent_stream,
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: CardOpMsg) {
        self.remove_modal();

        match event {
            CardOpMsg::Add(question, answer) => {
                self.model
                    .parent_stream
                    .emit(CardViewMsg::Add(question, answer));
            }
            CardOpMsg::AddModal => {
                let widget = self
                    .mod_box
                    .add_widget::<AddCardWidget>(self.model.stream.clone());
                self.model.modal_state = Some(ModalState::Add(widget));
            }
            CardOpMsg::Cancel => {
                self.model.modal_state = None;
                self.model.parent_stream.emit(CardViewMsg::Cleared);
            }
            CardOpMsg::Delete => {
                self.model.parent_stream.emit(CardViewMsg::Delete);
            }
            CardOpMsg::DeleteModal => {
                let widget = self
                    .mod_box
                    .add_widget::<DeleteCardWidget>(self.model.stream.clone());
                self.model.modal_state = Some(ModalState::Delete(widget));
            }
            CardOpMsg::Edit(question, answer) => {
                self.model
                    .parent_stream
                    .emit(CardViewMsg::Edit(question, answer));
            }
            CardOpMsg::EditModal => {
                let widget = self
                    .mod_box
                    .add_widget::<EditCardWidget>(self.model.stream.clone());
                self.model.modal_state = Some(ModalState::Edit(widget));
            }
        }
    }

    fn remove_modal(&mut self) {
        use ModalState::*;

        // Remove old modal state, if applicable
        match &self.model.modal_state {
            None => {}
            Some(Add(widget)) => self.mod_box.remove_widget(widget.clone()),
            Some(Delete(widget)) => self.mod_box.remove_widget(widget.clone()),
            Some(Edit(widget)) => self.mod_box.remove_widget(widget.clone()),
        }
    }

    view! {
        gtk::Grid {
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
                    clicked => CardOpMsg::AddModal,
                },
                gtk::Button {
                    label: "Edit",
                    cell: {
                        left_attach: 1,
                        top_attach: 0,
                    },
                    clicked => CardOpMsg::EditModal,
                },
                gtk::Button {
                    label: "Delete",
                    cell: {
                        left_attach: 0,
                        top_attach: 1,
                    },
                    clicked => CardOpMsg::DeleteModal,
                },
                gtk::Button {
                    label: "Cancel",
                    cell: {
                        left_attach: 1,
                        top_attach: 1,
                    },
                    clicked => CardOpMsg::Cancel,
                }
            },
            #[name="mod_box"]
            gtk::Box {}
        }
    }
}

pub struct AddCardModel {
    question: String,
    answer: String,
    parent_stream: EventStream<CardOpMsg>,
}

#[derive(Msg)]
pub enum AddCardMsg {
    Add,
    QChange,
    AChange,
}

#[widget]
impl Widget for AddCardWidget {
    fn model(parent_stream: EventStream<CardOpMsg>) -> AddCardModel {
        AddCardModel {
            question: "".to_string(),
            answer: "".to_string(),
            parent_stream,
        }
    }

    fn update(&mut self, event: AddCardMsg) {
        match event {
            AddCardMsg::Add => {
                // Don't add a nameless card
                if self.model.question.is_empty() || self.model.answer.is_empty() {
                    return;
                }
                self.model.parent_stream.emit(CardOpMsg::Add(
                    self.model.question.clone(),
                    self.model.answer.clone(),
                ));
                // Clear entry fields
                self.question.set_text("");
                self.answer.set_text("");
            }
            AddCardMsg::QChange => {
                let text = match self.question.get_text() {
                    None => {
                        println!("error: failed to get text on add card question change");
                        return;
                    }
                    Some(string) => string,
                };
                self.model.question = text.chars().collect();
            }
            AddCardMsg::AChange => {
                let text = match self.answer.get_text() {
                    None => {
                        println!("error: failed to get text on add card answer change");
                        return;
                    }
                    Some(string) => string,
                };
                self.model.answer = text.chars().collect();
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Add Card",
            },
            #[name="question"]
            gtk::Entry {
                changed => AddCardMsg::QChange,
            },
            #[name="answer"]
            gtk::Entry {
                changed => AddCardMsg::AChange,
            },
            gtk::Button {
                label: "Add",
                clicked => AddCardMsg::Add,
            }
        }
    }
}

pub struct DeleteCardModel {
    parent_stream: EventStream<CardOpMsg>,
}

#[derive(Msg)]
pub enum DeleteCardMsg {
    Delete,
}

#[widget]
impl Widget for DeleteCardWidget {
    fn model(parent_stream: EventStream<CardOpMsg>) -> DeleteCardModel {
        DeleteCardModel { parent_stream }
    }

    fn update(&mut self, event: DeleteCardMsg) {
        match event {
            DeleteCardMsg::Delete => {
                self.model.parent_stream.emit(CardOpMsg::Delete);
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Delete Card",
                widget_name: "delete_card_label",
            },
            gtk::Button {
                label: "Delete Selected Card (for real)",
                widget_name: "delete_card_button",
                clicked => DeleteCardMsg::Delete,
            }
        }
    }
}

pub struct EditCardModel {
    question: String,
    answer: String,
    parent_stream: EventStream<CardOpMsg>,
}

#[derive(Msg)]
pub enum EditCardMsg {
    Edit,
    QChange,
    AChange,
}

#[widget]
impl Widget for EditCardWidget {
    fn model(parent_stream: EventStream<CardOpMsg>) -> EditCardModel {
        EditCardModel {
            question: "".to_string(),
            answer: "".to_string(),
            parent_stream,
        }
    }

    fn update(&mut self, event: EditCardMsg) {
        match event {
            EditCardMsg::Edit => {
                // Don't edit card to nothing
                if self.model.question.is_empty() || self.model.answer.is_empty() {
                    return;
                }
                self.model.parent_stream.emit(CardOpMsg::Edit(
                    self.model.question.clone(),
                    self.model.answer.clone(),
                ));
                // Clear entry fields
                self.question.set_text("");
                self.answer.set_text("");
            }
            EditCardMsg::QChange => {
                let text = match self.question.get_text() {
                    None => {
                        println!("error: failed to get text on edit card question change");
                        return;
                    }
                    Some(string) => string,
                };
                self.model.question = text.chars().collect();
            }
            EditCardMsg::AChange => {
                let text = match self.answer.get_text() {
                    None => {
                        println!("error: failed to get text on edit card answer change");
                        return;
                    }
                    Some(string) => string,
                };
                self.model.answer = text.chars().collect();
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Edit Card",
            },
            #[name="question"]
            gtk::Entry {
                changed => EditCardMsg::QChange,
            },
            #[name="answer"]
            gtk::Entry {
                changed => EditCardMsg::AChange,
            },
            gtk::Button {
                label: "Edit",
                clicked => EditCardMsg::Edit,
            }
        }
    }
}
