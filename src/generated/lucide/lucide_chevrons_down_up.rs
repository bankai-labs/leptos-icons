use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m7 20 5-5 5 5" /> < path d = "m7 4 5 5 5-5" /> < / > } } pub const LucideChevronsDownUp : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;