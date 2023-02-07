use crate::{IconType, Path};
use leptos::*;

fn get_svg_path_element(cx: Scope) -> Fragment {
    view! {cx,
        <>
            <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
        </>
    }
}

pub const CHEVRON_RIGHT: Path = Path {
    path: get_svg_path_element,
    icon_type: IconType::HeroIcons(crate::HeroIconsType::Outline),
};

fn icon_path(cx: Scope) -> Fragment {
    view! { cx , < > < path d = "M10 3.75a2 2 0 10-4 0 2 2 0 004 0zM17.25 4.5a.75.75 0 000-1.5h-5.5a.75.75 0 000 1.5h5.5zM5 3.75a.75.75 0 01-.75.75h-1.5a.75.75 0 010-1.5h1.5a.75.75 0 01.75.75zM4.25 17a.75.75 0 000-1.5h-1.5a.75.75 0 000 1.5h1.5zM17.25 17a.75.75 0 000-1.5h-5.5a.75.75 0 000 1.5h5.5zM9 10a.75.75 0 01-.75.75h-5.5a.75.75 0 010-1.5h5.5A.75.75 0 019 10zM17.25 10.75a.75.75 0 000-1.5h-1.5a.75.75 0 000 1.5h1.5zM14 10a2 2 0 10-4 0 2 2 0 004 0zM10 16.25a2 2 0 10-4 0 2 2 0 004 0z" /> < / > }
}
pub const heroicons_mini_solid_adjustments_horizontal: Path = Path {
    path: icon_path,
    icon_type: IconType::HeroIcons(crate::HeroIconsType::Outline),
};
