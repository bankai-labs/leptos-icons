use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m12 15 3.5-3.5" /> < path d = "M20.3 18c.4-1 .7-2.2.7-3.4C21 9.8 17 6 12 6s-9 3.8-9 8.6c0 1.2.3 2.4.7 3.4" /> < / > } } pub const LucideGauge : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;