use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "9" r = "1" /> < circle cx = "19" cy = "9" r = "1" /> < circle cx = "5" cy = "9" r = "1" /> < circle cx = "12" cy = "15" r = "1" /> < circle cx = "19" cy = "15" r = "1" /> < circle cx = "5" cy = "15" r = "1" /> < / > } } pub const LucideGripHorizontal : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;