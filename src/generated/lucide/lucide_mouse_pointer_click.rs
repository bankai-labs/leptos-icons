use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m9 9 5 12 1.774-5.226L21 14 9 9z" /> < path d = "m16.071 16.071 4.243 4.243" /> < path d = "m7.188 2.239.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656-2.12 2.122" /> < / > } } pub const LucideMousePointerClick : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;