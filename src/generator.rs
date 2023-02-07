use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::fs::{create_dir_all, read, read_dir, remove_dir_all, write};
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
    struct Feature {
        name: String,
        children: Vec<String>,
    }

    let mut variants = Vec::new();
    let mut features = Vec::new();
    let mut imports = Vec::new();

    let ssvg = Regex::new(r##"<svg[^>]+>"##).unwrap();
    let esvg = Regex::new(r##"</svg>"##).unwrap();

    remove_dir_all("src/generated").unwrap();
    create_dir_all("src/generated").unwrap();

    let mut generate = |prefix: &str, dir: &str, icon_type: &str| {
        let mut function_mods = Vec::new();

        let feature_name = prefix.to_case(Case::Snake);
        let feature_ident = to_ident(&feature_name);
        let mut collection_feature = Feature {
            name: feature_name.clone(),
            children: Vec::new(),
        };

        let result = read_dir(dir);
        let mut paths: Vec<_> = result
            .expect(dir)
            .map(|result| result.unwrap())
            .map(|entry| entry.path())
            .map(|path_buf| path_buf.to_str().unwrap().to_owned())
            .collect();

        paths.sort();

        create_dir_all(format!("src/generated/{}", feature_name)).unwrap();

        for path in paths {
            let file_name = path.split('/').last().unwrap();
            if !file_name.ends_with(".svg") {
                continue;
                //panic!("never happens?");
            }
            let icon_name = file_name.split('.').next().unwrap();
            let name = prefix.to_owned() + "-" + icon_name;

            let contents = read(&path).expect(&path);
            let svg = std::str::from_utf8(&contents).unwrap();

            let svg = ssvg.replace(&svg, "");
            let svg = esvg.replace(&svg, "");

            let svg_tokens = TokenStream::from_str(&svg).expect(&path);

            let function_name = name.to_case(Case::Snake);
            let function_ident = to_ident(&function_name);

            let variant_name = name.to_case(Case::UpperCamel);
            let variant = to_ident(&variant_name);
            variants.push(quote! {
                #[cfg(feature = #variant_name)]
                #variant
            });

            collection_feature.children.push(variant_name.clone());
            features.push(Feature {
                name: variant_name.clone(),
                children: Vec::new(),
            });


            //let icon_type = to_ident(&icon_type);

            // Don't need when export separate mods #[cfg(feature = #variant_name)]
            let tokens = quote! {
                use leptos::*;
                use crate::{IconType, Path};

                fn icon_path(cx: Scope) -> Fragment {
                    view! {cx,
                        <>
                            #svg_tokens
                        </>
                    }
                }

                pub const #variant: Path = Path {
                    path: icon_path,
                    icon_type: {},
                };
            };

            let output = tokens.to_string(); // reformat(tokens.to_string(), true).unwrap();
            let output = output.replace("{ }", icon_type);

            write(
                format!("src/generated/{}/{}.rs", feature_name, function_name),
                output,
            )
            .unwrap();

            function_mods.push(quote! {
                #[cfg(feature = #variant_name)]
                mod #function_ident;
                #[cfg(feature = #variant_name)]
                pub use #function_ident::*;
            });
        }

        let children: Vec<_> = collection_feature
            .children
            .iter()
            .map(|f| {
                quote! {
                    feature = #f
                }
            })
            .collect();

        imports.push(quote! {
            cfg_if::cfg_if! {
                if #[cfg(any(
                    #(#children),*
                ))] {
                    mod #feature_ident;
                    pub use #feature_ident::*;
                }
            }
        });

        features.push(collection_feature);

        let tokens = quote! {
            #(#function_mods)*
        };

        let output = reformat(tokens.to_string(), true).unwrap();
        write(format!("src/generated/{}.rs", feature_name), output).unwrap();
    };

    generate(
        "HeroiconsOutline",
        "heroicons/optimized/24/outline",
        "IconType::HeroIcons(crate::HeroIconsType::Outline)",
    );
    generate(
        "HeroiconsSolid",
        "heroicons/optimized/24/solid",
        "IconType::HeroIcons(crate::HeroIconsType::Solid)",
    );
    generate(
        "HeroiconsMiniSolid",
        "heroicons/optimized/20/solid",
        "IconType::HeroIcons(crate::HeroIconsType::Mini)",
    );
    generate(
        "Lucide",
        "lucide/icons",
        "IconType::HeroIcons(crate::HeroIconsType::Mini)",
    );

    let tokens = quote! {
        #(#imports)*
    };

    let output = reformat(tokens.to_string(), false).unwrap();

    write("src/generated.rs", output).unwrap();

    features.sort_unstable_by_key(|feature| feature.name.clone());

    for feature in features {
        println!(
            r##"{} = [{}]"##,
            feature.name,
            feature
                .children
                .into_iter()
                .map(|c| format!(r##""{}""##, c))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn to_ident(string: &str) -> Ident {
    Ident::new(string, Span::call_site())
}

// https://github.com/rust-analyzer/rust-analyzer/blob/ada9e16537c22b490d13cdd54b9e1e4885856a4c/xtask/src/codegen.rs#L66-L78
fn reformat(text: impl std::fmt::Display, included: bool) -> Result<String, String> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text).map_err(|e| e.to_string())?;
    let output = rustfmt.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let preamble = "Generated file, do not edit by hand, see `src/generator.rs`";
    let prefix = if included { "//" } else { "//!" };
    Ok(format!("{} {}\n\n{}", prefix, preamble, stdout))
}