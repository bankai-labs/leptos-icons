use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M3.75 6.75h16.5M3.75 12h16.5M12 17.25h8.25" /> < / > } } pub const HeroiconsOutlineBars3BottomRight : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;