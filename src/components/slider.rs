use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Slider(
    #[props(default = 50.0)] value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    onchange: Option<EventHandler<f64>>,
) -> Element {
    let percentage = if max > min {
        ((value - min) / (max - min)) * 100.0
    } else {
        0.0
    };
    let track_style = format!("width: {}%;", percentage);
    let classes = cn(&[
        "relative flex w-full touch-none items-center select-none",
        if disabled { "opacity-50" } else { "" },
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            // Hidden native input for accessibility
            input {
                r#type: "range",
                class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                disabled,
                oninput: move |evt: FormEvent| {
                    if let Some(handler) = &onchange {
                        if let Ok(v) = evt.value().parse::<f64>() {
                            handler.call(v);
                        }
                    }
                },
            }
            // Visual track
            div {
                class: "relative h-1.5 w-full grow overflow-hidden rounded-full bg-muted",
                div {
                    class: "absolute h-full bg-primary",
                    style: "{track_style}",
                }
            }
            // Visual thumb
            div {
                class: "absolute block size-4 rounded-full border border-primary bg-white shadow-sm ring-ring/50 transition-[color,box-shadow]",
                style: "left: calc({percentage}% - 0.5rem);",
            }
        }
    }
}
