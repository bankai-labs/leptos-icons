use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "5" width = "6" height = "14" rx = "2" /> < rect x = "15" y = "7" width = "6" height = "10" rx = "2" /> < path d = "M3 2v20" /> < path d = "M21 2v20" /> < / > } } pub const LucideAlignHorizontalSpaceBetween : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;