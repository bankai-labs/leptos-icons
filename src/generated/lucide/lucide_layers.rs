use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polygon points = "12 2 2 7 12 12 22 7 12 2" /> < polyline points = "2 17 12 22 22 17" /> < polyline points = "2 12 12 17 22 12" /> < / > } } pub const LucideLayers : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;