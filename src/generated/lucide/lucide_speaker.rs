use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "4" y = "2" width = "16" height = "20" rx = "2" ry = "2" /> < circle cx = "12" cy = "14" r = "4" /> < line x1 = "12" y1 = "6" x2 = "12.01" y2 = "6" /> < / > } } pub const LucideSpeaker : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;