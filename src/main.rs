#![windows_subsystem = "windows"]
use std::env::join_paths;

use clipboard::{ClipboardContext, ClipboardProvider};
use image::GenericImageView;

use image::DynamicImage;
use imgui::Context;
use imgui::TextureId;
use imgui::{Font, FontAtlas, FontAtlasFlags, FontAtlasTexture, FontConfig, FontGlyph, FontSource};
use imgui_sfml_support::*;
use sfml::{
    audio::*,
    graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture},
    window::{ContextSettings, Event, Style},
    ResourceLoadError,
};
fn test_text_edit_window(ui: &imgui::Ui, clipboard: &mut ClipboardContext, text_buf: &mut String) {
    ui.window("text edit").build(|| {
        ui.input_text_multiline("input", &mut text_buf.clone(), [280.0, 80.0])
            .build();
        // imguiのボタンを作成
        if ui.button("Compile") {
            // テキストエディタの内容をクリップボードにコピーする
            clipboard.set_contents(text_buf.to_string()).unwrap();
        }
    });
}
fn main() -> Result<(), ResourceLoadError> {
    let mut wnd = RenderWindow::new(
        (800, 800),
        "Rust: SFML-ImGui",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    let mut imgui = Context::create();
    // フォントデータを読み込む
    let font_data =
        include_bytes!("/Users/daruma/Downloads/misaki_ttf_2021-05-05/misaki_gothic_2nd.ttf");

    // フォント設定を作る
    let font_config = imgui::FontConfig {
        size_pixels: 18.0,
        glyph_ranges: imgui::FontGlyphRanges::japanese(), // 日本語の文字を含む
        ..Default::default()
    };

    let fonts = imgui.fonts(); // &mut を取得するため、`&mut` は必要ありません
    let font_id = fonts.add_font(&[imgui::FontSource::TtfData {
        size_pixels: 15.0,
        data: font_data,
        config: Some(font_config.clone()),
    }]);
    fonts.build_rgba32_texture();

    let mut renderer = SFMLRenderer::init(&mut imgui);
    let mut platform = SFMLPlatform::init(&mut imgui, &wnd);
    let mut music = Music::from_file("/Users/daruma/Downloads/touhou-sinki.wav")
        .ok_or("")
        .unwrap();
    music.play();
    let mut back_tex = Texture::new().unwrap();
    back_tex.load_from_file(
        "/Users/daruma/Downloads/irisu203/irisu203/photo.png",
        sfml::graphics::Rect::new(0, 0, 640, 480),
    )?;
    let mut console_buf_index = 0;
    let mut console_buf_: Vec<String> = Vec::new();
    let mut console_buf = String::new();
    let mut back_sp = Sprite::new();
    let mut console_enter_flag = false;
    let mut enter_key = false;
    back_sp.set_texture(&back_tex, false);
    let mut clipboard = ClipboardContext::new().unwrap();
    let mut text_edit_buf = String::with_capacity(256);

    while wnd.is_open() {
        while let Some(event) = wnd.poll_event() {
            platform.handle_event(&mut imgui, event);
            match event {
                Event::Closed => wnd.close(),
                Event::KeyPressed {
                    code,
                    scan,
                    alt,
                    ctrl,
                    ..
                } => {
                    if code == sfml::window::Key::Num1 {
                        if music.status() != SoundStatus::PLAYING {
                            music.play();
                        }
                    }
                    if code == sfml::window::Key::Enter {
                        enter_key = true;
                    } else {
                        enter_key = false;
                    }
                    if code == sfml::window::Key::Q {
                        music.stop();
                    }
                }
                _ => (),
            }

            platform.prepare_frame(&mut imgui);
            let mut ui = imgui.new_frame();
            // do your imgui work here
            ui.show_demo_window(&mut true);
            ui.window("こんそーる")
                .size([400.0, 250.0], imgui::Condition::FirstUseEver)
                .position([0.0, 300.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    if ui.button("enter") {
                        console_enter_flag = true;
                        console_buf_.push(console_buf.clone());
                        console_buf = "".to_string();
                    }
                    let _ = ui.input_text(" ", &mut console_buf).build();

                    if console_enter_flag {
                        // Display entries in reverse order to show new ones at the bottom
                        for entry in console_buf_.iter().rev() {
                            ui.text(entry);
                        }
                    }
                });
            ui.window("test").build(|| {});
            test_text_edit_window(&ui, &mut clipboard, &mut text_edit_buf);
            wnd.clear(Color::BLACK);
            wnd.reset_gl_states();

            renderer.render(&mut imgui, &mut wnd);
            //wnd.draw(&back_sp);
            wnd.display();
        }
    }
    Ok(())
}
