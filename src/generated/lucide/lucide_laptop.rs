use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16" /> < / > } } pub const LucideLaptop : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;