use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1" /> < path d = "M7 22h1a4 4 0 0 0 4-4v-1" /> < path d = "M7 2h1a4 4 0 0 1 4 4v1" /> < / > } } pub const LucideTextCursor : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;