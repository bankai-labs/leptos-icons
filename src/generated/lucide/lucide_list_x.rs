use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M11 12H3" /> < path d = "M16 6H3" /> < path d = "M16 18H3" /> < path d = "m19 10-4 4" /> < path d = "m15 10 4 4" /> < / > } } pub const LucideListX : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;