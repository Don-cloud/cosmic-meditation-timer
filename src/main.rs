use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

use iced::widget::{button, column, container, row, text, text_input};
use iced::{executor, time, Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

const APP_NAME: &str = "Meditation Countdown Timer";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DEVELOPER: &str = "Sajal";

fn main() -> iced::Result {
    TimerApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    HoursChanged(String),
    MinutesChanged(String),
    SecondsChanged(String),
    StartPressed,
    PausePressed,
    ResetPressed,
    ToggleAbout,
    ToggleDeveloper,
    Tick,
}

struct TimerApp {
    input_hours: String,
    input_minutes: String,
    input_seconds: String,
    remaining: Duration,
    running: bool,
    status: String,
    show_about: bool,
    show_developer: bool,
}

impl Application for TimerApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                input_hours: String::new(),
                input_minutes: String::new(),
                input_seconds: String::new(),
                remaining: Duration::ZERO,
                running: false,
                status: String::from("Enter a duration and press Start."),
                show_about: false,
                show_developer: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Meditation Countdown v{APP_VERSION}")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::HoursChanged(value) => {
                if !self.running {
                    self.input_hours = digits_only(value);
                }
            }
            Message::MinutesChanged(value) => {
                if !self.running {
                    self.input_minutes = digits_only(value);
                }
            }
            Message::SecondsChanged(value) => {
                if !self.running {
                    self.input_seconds = digits_only(value);
                }
            }
            Message::StartPressed => {
                if self.running {
                    return Command::none();
                }

                if self.remaining.is_zero() {
                    match parse_total_duration(
                        &self.input_hours,
                        &self.input_minutes,
                        &self.input_seconds,
                    ) {
                        Some(duration) if !duration.is_zero() => {
                            self.remaining = duration;
                            self.running = true;
                            self.status = String::from("Running...");
                        }
                        Some(_) => {
                            self.status = String::from("Set at least 1 second.");
                        }
                        None => {
                            self.status = String::from("Duration is too large.");
                        }
                    }
                } else {
                    self.running = true;
                    self.status = String::from("Running...");
                }
            }
            Message::PausePressed => {
                if self.running {
                    self.running = false;
                    self.status = String::from("Paused.");
                }
            }
            Message::ResetPressed => {
                self.running = false;
                self.remaining = Duration::ZERO;
                self.status = String::from("Reset. Enter a new duration.");
            }
            Message::ToggleAbout => {
                self.show_about = !self.show_about;
            }
            Message::ToggleDeveloper => {
                self.show_developer = !self.show_developer;
            }
            Message::Tick => {
                if self.running {
                    if self.remaining > Duration::from_secs(1) {
                        self.remaining -= Duration::from_secs(1);
                    } else {
                        self.remaining = Duration::ZERO;
                        self.running = false;
                        self.status = String::from("Time is up. Bell played.");
                        play_meditation_bell_async();
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.running {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let hrs = text_input("Hours", &self.input_hours)
            .on_input(Message::HoursChanged)
            .padding(10)
            .width(Length::Fixed(110.0));

        let mins = text_input("Minutes", &self.input_minutes)
            .on_input(Message::MinutesChanged)
            .padding(10)
            .width(Length::Fixed(110.0));

        let secs = text_input("Seconds", &self.input_seconds)
            .on_input(Message::SecondsChanged)
            .padding(10)
            .width(Length::Fixed(110.0));

        let inputs = row![hrs, mins, secs].spacing(12).align_items(Alignment::Center);

        let time_display = if self.remaining.is_zero() {
            match parse_total_duration(&self.input_hours, &self.input_minutes, &self.input_seconds) {
                Some(d) => d,
                None => Duration::ZERO,
            }
        } else {
            self.remaining
        };

        let timer_text = text(format_duration(time_display)).size(56);

        let start_button = button("Start")
            .padding(10)
            .on_press(Message::StartPressed);

        let pause_button = button("Pause")
            .padding(10)
            .on_press(Message::PausePressed);

        let reset_button = button("Reset")
            .padding(10)
            .on_press(Message::ResetPressed);

        let about_button = button("About")
            .padding(10)
            .on_press(Message::ToggleAbout);

        let developer_button = button("Developer")
            .padding(10)
            .on_press(Message::ToggleDeveloper);

        let mut content = column![
            text(APP_NAME).size(34),
            text(format!("Version {APP_VERSION}")).size(16),
            text("Set hours, minutes, or seconds").size(18),
            inputs,
            timer_text,
            row![start_button, pause_button, reset_button].spacing(10),
            row![about_button, developer_button].spacing(10),
            text(&self.status).size(16)
        ];

        if self.show_about {
            content = content.push(
                container(
                    column![
                        text("About").size(20),
                        text("A simple meditation timer with a bell at zero.").size(16),
                        text(format!("Version: {APP_VERSION}")).size(16),
                    ]
                    .spacing(6),
                )
                .padding(12)
                .width(Length::Fill),
            );
        }

        if self.show_developer {
            content = content.push(
                container(
                    column![
                        text("Developer").size(20),
                        text(format!("Name: {APP_DEVELOPER}")).size(16),
                        text("Built with Rust, Iced, and Rodio").size(16),
                    ]
                    .spacing(6),
                )
                .padding(12)
                .width(Length::Fill),
            );
        }

        let content = content
        .spacing(20)
        .align_items(Alignment::Center)
        .padding(24)
        .max_width(500);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn digits_only(value: String) -> String {
    value.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn parse_total_duration(hours: &str, minutes: &str, seconds: &str) -> Option<Duration> {
    let h = parse_u64_or_zero(hours);
    let m = parse_u64_or_zero(minutes);
    let s = parse_u64_or_zero(seconds);

    let seconds_total = h
        .checked_mul(3600)?
        .checked_add(m.checked_mul(60)?)?
        .checked_add(s)?;

    Some(Duration::from_secs(seconds_total))
}

fn parse_u64_or_zero(value: &str) -> u64 {
    if value.trim().is_empty() {
        0
    } else {
        value.parse::<u64>().unwrap_or(0)
    }
}

fn format_duration(duration: Duration) -> String {
    let total = duration.as_secs();
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

fn play_meditation_bell_async() {
    thread::spawn(|| {
        if let Err(err) = play_meditation_bell() {
            eprintln!("Failed to play bell: {err}");
        }
    });
}

fn play_meditation_bell() -> Result<(), String> {
    let sample_rate = 44_100;
    let samples = synth_meditation_bell(sample_rate);

    let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
    let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
    let source = SamplesBuffer::new(1, sample_rate, samples);

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

fn synth_meditation_bell(sample_rate: u32) -> Vec<f32> {
    let total_seconds = 5.0_f32;
    let total_samples = (total_seconds * sample_rate as f32) as usize;
    let mut output = vec![0.0_f32; total_samples];

    add_bell_strike(&mut output, sample_rate, 0.0, 196.0, 2.8, 0.8);
    add_bell_strike(&mut output, sample_rate, 1.6, 196.0, 2.6, 0.6);

    for sample in &mut output {
        *sample = (*sample).clamp(-1.0, 1.0);
    }

    output
}

fn add_bell_strike(
    output: &mut [f32],
    sample_rate: u32,
    start_time: f32,
    base_freq: f32,
    decay_seconds: f32,
    gain: f32,
) {
    let start_index = (start_time * sample_rate as f32) as usize;
    let strike_len = (decay_seconds * sample_rate as f32) as usize;

    for i in 0..strike_len {
        let idx = start_index + i;
        if idx >= output.len() {
            break;
        }

        let t = i as f32 / sample_rate as f32;
        let attack = (t / 0.03).min(1.0);
        let decay = (-2.7 * t / decay_seconds).exp();
        let envelope = attack * decay * gain;

        let fundamental = (2.0 * PI * base_freq * t).sin();
        let harmonic_2 = (2.0 * PI * base_freq * 2.71 * t).sin() * 0.45;
        let harmonic_3 = (2.0 * PI * base_freq * 5.43 * t).sin() * 0.22;
        let shimmer = (2.0 * PI * (base_freq * 8.2) * t).sin() * 0.08;

        output[idx] += envelope * (fundamental + harmonic_2 + harmonic_3 + shimmer) / 1.75;
    }
}
