use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m3 12 7-7v4h11v6H10v4z" /> < / > } } pub const LucideArrowBigLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;