use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> < rect x = "7" y = "7" width = "3" height = "9" /> < rect x = "14" y = "7" width = "3" height = "5" /> < / > } } pub const LucideTrello : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;