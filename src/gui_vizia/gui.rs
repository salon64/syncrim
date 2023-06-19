use crate::common::{ComponentStore, SimState, Simulator};
use crate::gui_vizia::grid::GridView;
use crate::gui_vizia::menu::Menu;
use std::rc::Rc;
use vizia::{
    icons,
    prelude::*,
    //vg::{Paint, Path},
};

#[derive(Lens)]
pub struct Gui {
    pub simulator: Rc<Simulator>,
    pub state: SimState,
    // History, acts like a stack
    pub history: Vec<Vec<u32>>,
    pub pause: bool,
    pub is_saved: bool,
    pub show_about: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum GuiEvent {
    Clock,
    Reset,
    UnClock,
    Play,
    Pause,
    PlayToggle,
    Preferences,
    ShowAbout,
    HideAbout,
}

impl Model for Gui {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            WindowEvent::WindowClose => {
                if !self.is_saved {
                    // self.show_dialog = true;
                    meta.consume();
                    self.is_saved = true;
                }
            }
            _ => {}
        });

        event.map(|app_event, _meta| match app_event {
            GuiEvent::Clock => {
                // push current state
                self.history.push(self.state.lens_values.clone());
                self.simulator.clock(&mut self.state);
            }
            GuiEvent::Reset => {
                self.simulator.reset(&mut self.state);
                // clear history
                self.history = vec![];
                // make sure its in paused mode
                self.pause = true;
            }
            GuiEvent::UnClock => {
                if let Some(state) = self.history.pop() {
                    // set old state
                    self.state.lens_values = state;
                }
            }
            GuiEvent::Play => self.pause = false,
            GuiEvent::Pause => self.pause = true,
            GuiEvent::PlayToggle => self.pause = !self.pause,
            GuiEvent::Preferences => println!("Preferences"),
            GuiEvent::ShowAbout => self.show_about = true,
            GuiEvent::HideAbout => self.show_about = false,
        });
    }
}

const STYLE: &str = r#"
    .tt_shortcut {
        color: #c4c4c4;
    }

    submenu.file_menu > popup {
        width: 200px;
    }
"#;

pub fn gui(cs: &ComponentStore) {
    let (simulator, mut sim_state) = Simulator::new(cs);
    let simulator = Rc::new(simulator);
    // Initial clock to propagate constants
    simulator.clock(&mut sim_state);

    Application::new(move |cx| {
        // Styling
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // Create keymap
        crate::gui_vizia::keymap::new(cx);

        Gui {
            simulator: simulator.clone(),
            state: sim_state,
            history: vec![],
            pause: true,
            is_saved: false,
            show_about: false,
        }
        .build(cx);

        Menu::new(cx).background_color(Color::beige()).size(Auto);

        // Grid
        GridView::new(cx, |cx| {
            for c in &simulator.ordered_components {
                c.view(cx, simulator.clone());
            }
        })
        .top(Stretch(1.0))
        .bottom(Stretch(1.0));

        // a label to display the raw state for debugging purpose
        Label::new(
            cx,
            Gui::state
                .then(SimState::lens_values)
                .map(|v| format!("Raw state {:?}", v)),
        );

        HStack::new(cx, |cx| {
            // Reset
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Reset),
                |cx| Label::new(cx, icons::ICON_PLAYER_SKIP_BACK),
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Reset");
                    Label::new(cx, " Shift + Ctrl + F5").class("tt_shortcut");
                })
                .size(Auto);
            });

            // UnClock (step back)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::UnClock),
                |cx| Label::new(cx, icons::ICON_CHEVRON_LEFT),
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "UnClock");
                    Label::new(cx, " Shift + F10").class("tt_shortcut");
                })
                .size(Auto);
            });

            // Clock (step forward)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Clock),
                |cx| Label::new(cx, icons::ICON_CHEVRON_RIGHT),
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Clock");
                    Label::new(cx, " F10").class("tt_shortcut");
                })
                .size(Auto);
            });

            // Play (continuous mode)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Play),
                |cx| {
                    Label::new(
                        cx,
                        Gui::pause.map(|pause| {
                            if *pause {
                                icons::ICON_PLAYER_PLAY
                            } else {
                                icons::ICON_PLAYER_PLAY_FILLED
                            }
                        }),
                    )
                },
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Play");
                    Label::new(cx, " F5 (Toggle)").class("tt_shortcut");
                })
                .size(Auto);
            });

            // Pause (step mode)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Pause),
                |cx| {
                    Label::new(
                        cx,
                        Gui::pause.map(|pause| {
                            if *pause {
                                icons::ICON_PLAYER_PAUSE_FILLED
                            } else {
                                icons::ICON_PLAYER_PAUSE
                            }
                        }),
                    )
                },
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Pause");
                    Label::new(cx, " F5 (Toggle)").class("tt_shortcut");
                })
                .size(Auto);
            });
            Popup::new(cx, Gui::show_about, true, |cx| {
                Label::new(cx, "Modal Title").class("title");
                Label::new(cx, "This is a message");
                Button::new(
                    cx,
                    |cx| cx.emit(GuiEvent::HideAbout),
                    |cx| Label::new(cx, "Ok"),
                )
                .class("accent");
            })
            // .on_blur(|cx| cx.emit(GuiEvent::HideAbout))
            .class("modal");
        });
    })
    .title("SyncRim")
    .run();
}