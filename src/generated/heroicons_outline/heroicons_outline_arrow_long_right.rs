use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M17.25 8.25L21 12m0 0l-3.75 3.75M21 12H3" /> < / > } } pub const HeroiconsOutlineArrowLongRight : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;