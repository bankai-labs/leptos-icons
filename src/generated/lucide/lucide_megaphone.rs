use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m3 11 18-5v12L3 14v-3z" /> < path d = "M11.6 16.8a3 3 0 1 1-5.8-1.6" /> < / > } } pub const LucideMegaphone : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;