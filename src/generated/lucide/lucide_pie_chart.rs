use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M21.21 15.89A10 10 0 1 1 8 2.83" /> < path d = "M22 12A10 10 0 0 0 12 2v10z" /> < / > } } pub const LucidePieChart : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;