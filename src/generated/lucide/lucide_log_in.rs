use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" /> < polyline points = "10 17 15 12 10 7" /> < line x1 = "15" y1 = "12" x2 = "3" y2 = "12" /> < / > } } pub const LucideLogIn : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;