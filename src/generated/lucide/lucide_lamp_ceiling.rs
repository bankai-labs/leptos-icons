use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M12 2v5" /> < path d = "M6 7h12l4 9H2l4-9Z" /> < path d = "M9.17 16a3 3 0 1 0 5.66 0" /> < / > } } pub const LucideLampCeiling : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;