use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "2" x2 = "5" y1 = "12" y2 = "12" /> < line x1 = "19" x2 = "22" y1 = "12" y2 = "12" /> < line x1 = "12" x2 = "12" y1 = "2" y2 = "5" /> < line x1 = "12" x2 = "12" y1 = "19" y2 = "22" /> < circle cx = "12" cy = "12" r = "7" /> < / > } } pub const LucideLocate : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;