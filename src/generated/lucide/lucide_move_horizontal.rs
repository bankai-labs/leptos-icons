use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "18 8 22 12 18 16" /> < polyline points = "6 8 2 12 6 16" /> < line x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> < / > } } pub const LucideMoveHorizontal : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;