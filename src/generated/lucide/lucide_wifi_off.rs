use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" /> < path d = "M8.5 16.5a5 5 0 0 1 7 0" /> < path d = "M2 8.82a15 15 0 0 1 4.17-2.65" /> < path d = "M10.66 5c4.01-.36 8.14.9 11.34 3.76" /> < path d = "M16.85 11.25a10 10 0 0 1 2.22 1.68" /> < path d = "M5 13a10 10 0 0 1 5.24-2.76" /> < line x1 = "12" y1 = "20" x2 = "12.01" y2 = "20" /> < / > } } pub const LucideWifiOff : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;