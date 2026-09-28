#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, RichText};
use rillwire_core::{Backend, Command, DemoBackend, Event, Message, MessageCache, MessageKey};
use std::sync::mpsc::{Receiver, TryRecvError, sync_channel};

const CHANNELS: [&str; 4] = ["general", "build-log", "native-ui", "transport-lab"];
const TEAL: Color32 = Color32::from_rgb(109, 224, 202);

fn main() -> eframe::Result {
    eframe::run_native(
        "Rillwire · native research preview",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1180.0, 760.0])
                .with_min_inner_size([860.0, 560.0]),
            renderer: eframe::Renderer::Wgpu,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Rillwire::new(&cc.egui_ctx)))),
    )
}

struct Rillwire {
    cache: MessageCache,
    backend: DemoBackend,
    channel: usize,
    visible: Vec<MessageKey>,
    composer: String,
    query: String,
    selection: Option<MessageKey>,
    status: String,
    focus_search: bool,
    stress: Option<Receiver<Message>>,
    next_synthetic: u64,
}

fn synthetic(id: u64) -> Message {
    let authors = ["Ari", "Mika", "Jules", "Rowan"];
    let bodies = [
        "A small client should still feel like a complete place to talk.",
        "The cache has an entry cap and a byte budget. Both matter.",
        "This is synthetic local data. Nothing here was sent to Discord.",
        "Let's measure input-to-paint latency on Windows before calling it fast.",
        "Voice needs its own audio thread, a jitter buffer, and DAVE support.",
        "Transport experiments belong in a lab where we control both peers.",
    ];
    Message {
        key: MessageKey {
            channel_id: id % 4,
            id,
        },
        author: authors[(id / 4) as usize % 4].into(),
        content: bodies[(id / 4) as usize % 6].into(),
    }
}

impl Rillwire {
    fn new(ctx: &egui::Context) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(19, 24, 30);
        visuals.window_fill = Color32::from_rgb(25, 31, 38);
        visuals.selection.bg_fill = Color32::from_rgb(33, 77, 72);
        visuals.selection.stroke.color = TEAL;
        ctx.set_visuals(visuals);
        let mut app = Self {
            cache: MessageCache::new(4096, 4 * 1024 * 1024),
            backend: DemoBackend::default(),
            channel: 0,
            visible: Vec::new(),
            composer: String::new(),
            query: String::new(),
            selection: None,
            status: "Local demo · no Discord connection".into(),
            focus_search: false,
            stress: None,
            next_synthetic: 1200,
        };
        for id in 0..1200 {
            app.cache.insert(synthetic(id)).unwrap();
        }
        app.refresh();
        app
    }
    fn refresh(&mut self) {
        let q = self.query.to_lowercase();
        self.visible = self
            .cache
            .channel_keys(self.channel as u64)
            .into_iter()
            .filter(|key| {
                self.cache.get(*key).is_some_and(|m| {
                    q.is_empty()
                        || m.content.to_lowercase().contains(&q)
                        || m.author.to_lowercase().contains(&q)
                })
            })
            .collect();
    }
    fn send(&mut self) {
        match self.backend.execute(Command::SendMessage {
            channel_id: self.channel as u64,
            content: self.composer.clone(),
        }) {
            Ok(Event::MessageCreated(message)) => {
                self.selection = Some(message.key);
                if self.cache.insert(message).is_ok() {
                    self.composer.clear();
                    self.refresh();
                    self.status =
                        "Added locally. Demo messages disappear when the app closes.".into();
                }
            }
            Err(error) => self.status = error,
            _ => (),
        }
    }
    fn start_stress(&mut self, ctx: &egui::Context) {
        let (tx, rx) = sync_channel(128);
        let start = self.next_synthetic;
        self.next_synthetic += 50_000;
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            for id in start..start + 50_000 {
                if tx.send(synthetic(id)).is_err() {
                    return;
                }
                if id % 64 == 0 {
                    ctx.request_repaint();
                }
            }
            ctx.request_repaint();
        });
        self.stress = Some(rx);
        self.status = "Feeding 50,000 synthetic messages through a bounded queue…".into();
    }
    fn drain_stress(&mut self, ctx: &egui::Context) {
        let mut changed = false;
        let mut finished = false;
        if let Some(rx) = &self.stress {
            for _ in 0..256 {
                match rx.try_recv() {
                    Ok(message) => {
                        let _ = self.cache.insert(message);
                        changed = true;
                    }
                    Err(TryRecvError::Disconnected) => {
                        finished = true;
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                }
            }
        }
        if finished {
            self.stress = None;
            self.status =
                "Load complete. Cache retains at most 4,096 messages / 4 MiB of text.".into();
        }
        if changed {
            self.refresh();
            ctx.request_repaint();
        }
    }
}

