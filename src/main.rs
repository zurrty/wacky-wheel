mod app;
mod wheel;

use clap::Parser;
use thiserror::Error;

use macroquad::prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("JSON Error")]
    Json(#[from] serde_json::Error),
    #[error("Macroquad Error")]
    Macroquad(#[from] macroquad::Error),
    #[error("Missing Resource")]
    MissingResource(String),
    #[error("Failed to create audio context")]
    AudioContextFailed,

    #[error("Other error")]
    Other(#[from] Box<dyn std::error::Error>),
}

/// Simple GUI program to spin a wheel
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the wheel you want to use.
    #[arg(short, long)]
    wheel: Option<String>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Wheel Spin".to_owned(),
        window_resizable: true,
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Error> {
    set_pc_assets_folder("assets");

    let args = Args::parse();

    let wheel = match args.wheel {
        Some(path) => wheel::Wheel::load(path).unwrap(),
        None => wheel::Wheel::load("assets/default-wheel.json")?,
    };
    let font = match wheel.font.as_str() {
        "default" => load_ttf_font("font/NotoSans.ttf").await?,
        _ => {
            let bytes = std::fs::read(&wheel.font)?;

            load_ttf_font_from_bytes(&bytes)?
        }
    };

    let mut app = app::App::new(wheel, font);
    let mut selected: Option<usize> = None;
    let mut show_end_screen = false;
    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
        if is_key_pressed(macroquad::prelude::KeyCode::Space) && !app.is_spinning {
            app.start_spin();
            show_end_screen = false;
        }
        app.update_wheel();
        if app.is_spinning {
            let new = app.get_index();
            if selected != Some(new) {
                selected = Some(new);
            }
        } else if let Some(index) = selected {
            let choice = app.wheel.choices.get(index).cloned().unwrap();
            println!("Selected item: {:?}", choice);
            selected = None;
            show_end_screen = true;
        }
        app.draw_wheel();

        if show_end_screen {
            let center_x = screen_width() / 2.0;
            let center_y = screen_height() / 2.0;
            let choice = app.wheel.choices.get(app.get_index()).cloned().unwrap();
            let name_dims = measure_text(&choice.name, Some(&app.font), 52, 1.0);
            let rect_height = name_dims.height + 72.0;
            let rect_y = center_y - rect_height / 2.0;
            draw_rectangle(0.0, rect_y, screen_width(), rect_height, DARKGRAY);
            draw_rectangle_lines(
                -24.0,
                rect_y,
                screen_width() + 48.0,
                rect_height,
                12.0,
                GRAY,
            );

            draw_text_ex(
                &choice.name,
                center_x - name_dims.width / 2.0,
                rect_y + name_dims.height + 8.0,
                TextParams {
                    font: Some(&app.font),
                    font_size: 52,
                    font_scale: 1.0,
                    color: WHITE,
                    ..Default::default()
                },
            );
            if let Some(description) = &choice.desc {
                let desc_dims = measure_text(description, Some(&app.font), 32, 1.0);
                draw_text_ex(
                    description,
                    center_x - desc_dims.width / 2.0,
                    rect_y + (rect_height - 24.0),
                    TextParams {
                        font: Some(&app.font),
                        font_size: 32,
                        font_scale: 1.0,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        const SPIN_TEXT: &str = "Press SPACE to spin.";
        let center_x = screen_width() / 2.0;
        let dimensions = measure_text(SPIN_TEXT, Some(&app.font), 1, 16.0);
        draw_text(
            SPIN_TEXT,
            center_x - dimensions.width / 2.0,
            screen_height() - 16.0,
            16.0,
            WHITE,
        );

        next_frame().await;
    }
}
