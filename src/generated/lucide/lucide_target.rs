use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "10" /> < circle cx = "12" cy = "12" r = "6" /> < circle cx = "12" cy = "12" r = "2" /> < / > } } pub const LucideTarget : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;