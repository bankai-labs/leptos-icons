use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" /> < polyline points = "14 2 14 8 20 8" /> < circle cx = "12" cy = "15" r = "2" /> < path d = "M12 12v1" /> < path d = "M12 17v1" /> < path d = "m14.6 13.5-.87.5" /> < path d = "m10.27 16-.87.5" /> < path d = "m14.6 16.5-.87-.5" /> < path d = "m10.27 14-.87-.5" /> < / > } } pub const LucideFileCog2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;