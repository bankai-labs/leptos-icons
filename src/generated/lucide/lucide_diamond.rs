use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "12" y = "1" width = "15.56" height = "15.56" rx = "2.41" transform = "rotate(45 12 1)" /> < / > } } pub const LucideDiamond : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;