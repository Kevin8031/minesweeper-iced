use iced::{Sandbox, Text, Column, Button, button, Container, Row, Element};
use minesweeper::{Game, GameOpts};

#[derive(Debug, Clone)]
enum Message {
    Beginner,
    Intermediate,
    Expert,
    Custom
}

#[derive(Default)]
struct Minesweeper {
    btn1: button::State,
    btn2: button::State,
    btn3: button::State,
    btn4: button::State,
    is_playing: bool,
    cells: Vec<button::State>,
    game: Game
}

impl Minesweeper {
    fn render_menu(&mut self) -> Column<Message> {
        Column::new()
            .padding(20)
            .push(
                Button::new(&mut self.btn1, Text::new("8x8 - 10 mines"))
                    .on_press(Message::Beginner),
            )
            .push(
                Button::new(&mut self.btn2, Text::new("16x16 - 40 mines"))
                    .on_press(Message::Intermediate),
            )
            .push(
                Button::new(&mut self.btn3, Text::new("30x16 - 99 mines"))
                    .on_press(Message::Expert),
            )
            .push(
                Button::new(&mut self.btn4, Text::new("Custom - ???"))
                    .on_press(Message::Custom),
            )
    }

    fn render_game(&mut self) -> Element<Message> {
        let row: Element<Message> = self.cells
            .iter_mut()
            .enumerate()
            .fold(Row::new(), |row, (i, state)| {
                row.push(Button::new(state, Text::new(format!("{}", i))))
            })
            .into();

        // for x in 0..width {
        //     for y in 0..self.game.opts.height {                
        //         row.push(Button::new(
        //             &mut button::State::new(),
        //             Text::new("ciao")
        //         ));
        //     }
        // }

        row
    }
}

impl Sandbox for Minesweeper {
    type Message = Message;

    fn new() -> Self {
        Minesweeper::default()
    }

    fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Beginner => {
                self.game = Game::new(GameOpts::preset_8x8_10_mines());
                self.cells = vec![button::State::new(); self.game.opts.width * self.game.opts.height];
                
                self.is_playing = true;
            }
            Message::Intermediate => {
                self.game = Game::new(GameOpts::preset_16x16_40_mines());
                self.cells = vec![button::State::new(); self.game.opts.width * self.game.opts.height];
                
                self.is_playing = true;
            }
            Message::Expert => {
                self.game = Game::new(GameOpts::preset_30x16_40_mines());
                self.cells = vec![button::State::new(); self.game.opts.width * self.game.opts.height];
                
                self.is_playing = true;
            }
            Message::Custom => {
                self.is_playing = true;
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content: Element<Message> = if !self.is_playing {
            self.render_menu().into()
        } else {
            self.render_game().into()
        };
    
        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() {
    let settings = iced::Settings::default();

    Minesweeper::run(settings).unwrap();
}
