use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m18 16 4-4-4-4" /> < path d = "m6 8-4 4 4 4" /> < path d = "m14.5 4-5 16" /> < / > } } pub const LucideCode2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;