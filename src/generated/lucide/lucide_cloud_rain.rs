use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" /> < path d = "M16 14v6" /> < path d = "M8 14v6" /> < path d = "M12 16v6" /> < / > } } pub const LucideCloudRain : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;