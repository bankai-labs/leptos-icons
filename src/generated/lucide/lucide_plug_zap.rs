use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m13 2-2 2.5h3L12 7" /> < path d = "M12 22v-3" /> < path d = "M10 13v-2.5" /> < path d = "M10 12.5v-2" /> < path d = "M14 12.5v-2" /> < path d = "M16 15a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-2z" /> < / > } } pub const LucidePlugZap : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;