use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> < polyline points = "11 3 11 11 14 8 17 11 17 3" /> < / > } } pub const LucideAlbum : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;