use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "9 14 4 9 9 4" /> < path d = "M20 20v-7a4 4 0 0 0-4-4H4" /> < / > } } pub const LucideCornerUpLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;