use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M17 22V2L7 7l10 5" /> < / > } } pub const LucideFlagTriangleLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;