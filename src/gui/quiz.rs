use gtk::*;

use relm::{Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::window;

pub struct QuizModel {
    active_quiz: Option<Component<StartedQuiz>>,
    parent_stream: EventStream<window::Msg>,
    stream: EventStream<QuizMsg>,
}

#[derive(Msg)]
pub enum QuizMsg {
    StartQuiz,
    GotQuiz(Vec<(u32, u32, String, String)>),
    QuizComplete(Vec<(u32, u32, Option<bool>)>),
}

#[widget]
impl Widget for QuizView {
    fn model(relm: &Relm<Self>, parent_stream: EventStream<window::Msg>) -> QuizModel {
        QuizModel {
            active_quiz: None,
            parent_stream,
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: QuizMsg) {
        use QuizMsg::*;

        match event {
            StartQuiz => {
                self.model.parent_stream.emit(window::Msg::StartQuiz);
            }
            GotQuiz(quiz) => {
                // Don't start a new quiz if we already have one active
                if let Some(_) = self.model.active_quiz {
                    return;
                }
                let widget = self
                    .quiz_box
                    .add_widget::<StartedQuiz>((self.model.stream.clone(), quiz));
                self.model.active_quiz = Some(widget);
            }
            QuizComplete(results) => {
                self.model
                    .parent_stream
                    .emit(window::Msg::QuizComplete(results));
                if let Some(widget) = self.model.active_quiz.take() {
                    self.quiz_box.remove_widget(widget);
                }
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            halign: gtk::Align::Center,
            gtk::Button {
                label: "Quiz Me!",
                clicked => QuizMsg::StartQuiz,
            },
            #[name="quiz_box"]
            gtk::Box {}
        }
    }
}

pub struct StartedQuizModel {
    active_answer: String,
    parent_stream: EventStream<QuizMsg>,
    question_idx: usize,
    quiz_vec: Vec<(u32, u32, String, String, Option<bool>)>,
}

#[derive(Msg)]
pub enum StartedQuizMsg {
    Correct,
    Incorrect,
    Reveal,
    Stop,
}

#[widget]
impl Widget for StartedQuiz {
    fn model(
        (parent_stream, quiz): (EventStream<QuizMsg>, Vec<(u32, u32, String, String)>),
    ) -> StartedQuizModel {
        let quiz_vec: Vec<(u32, u32, String, String, Option<bool>)> = quiz
            .into_iter()
            .map(|question| (question.0, question.1, question.2, question.3, None))
            .collect();
        StartedQuizModel {
            active_answer: "...".to_string(),
            parent_stream,
            question_idx: 0,
            quiz_vec,
        }
    }

    fn update(&mut self, event: StartedQuizMsg) {
        use StartedQuizMsg::*;

        self.model.active_answer = "...".to_string();
        match event {
            Correct => {
                self.model.quiz_vec[self.model.question_idx].4 = Some(true);
                if self.model.question_idx == (self.model.quiz_vec.len() - 1) {
                    self.report_results();
                } else {
                    self.model.question_idx += 1;
                }
            }
            Incorrect => {
                self.model.quiz_vec[self.model.question_idx].4 = Some(false);
                if self.model.question_idx == (self.model.quiz_vec.len() - 1) {
                    self.report_results();
                } else {
                    self.model.question_idx += 1;
                }
            }
            Reveal => {
                self.model.active_answer = self.model.quiz_vec[self.model.question_idx].3.clone();
            }
            Stop => {
                self.report_results();
            }
        }
    }

    fn report_results(&self) {
        let results = self
            .model
            .quiz_vec
            .iter()
            .map(|question| (question.0, question.1, question.4))
            .collect();
        self.model
            .parent_stream
            .emit(QuizMsg::QuizComplete(results));
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Active Quiz",
            },
            gtk::Label {
                label: &self.model.quiz_vec[self.model.question_idx].2,
            },
            gtk::Label {
                label: &self.model.active_answer,
            },
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                gtk::Button {
                    label: "Recalled",
                    clicked => StartedQuizMsg::Correct,
                },
                gtk::Button {
                    label: "Forgot",
                    clicked => StartedQuizMsg::Incorrect,
                },
                gtk::Button {
                    label: "Reveal Answer",
                    clicked => StartedQuizMsg::Reveal,
                },
                gtk::Button {
                    label: "Stop",
                    clicked => StartedQuizMsg::Stop,
                },
            },
        }
    }
}
