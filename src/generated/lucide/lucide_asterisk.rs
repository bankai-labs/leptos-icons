use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M12 6v12" /> < path d = "M17.196 9 6.804 15" /> < path d = "m6.804 9 10.392 6" /> < / > } } pub const LucideAsterisk : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;