use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M21 3 9 15" /> < path d = "M12 3H3v18h18v-9" /> < path d = "M16 3h5v5" /> < path d = "M14 15H9v-5" /> < / > } } pub const LucideScaling : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;