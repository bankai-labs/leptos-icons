use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /> < path stroke - linecap = "round" stroke - linejoin = "round" d = "M9 9.563C9 9.252 9.252 9 9.563 9h4.874c.311 0 .563.252.563.563v4.874c0 .311-.252.563-.563.563H9.564A.562.562 0 019 14.437V9.564z" /> < / > } } pub const HeroiconsOutlineStopCircle : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;