impl Rillwire {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        self.drain_stress(&ctx);
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::K)) {
            self.focus_search = true;
        }
        egui::Panel::top("masthead").show(ui, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("≋  RILLWIRE").size(22.0).strong().color(TEAL));
                ui.separator();
                ui.label("Conversation, in its element.");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("OFFLINE PREVIEW  /  v0.1")
                            .small()
                            .color(TEAL),
                    );
                });
            });
            ui.add_space(10.0);
        });
        egui::Panel::bottom("status").show(ui, |ui| {
            ui.label(RichText::new(&self.status).small());
        });
        egui::Panel::left("channels")
            .exact_size(206.0)
            .resizable(false)
            .show(ui, |ui| {
                ui.add_space(20.0);
                ui.heading("The workshop");
                ui.label(RichText::new("SYNTHETIC COMMUNITY").small().weak());
                ui.add_space(22.0);
                for (index, channel) in CHANNELS.iter().enumerate() {
                    if ui
                        .add_sized(
                            [182.0, 36.0],
                            egui::Button::new(format!("#  {channel}"))
                                .selected(self.channel == index),
                        )
                        .clicked()
                    {
                        self.channel = index;
                        self.selection = None;
                        self.refresh();
                    }
                }
                ui.add_space(26.0);
                ui.label(RichText::new("VOICE").small().weak());
                ui.add_enabled(false, egui::Button::new("Join workshop voice"));
                ui.label(
                    RichText::new("Planned · DAVE integration required")
                        .small()
                        .weak(),
                );
                ui.add_space(30.0);
                ui.separator();
                ui.label("Native Rust + GPU rendering");
                ui.label(
                    RichText::new("No browser engine in this desktop app.")
                        .small()
                        .weak(),
                );
            });
        egui::Panel::right("inspector").exact_size(235.0).resizable(false).show(ui, |ui| {
            ui.add_space(20.0); ui.heading("Under the surface"); ui.add_space(16.0);
            ui.label(RichText::new(format!("{}", self.cache.len())).size(32.0).color(TEAL)); ui.label("retained messages / 4,096");
            ui.add_space(14.0); ui.label(format!("{:.1} KiB text payload", self.cache.payload_bytes() as f64 / 1024.0));
            ui.label(RichText::new("Text only. This is not process RAM.").small().weak());
            ui.add_space(16.0);
            if ui.add_enabled(self.stress.is_none(), egui::Button::new("Feed 50,000 messages")).clicked() { self.start_stress(&ctx); }
            ui.add_space(18.0); ui.separator(); ui.heading("Message detail");
            egui::ScrollArea::vertical().id_salt("detail").show(ui, |ui| {
                if let Some(message) = self.selection.and_then(|key| self.cache.get(key)) {
                    ui.label(RichText::new(&message.author).color(TEAL));
                    ui.label(&message.content);
                } else { ui.label(RichText::new("Select a message to read its full text. Timeline rows use a compact one-line preview.").weak()); }
            });
        });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!("# {}", CHANNELS[self.channel]));
            let search = ui.add(
                egui::TextEdit::singleline(&mut self.query)
                    .hint_text("Search this cached channel · Ctrl+K")
                    .desired_width(f32::INFINITY),
            );
            if self.focus_search {
                search.request_focus();
                self.focus_search = false;
            }
            if search.changed() {
                self.refresh();
            }
            ui.separator();
            let timeline_height = (ui.available_height() - 96.0).max(100.0);
            egui::ScrollArea::vertical()
                .id_salt(("messages", self.channel))
                .max_height(timeline_height)
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show_rows(ui, 48.0, self.visible.len(), |ui, range| {
                    for row in range {
                        let key = self.visible[row];
                        if let Some(message) = self.cache.get(key) {
                            ui.allocate_ui(egui::vec2(ui.available_width(), 48.0), |ui| {
                                ui.label(RichText::new(&message.author).color(TEAL).strong());
                                if ui
                                    .add(
                                        egui::Label::new(&message.content)
                                            .truncate()
                                            .sense(egui::Sense::click()),
                                    )
                                    .clicked()
                                {
                                    self.selection = Some(key);
                                }
                            });
                        }
                    }
                });
            ui.separator();
            let composer = ui.add(
                egui::TextEdit::singleline(&mut self.composer)
                    .hint_text("Write a local demo message…")
                    .char_limit(2000)
                    .desired_width(f32::INFINITY),
            );
            ui.horizontal(|ui| {
                let enter = composer.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Add locally").clicked() || enter {
                    self.send();
                    composer.request_focus();
                }
                ui.label(
                    RichText::new("No account required · no messages leave this app")
                        .small()
                        .weak(),
                );
            });
        });
    }
}

impl eframe::App for Rillwire {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_compose_and_channel_search_produce_drawable_ui() {
        let ctx = egui::Context::default();
        let mut app = Rillwire::new(&ctx);
        app.channel = 2;
        app.composer = "Unique local message".into();
        app.send();
        app.query = "Unique local".into();
        app.refresh();
        assert_eq!(app.visible.len(), 1);
        assert_eq!(app.visible[0].channel_id, 2);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 760.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| app.draw(ui));
        assert!(!output.shapes.is_empty());
        // This headless UI test has no GPU; deliberately discard the texture upload.
        output.textures_delta.clear();
        app.channel = 1;
        app.refresh();
        assert!(app.visible.is_empty());
    }
}
