use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2.5" /> < rect x = "14" y = "17" width = "8" height = "5" rx = "1" /> < path d = "M20 17v-2a2 2 0 1 0-4 0v2" /> < / > } } pub const LucideFolderLock : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;