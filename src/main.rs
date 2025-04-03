// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! An example of a transparent window background.
//! Useful for dropdowns, tooltips and other overlay windows.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use chrono::Local;
use druid::text::FontDescriptor;
use druid::widget::prelude::*;
use druid::widget::Controller;
use druid::widget::{Flex, Image, Label, WidgetExt};
use druid::TimerToken;
use druid::{AppLauncher, Color, ImageBuf, Lens, WindowDesc};
use image::ImageReader;
use std::io::Cursor;
use std::time::Duration;
use stock::fetch_stock_data;
mod stock;

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    current_time: String, // Add a field for the current time
    stock: String,
}

struct DragController;

impl<T, W: Widget<T>> Controller<T, W> for DragController {
    fn event(
        &mut self,
        _child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        _data: &mut T,
        _env: &Env,
    ) {
        if let Event::MouseMove(_) = event {
            ctx.window().handle_titlebar(true);
        }
    }
}

struct TimeUpdater {
    timer: TimerToken,
}

impl TimeUpdater {
    fn new() -> Self {
        Self {
            timer: TimerToken::INVALID,
        }
    }
}

impl<W: Widget<HelloState>> Controller<HelloState, W> for TimeUpdater {
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &HelloState,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.timer = ctx.request_timer(Duration::from_secs(1)); // 每秒更新时间
        }
        child.lifecycle(ctx, event, data, env);
    }

    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut HelloState,
        env: &Env,
    ) {
        if let Event::Timer(id) = event {
            if *id == self.timer {
                // 更新当前时间
                data.current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                // 每 30 秒刷新股票数据
                if Local::now().timestamp() % 30 == 0 {
                    let stock = stock::fetch_stock_data();
                    data.stock = stock;
                }

                ctx.request_update();
                self.timer = ctx.request_timer(Duration::from_secs(1)); // 每秒触发一次
            }
        }
        child.event(ctx, event, data, env);
    }
}

pub fn main() {
    let window = WindowDesc::new(build_root_widget())
        .show_titlebar(false)
        .window_size((180., 300.))
        .set_always_on_top(true)
        .transparent(true)
        .resizable(false)
        .title("Transparent background");

    let stock = fetch_stock_data();

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(HelloState {
            name: "".into(),
            stock: stock,
            current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
        .expect("launch failed");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // 使用 image crate 加载图片
    let image_data = include_bytes!("./dragon.png");
    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    // 将 image crate 的 DynamicImage 转换为 Druid 的 ImageBuf
    let raw_image = img.to_rgba8();
    let (width, height) = raw_image.dimensions();
    let image_buf = ImageBuf::from_raw(
        raw_image.into_raw(),
        druid::piet::ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    );

    // 创建 Druid 的 Image 小部件
    let image = Image::new(image_buf);
    // 加载自定义字体
    let font = FontDescriptor::new(druid::FontFamily::MONOSPACE);
    let time_label = Label::new(|data: &HelloState, _env: &Env| format!("{}", data.current_time))
        .with_text_color(Color::YELLOW)
        .with_font(font.clone())
        .with_text_size(16.0)
        .background(Color::rgba8(0, 0, 0, 128));

    let stock_label = Label::new(|data: &HelloState, _env: &Env| format!("{}", data.stock))
        .with_text_color(Color::GREEN)
        .with_font(font)
        .with_text_size(16.0)
        .background(Color::rgba8(0, 0, 0, 128));

    // 布局
    Flex::column()
        .with_child(image)
        .with_spacer(4.0)
        .with_child(time_label)
        .with_spacer(1.0)
        .with_child(stock_label)
        .with_spacer(4.0)
        .controller(DragController)
        .controller(TimeUpdater::new())
}
