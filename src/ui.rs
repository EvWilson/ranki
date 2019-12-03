use super::collection;

use orbtk::prelude::*;

struct UISettings {
    width: i32,
    height: i32,
}

const SETTINGS: UISettings = UISettings {
    width: 640,
    height: 480,
};

#[derive(Debug, Copy, Clone)]
enum Action {
    ChooseDeck(Entity),
}

#[derive(Default)]
pub struct MainViewState {
    action: Option<Action>,
}

impl State for MainViewState {
    fn update(&self, ctx: &mut Context) {

    }
}

type StringVec = Vec<String>;

widget!(MainView<MainViewState> {
    text: String,
    stuff: StringVec
});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .width(SETTINGS.width as f64)
            .height(SETTINGS.height as f64)
            .child(
                Grid::create()
                    .rows(Rows::create().row(72.0).row("*").build())
                    .rows(Rows::create().row(100.0).row("|").build())
                    .child(
                        Container::create()
                            .attach(Grid::row(0))
                            .child(
                                TextBlock::create()
                                    .width(10.0)
                                    .height(10.0)
                                    .text("Hi")
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        ScrollViewer::create()
                            .attach(Grid::row(1))
                            .child(
                                ListView::create()
                                    .items_builder(
                                        move |bc, index| {
                                            let text = "BLAH";

                                            TextBlock::create()
                                                .text(text)
                                                .build(bc)
                                        }
                                    )
                                    .build(ctx)
                            )
                            .build(ctx),
                    )
                    .selector(Selector::from("container").class("title"))
                    .build(ctx)
            )
    }
}

pub fn render(collection: collection::Collection) {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((200.0, 100.0))
                .size(SETTINGS.width as f64, SETTINGS.height as f64)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
