use eyre::{eyre, Result};
use gumdrop::Options;
use obsidian_export::{ExportError, Exporter, FrontmatterStrategy, WalkOptions, PostprocessorResult};
use obsidian_export::serde_yaml::Value;
use std::{env, path::PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Options)]
struct Opts {
    #[options(help = "Display program help")]
    help: bool,

    #[options(help = "Display version information")]
    version: bool,

    #[options(help = "Read notes from this source", free, required)]
    source: Option<PathBuf>,

    #[options(help = "Write notes to this destination", free, required)]
    destination: Option<PathBuf>,

    #[options(no_short, help = "Only export notes under this sub-path")]
    start_at: Option<PathBuf>,

    #[options(
        help = "Frontmatter strategy (one of: always, never, auto)",
        no_short,
        long = "frontmatter",
        parse(try_from_str = "frontmatter_strategy_from_str"),
        default = "auto"
    )]
    frontmatter_strategy: FrontmatterStrategy,

    #[options(
        no_short,
        help = "Read ignore patterns from files with this name",
        default = ".export-ignore"
    )]
    ignore_file: String,

    #[options(no_short, help = "Export hidden files", default = "false")]
    hidden: bool,

    #[options(no_short, help = "Disable git integration", default = "false")]
    no_git: bool,

    #[options(no_short, help = "Don't process embeds recursively", default = "false")]
    no_recursive_embeds: bool,
    #[options(no_short, help = "Only convert files with publish: true in the frontmatter", default = "false")]
    only_publish: bool,
}

fn frontmatter_strategy_from_str(input: &str) -> Result<FrontmatterStrategy> {
    match input {
        "auto" => Ok(FrontmatterStrategy::Auto),
        "always" => Ok(FrontmatterStrategy::Always),
        "never" => Ok(FrontmatterStrategy::Never),
        _ => Err(eyre!("must be one of: always, never, auto")),
    }
}

fn main() {
    // Due to the use of free arguments in Opts, we must bypass Gumdrop to determine whether the
    // version flag was specified. Without this, "missing required free argument" would get printed
    // when no other args are specified.
    if env::args().any(|arg| arg == "-v" || arg == "--version") {
        println!("obsidian-export {}", VERSION);
        std::process::exit(0);
    }

    let args = Opts::parse_args_default_or_exit();
    let root = args.source.unwrap();
    let destination = args.destination.unwrap();

    let walk_options = WalkOptions {
        ignore_filename: &args.ignore_file,
        ignore_hidden: !args.hidden,
        honor_gitignore: !args.no_git,
        ..Default::default()
    };

    let mut exporter = Exporter::new(root, destination);
    exporter.frontmatter_strategy(args.frontmatter_strategy);
    exporter.process_embeds_recursively(!args.no_recursive_embeds);
    exporter.walk_options(walk_options);

    if args.only_publish {
        println!("Jaaaa");
        exporter.add_postprocessor(&|mut context, events| {
            // This is the key that we are searching for.
            let key = Value::String("publish".to_string());
            // This is the value that we are searching for.
            let value = Some(&Value::Bool(true));

            // Check if key equals value
            if context.frontmatter.get(&key) == value {
                return (context, events, PostprocessorResult::Continue)
            }

            (context, events, PostprocessorResult::StopAndSkipNote)
        });
    }
    

    if let Some(path) = args.start_at {
        exporter.start_at(path);
    }

    if let Err(err) = exporter.run() {
        match err {
            ExportError::FileExportError {
                ref path,
                ref source,
            } => match &**source {
                // An arguably better way of enhancing error reports would be to construct a custom
                // `eyre::EyreHandler`, but that would require a fair amount of boilerplate and
                // reimplementation of basic reporting.
                ExportError::RecursionLimitExceeded { file_tree } => {
                    eprintln!(
                        "Error: {:?}",
                        eyre!(
                            "'{}' exceeds the maximum nesting limit of embeds",
                            path.display()  
                        )
                    );
                    eprintln!("\nFile tree:");
                    for (idx, path) in file_tree.iter().enumerate() {
                        eprintln!("  {}-> {}", "  ".repeat(idx), path.display());
                    }
                    eprintln!("\nHint: Ensure notes are non-recursive, or specify --no-recursive-embeds to break cycles")
                }
                _ => eprintln!("Error: {:?}", eyre!(err)),
            },
            _ => eprintln!("Error: {:?}", eyre!(err)),
        };
        std::process::exit(1);
    };
}
