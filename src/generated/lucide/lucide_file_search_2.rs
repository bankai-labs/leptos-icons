use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" /> < polyline points = "14 2 14 8 20 8" /> < circle cx = "11.5" cy = "14.5" r = "2.5" /> < path d = "M13.25 16.25 15 18" /> < / > } } pub const LucideFileSearch2 : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;