use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m4 4 7.07 17 2.51-7.39L21 11.07z" /> < / > } } pub const LucideMousePointer2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;