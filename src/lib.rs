use leptos::*;

pub mod test;

#[derive(Clone)]
pub struct Path {
    pub path: fn(leptos::Scope) -> leptos::Fragment,
    pub icon_type: IconType,
}

#[derive(Clone)]
pub enum IconType {
    HeroIcons(HeroIconsType),
}

#[derive(Clone)]
pub enum HeroIconsType {
    Outline,
    Solid,
    Mini,
}

#[component]
pub fn Icon(
    cx: Scope,
    path: Path,

    #[prop(into)]
    #[prop(optional)]
    class: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    fill: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    stroke_width: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    stroke: MaybeSignal<String>,
) -> impl IntoView {
    match path.icon_type {
        IconType::HeroIcons(HeroType) => match HeroType {
            HeroIconsType::Outline => {
                view! {cx,
                    <svg xmlns="http://www.w3.org/2000/svg" class=class() width="24" height="24" viewBox="0 0 24 24"
                        fill=move || fill().is_empty().then_some("none".to_string()).unwrap_or(fill())
                        stroke-width=move || stroke_width().is_empty().then_some("2".to_string()).unwrap_or(stroke_width())
                        stroke=move || stroke().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke())
                    >
                        {(path.path)(cx)}
                    </svg>
                }
            }
            HeroIconsType::Solid => {
                view! {cx,
                    <svg xmlns="http://www.w3.org/2000/svg" class=class() width="24" height="24" viewBox="0 0 24 24"
                        fill=move || fill().is_empty().then_some("currentColor".to_string()).unwrap_or(fill())
                        stroke-width=move || stroke_width().is_empty().then_some("2".to_string()).unwrap_or(stroke_width()) stroke=move || stroke().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke())
                    >
                        {(path.path)(cx)}
                    </svg>
                }
            }
            HeroIconsType::Mini => {
                view! {cx,
                    <svg xmlns="http://www.w3.org/2000/svg" class=class() width="20" height="20" viewBox="0 0 20 20"
                        fill=move || fill().is_empty().then_some("currentColor".to_string()).unwrap_or(fill())
                        stroke-width=move || stroke_width().is_empty().then_some("2".to_string()).unwrap_or(stroke_width()) stroke=move || stroke().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke())
                    >
                        {(path.path)(cx)}
                    </svg>

                }
            }
        },
    }
}
