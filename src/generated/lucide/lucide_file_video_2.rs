use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M4 8V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4" /> < polyline points = "14 2 14 8 20 8" /> < path d = "m10 15.5 4 2.5v-6l-4 2.5" /> < rect x = "2" y = "12" width = "8" height = "6" rx = "1" /> < / > } } pub const LucideFileVideo2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;