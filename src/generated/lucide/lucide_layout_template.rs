use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M21 3H3v7h18V3z" /> < path d = "M21 14h-5v7h5v-7z" /> < path d = "M12 14H3v7h9v-7z" /> < / > } } pub const LucideLayoutTemplate : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;