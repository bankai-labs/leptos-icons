use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M5 16v2" /> < path d = "M19 16v2" /> < rect x = "2" y = "8" width = "20" height = "8" rx = "2" /> < path d = "M18 12h0" /> < / > } } pub const LucideRadioReceiver : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;