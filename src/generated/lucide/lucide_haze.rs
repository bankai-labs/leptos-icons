use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m5.2 6.2 1.4 1.4" /> < path d = "M2 13h2" /> < path d = "M20 13h2" /> < path d = "m17.4 7.6 1.4-1.4" /> < path d = "M22 17H2" /> < path d = "M22 21H2" /> < path d = "M16 13a4 4 0 0 0-8 0" /> < path d = "M12 5V2.5" /> < / > } } pub const LucideHaze : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;