extern crate iced;
use iced::{
    button, keyboard, pane_grid, scrollable, Align, Button, Column, Container,
    Element, HorizontalAlignment, Length, PaneGrid, Sandbox, Scrollable,
    Settings, Text,
};

pub fn main() {
    AnimeGUI::run(Settings::default())
}

enum GUI_State {
    Downloading,
    Browsing
}

#[derive(Default)]
struct AnimeGUI {
    gui_state: GUI_State::Downloading,
    panes: pane_grid::State<Content>
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SwitchToBrowse,
    SwitchToDownload,
    StartSearch,
    Resized(pane_grid::ResizeEvent)
}

impl Sandbox for AnimeGUI {
    type Message = Message;

    fn new() -> Self {
        let (gui_panes, _) = pane_grid::State::new(Content::new(0));
        AnimeGUI {
            gui_state: GUI_State::Downloading,
            panes: gui_panes
        }
    }

    fn title(&self) -> String {
        String::from("AnimeGUI - XDCC Anime Client")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchToBrowse => {
                self.gui_state = GUI_State::Browsing;
            },
            Message::SwitchToDownload => {
                self.gui_state = GUI_State::Downloading;
            },
            Message::StartSearch => {

            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self.gui_state {
            GUI_State::Downloading =>
                Column::new()
                    .padding(50)
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.increment_button, Text::new("Switch to Browsing"))
                            .on_press(Message::IncrementPressed),
                    )
                    .push(Text::new("Downloading").size(20))
                    .into()
            GUI_State::Browsing =>
                Column::new()
                    .spacing(20)
                    .push(Text::new("Browsing").size(20))
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
