use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "10" /> < line x1 = "4.93" y1 = "4.93" x2 = "19.07" y2 = "19.07" /> < / > } } pub const LucideSlash : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;