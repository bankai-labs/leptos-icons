use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M21 12a9 9 0 1 1-6.219-8.56" /> < / > } } pub const LucideLoader2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;