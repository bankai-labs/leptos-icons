# leptos-icons
This crate makes it easy to use [SVG icons](#) from the following collections (more may be added in the future):

- TODO: [Bootstrap](https://icons.getbootstrap.com/) - MIT License
- TODO: [Font Awesome](https://fontawesome.com/icons) - CC BY 4.0 License
  - Regular
  - Solid
- [Heroicons](https://github.com/tailwindlabs/heroicons) - MIT License
  - Solid
  - Outline
  - Mini Solid
- TODO: [Lipis Flag Icons](https://github.com/lipis/flag-icons)* - MIT License
- TODO: [Lucide](https://github.com/lucide-icons/lucide) - ISC License
- TODO: [Octicons](https://primer.style/octicons/) - MIT License

## Usage

<!--Use the [gallery]() to find icons you like, and add them as feature flags.-->

```toml
[dependencies]
leptos_icons = { git = "https://github.com/bankai-labs/leptos-icons", features = [
    "HeroiconsOutlineFolder",
    "HeroiconsOutlineChevronRight",
    "HeroiconsOutlineEllipsisVertical"
] }
```

Then, add an `<Icon>` component with the corresponding path.

```rust
use leptos_icons::{Icon, HeroiconsOutlineFolder, HeroiconsOutlineChevronRight, HeroiconsOutlineEllipsisVertical};
view!{
    <>
        <Icon class="w-5 h-5 mr-3" path={HeroiconsOutlineFolder} />
        <Icon class=Signal::derive(cx, move || value() *2) path={HeroiconsOutlineChevronRight} />
        <Icon class="w-5 h-5 mr-3" stroke="2" path={HeroiconsOutlineEllipsisVertical} />
    </>
}
```

## Feature Flags

Each icon collection must be included with the corresponding feature flag, such as `lucide` or `font_awesome_solid`.

To save binary size, individual icons can also be included by feature flag, such as `LucideZoomIn` or `FontAwesomeSolidAtom`.

By default, no collections or icons are included. Be warned that including too many icons may result in a `.wasm` binary
that some WebAssembly engines refuse to load, especially in debug mode (see https://github.com/rustwasm/wasm-pack/issues/981).

## License

Code is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Icons are licensed by their respective creators (see above). An license summary is emitted to the DOM for each icon:
```html
<svg data-license="...original license...">
```
