use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "3 8 7 12 3 16" /> < line x1 = "21" y1 = "12" x2 = "11" y2 = "12" /> < line x1 = "21" y1 = "6" x2 = "11" y2 = "6" /> < line x1 = "21" y1 = "18" x2 = "11" y2 = "18" /> < / > } } pub const LucideIndent : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;