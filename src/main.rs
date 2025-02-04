#![windows_subsystem = "windows"]
use freya::prelude::*;
use time::{format_description, Duration, OffsetDateTime, PrimitiveDateTime};

fn format_countdown(duration: Duration) -> String {
    format!("Weeks:{} Days:{} Hours:{} Minutes:{} Seconds:{}", duration.whole_weeks(), duration.whole_days(), duration.whole_hours(), duration.whole_minutes(), duration.whole_seconds())
}

fn app() -> Element {
    let mut current_date = use_signal(|| OffsetDateTime::now_local().unwrap());
    use_hook(move || {
        spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
            loop {
                interval.tick().await;
                current_date.set(OffsetDateTime::now_local().unwrap());
            }
        });
    });
    let date_format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let read_date = *current_date.read();
    let formatted_date = read_date.format(&date_format).unwrap();
    let mut date_input = use_signal(|| formatted_date.clone());
    let mut is_inputting = use_signal(|| false);
    let mut future_date = use_signal(|| read_date);
    rsx!(
        rect {
            font_family: "consolas",
            height: "100%",
            width: "100%",
            label { "Current Time: {formatted_date}" }
            rect {
                width: "100%",
                height: "100%",
                main_align: "center",
                cross_align: "center",
                rect {
                    main_align: "center",
                    cross_align: "center",
                    direction: "horizontal",
                    if *is_inputting.read() {
                        Input {
                            theme: theme_with!(InputTheme {
                                width: "191".into(),
                                margin: "0".into()
                            }),
                            value: date_input,
                            onchange: move |e| date_input.set(e)
                        }
                        Button {
                            label { "Done" },
                            onclick: move |_| {
                                future_date
                                    .set(PrimitiveDateTime::parse(&date_input.read(), &date_format)
                                    .expect("Failed to read input")
                                    .assume_offset(read_date.offset()));
                                is_inputting.set(false)
                            }
                        }
                    } else {
                        label {
                            margin: "0 12",
                            "{date_input.read()}"
                        }
                        Button {
                            label { "Edit" },
                            onclick: move |_| is_inputting.set(true)
                        }
                    }
                }
                label {
                    "{format_countdown(*future_date.read() - read_date)}"
                }
            }
        }
    )
}

const SIZE: f64 = 900.0;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(SIZE)
            .with_min_width(SIZE)
            .with_height(SIZE)
            .with_min_height(SIZE)
            .with_title("Countdown")
            .build(),
    );
}
