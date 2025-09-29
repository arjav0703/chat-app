#![allow(unused)]
use chrono::Local;
use cursive::{
    Cursive, CursiveExt,
    align::HAlign,
    event::Key,
    theme::{BaseColor, Color, ColorStyle, Effect, Palette, PaletteColor, Style, Theme},
    traits::*,
    views::{Dialog, EditView, LinearLayout, OnEventView, Panel, ScrollView, SelectView, TextView},
};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, sync::Arc};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    Broadcast,
    UserMessage,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = env::args().nth(1).unwrap_or("Anonymous".to_string());

    //cursion setup
    let mut siv = Cursive::default();
    siv.set_theme(create_theme());

    let header = TextView::new("🎀 Rusty Chat 🎀")
        .h_align(HAlign::Center)
        .style(Style::from(ColorStyle::new(
            PaletteColor::TitlePrimary,
            PaletteColor::Background,
        )))
        .fixed_height(1);

    let message_view = TextView::new("")
        .with_name("message_view")
        .scrollable()
        .min_height(25)
        .full_height();

    let message_view = ScrollView::new(message_view)
        .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
        .min_width(50)
        .full_width();

    let input_view = EditView::new()
        .on_submit(move |s, text| {
            send_messages(s, text.to_string());
            s.call_on_name("input_view", |view: &mut EditView| {
                view.set_content("");
            });
        })
        .with_name("input_view")
        .max_height(4)
        .full_width();

    let help_text = TextView::new("ESC -> Quit | Enter -> Send Message")
        .h_align(HAlign::Center)
        .style(Style::from(ColorStyle::new(
            PaletteColor::Secondary,
            PaletteColor::Background,
        )))
        .fixed_height(1);

    let layout = LinearLayout::vertical()
        .child(Panel::new(header))
        .child(
            Dialog::around(message_view)
                .title("Messages")
                .title_position(HAlign::Center)
                .full_width(),
        )
        .child(Panel::new(input_view))
        .child(Panel::new(help_text).full_width());

    let centered_layout = LinearLayout::horizontal()
        .child(cursive::views::DummyView.fixed_width(2))
        .child(layout)
        .child(cursive::views::DummyView.fixed_width(2));

    siv.add_fullscreen_layer(centered_layout);

    // keybindings
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('/', |s| {
        s.call_on_name("input", |view: &mut EditView| {
            view.set_content("/");
        });
    });

    let stream = TcpStream::connect("127.0.0.1:6969").await?;
    let (reader, mut writer) = stream.into_split();
    writer
        .write_all(format!("{}\n", username).as_bytes())
        .await?;

    let writer = Arc::new(Mutex::new(writer));
    let writer_clone = Arc::clone(&writer);

    siv.set_user_data(writer);

    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    let sink = siv.cb_sink().clone();

    tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if let Ok(message) = serde_json::from_str::<Message>(&line) {
                let formatted_message = format!(
                    "[{}] {}: {}\n",
                    message.timestamp, message.username, message.content
                );

                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                    s.call_on_name("message_view", |view: &mut TextView| {
                        view.append(formatted_message);
                    });
                }));
            }
        }
    });

    siv.run();
    let _ = writer_clone.lock().await.shutdown().await;

    Ok(())
}

fn create_theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = true;
    theme.borders = cursive::theme::BorderStyle::Outset;

    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Rgb(237, 133, 182);
    palette[PaletteColor::View] = Color::Rgb(255, 192, 203);
    palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Highlight] = Color::Dark(BaseColor::White);
    palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);

    theme.palette = palette;

    theme
}

fn send_messages(siv: &mut Cursive, text: String) {
    if text.is_empty() {
        return;
    }

    match text.as_str() {
        "clear" => {
            siv.call_on_name("message_view", |view: &mut TextView| {
                view.set_content("");
            });
        }
        "/quit" => siv.quit(),
        _ => {}
    }

    let writer = siv
        .user_data::<Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>>()
        .unwrap()
        .clone();
    tokio::spawn(async move {
        let mut writer = writer.lock().await;
        if let Err(e) = writer.write_all(format!("{}\n", text).as_bytes()).await {
            eprintln!("Failed to send message: {}", e);
        }
    });
}
