use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M2 20h.01" /> < path d = "M7 20v-4" /> < path d = "M12 20v-8" /> < / > } } pub const LucideSignalMedium : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;