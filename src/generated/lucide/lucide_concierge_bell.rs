use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M2 18a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2H2v-2Z" /> < path d = "M20 16a8 8 0 1 0-16 0" /> < path d = "M12 4v4" /> < path d = "M10 4h4" /> < / > } } pub const LucideConciergeBell : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;