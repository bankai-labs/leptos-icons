use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "2" /> < path d = "M4.93 19.07a10 10 0 0 1 0-14.14" /> < path d = "M7.76 16.24a6 6 0 0 1-1.3-1.95 6 6 0 0 1 0-4.59 6 6 0 0 1 1.3-1.95" /> < path d = "M16.24 7.76a6 6 0 0 1 1.3 2 6 6 0 0 1 0 4.59 6 6 0 0 1-1.3 1.95" /> < path d = "M19.07 4.93a10 10 0 0 1 0 14.14" /> < / > } } pub const LucideRadio : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;