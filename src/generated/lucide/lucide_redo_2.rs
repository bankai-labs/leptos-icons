use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m15 14 5-5-5-5" /> < path d = "M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13" /> < / > } } pub const LucideRedo2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;