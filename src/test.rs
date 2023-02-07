use leptos::*;

use crate::{IconType, Path, HeroIconsType};


fn get_svg_path_element(cx: Scope) -> Fragment {
    view! {cx,
        <>
            <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
        </>
    }
}

pub const CHEVRON_RIGHT: Path = Path {
    path: get_svg_path_element,
    icon_type: IconType::HeroIcons(HeroIconsType::Outline),
};