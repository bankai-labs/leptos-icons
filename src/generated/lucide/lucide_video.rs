use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m22 8-6 4 6 4V8Z" /> < rect x = "2" y = "6" width = "14" height = "12" rx = "2" ry = "2" /> < / > } } pub const LucideVideo : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;