use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M15 11.25l1.5 1.5.75-.75V8.758l2.276-.61a3 3 0 10-3.675-3.675l-.61 2.277H12l-.75.75 1.5 1.5M15 11.25l-8.47 8.47c-.34.34-.8.53-1.28.53s-.94.19-1.28.53l-.97.97-.75-.75.97-.97c.34-.34.53-.8.53-1.28s.19-.94.53-1.28L12.75 9M15 11.25L12.75 9" /> < / > } } pub const HeroiconsOutlineEyeDropper : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;