use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "15 3 21 3 21 9" /> < polyline points = "9 21 3 21 3 15" /> < line x1 = "21" y1 = "3" x2 = "14" y2 = "10" /> < line x1 = "3" y1 = "21" x2 = "10" y2 = "14" /> < / > } } pub const LucideMaximize2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;