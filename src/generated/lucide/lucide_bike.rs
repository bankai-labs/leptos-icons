use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "5.5" cy = "17.5" r = "3.5" /> < circle cx = "18.5" cy = "17.5" r = "3.5" /> < path d = "M15 6a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm-3 11.5V14l-3-3 4-3 2 3h2" /> < / > } } pub const LucideBike : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;