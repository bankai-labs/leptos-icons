use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "7" y = "9" width = "10" height = "6" rx = "2" /> < path d = "M22 20H2" /> < path d = "M22 4H2" /> < / > } } pub const LucideAlignVerticalSpaceAround : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;