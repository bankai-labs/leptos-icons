use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "16" cy = "4" r = "1" /> < path d = "m18 19 1-7-5.87.94" /> < path d = "m5 8 3-3 5.5 3-2.21 3.1" /> < path d = "M4.24 14.48c-.19.58-.27 1.2-.23 1.84a5 5 0 0 0 5.31 4.67c.65-.04 1.25-.2 1.8-.46" /> < path d = "M13.76 17.52c.19-.58.27-1.2.23-1.84a5 5 0 0 0-5.31-4.67c-.65.04-1.25.2-1.8.46" /> < / > } } pub const LucideAccessibility : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;