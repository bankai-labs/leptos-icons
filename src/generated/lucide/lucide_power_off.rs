use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M18.36 6.64A9 9 0 0 1 20.77 15" /> < path d = "M6.16 6.16a9 9 0 1 0 12.68 12.68" /> < path d = "M12 2v4" /> < path d = "m2 2 20 20" /> < / > } } pub const LucidePowerOff : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;