use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M18 7c0-5.333-8-5.333-8 0" /> < path d = "M10 7v14" /> < path d = "M6 21h12" /> < path d = "M6 13h10" /> < / > } } pub const LucidePoundSterling : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;