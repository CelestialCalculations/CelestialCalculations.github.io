extern crate orbtk;

use orbtk::prelude::*;
use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
enum Action {
    ClearText,
    EntryActivated(Entity),
    EntryChanged(Entity),
    ValueChanged(Entity),
}

#[derive(Default)]
pub struct MainViewState {
    action: Cell<Option<Action>>,
}

impl MainViewState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for MainViewState {
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::ClearText => {
                    ctx.widget().set("text_input", String16::from(""));
                    ctx.widget().set("text_result", String16::from(""));
                }
                Action::EntryChanged(entity) => {
                    let widget = ctx.get_widget(entity);
                    let text = widget.get::<String16>("text");
                    println!("entry changed: {}", text);
                }
                _ => {
                    println!("unsupported action");
                }
            }

            self.action.set(None);
        }
    }
}

fn create_header(ctx: &mut BuildContext, text: &str) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(Selector::new().with("text-block").class("h1"))
        .build(ctx)
}

widget!(
    MainView<MainViewState> {
        text_input: String16,
        text_result: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("Chapter.2 - Unit conversion")
            .text_input("0.0")
            .text_result("Result: 0")
            .child(
                Grid::create()
                    .margin(10.0)
                    .columns(Columns::create().column(150.0).column(150.0).build())
                    .child(
                        Stack::create()
                            .attach(Grid::column(0))
                            .child(create_header(ctx, "From unit"))
                            .child(
                                TextBox::create()
                                    .water_mark("From value")
                                    .text(("text_input", id))
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .text("Clear")
                                    .on_click(move |_| {
                                        state.action(Action::ClearText);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Stack::create()
                            .attach(Grid::column(1))
                            .child(create_header(ctx, "To unit"))
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Chapter.2 - unit conversion")
                .position((100.0, 100.0))
                .size(450.0, 450.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run()
}
