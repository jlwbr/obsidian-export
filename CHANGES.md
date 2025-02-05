# Changelog

## v21.9.1 (2021-09-24)

### Changes

* Treat SVG files as embeddable images. [Narayan Sainaney]

  This will ensure SVG files are included as an image when using `![[foo.svg]]` syntax, as opposed to only being linked to.

### Other

* Bump pulldown-cmark-to-cmark from 6.0.2 to 6.0.3. [dependabot[bot]]

  Bumps [pulldown-cmark-to-cmark](https://github.com/Byron/pulldown-cmark-to-cmark) from 6.0.2 to 6.0.3.
  - [Release notes](https://github.com/Byron/pulldown-cmark-to-cmark/releases)
  - [Changelog](https://github.com/Byron/pulldown-cmark-to-cmark/blob/main/CHANGELOG.md)
  - [Commits](https://github.com/Byron/pulldown-cmark-to-cmark/compare/v6.0.2...v6.0.3)

  ---
  updated-dependencies:
  - dependency-name: pulldown-cmark-to-cmark
    dependency-type: direct:production
    update-type: version-update:semver-patch
  ...

* Bump serde_yaml from 0.8.20 to 0.8.21. [dependabot[bot]]

  Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.20 to 0.8.21.
  - [Release notes](https://github.com/dtolnay/serde-yaml/releases)
  - [Commits](https://github.com/dtolnay/serde-yaml/compare/0.8.20...0.8.21)

  ---
  updated-dependencies:
  - dependency-name: serde_yaml
    dependency-type: direct:production
    update-type: version-update:semver-patch
  ...



## v21.9.0 (2021-09-12)

> This release switches to a [calendar versioning scheme](https://calver.org/overview.html).
> Details on this decision can be read in [switching obsidian-export to CalVer](https://nick.groenen.me/posts/switching-obsidian-export-to-calver/).

### New

* Support postprocessors running on embedded notes. [Nick Groenen]

  This introduces support for postprocessors that are run on the result of
  a note that is being embedded into another note. This differs from the
  existing postprocessors (which remain unchanged) that run once all
  embeds have been processed and merged with the final note.

  These "embed postprocessors" may be set through the new
  `Exporter::add_embed_postprocessor` method.

* Add start_at option to export a partial vault. [Nick Groenen]

  This introduces a new `--start-at` CLI argument and corresponding
  `start_at()` method on the Exporter type that allows exporting of only a
  given subdirectory within a vault.

  See the updated README file for more details on when and how this may be
  used.

### Other

* Don't build docs for the bin target. [Nick Groenen]

  The library contains documentation covering both CLI and library usage,
  there's no separate documentation for just the binary target.

* Move postprocessor tests into their own file for clarity. [Nick Groenen]

* Update indirect dependencies. [Nick Groenen]

* Bump serde_yaml from 0.8.19 to 0.8.20. [dependabot[bot]]

  Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.19 to 0.8.20.
  - [Release notes](https://github.com/dtolnay/serde-yaml/releases)
  - [Commits](https://github.com/dtolnay/serde-yaml/compare/0.8.19...0.8.20)

  ---
  updated-dependencies:
  - dependency-name: serde_yaml
    dependency-type: direct:production
    update-type: version-update:semver-patch
  ...

* Don't borrow references that are immediately dereferenced. [Nick Groenen]

  This was caught by a recently introduced clippy rule

* Bump serde_yaml from 0.8.17 to 0.8.19. [dependabot[bot]]

  Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.17 to 0.8.19.
  - [Release notes](https://github.com/dtolnay/serde-yaml/releases)
  - [Commits](https://github.com/dtolnay/serde-yaml/compare/0.8.17...0.8.19)

  ---
  updated-dependencies:
  - dependency-name: serde_yaml
    dependency-type: direct:production
    update-type: version-update:semver-patch
  ...

* Update dependencies. [Nick Groenen]

* Fix 4 new clippy lints. [Nick Groenen]

* Bump regex from 1.4.6 to 1.5.3. [dependabot[bot]]

  Bumps [regex](https://github.com/rust-lang/regex) from 1.4.6 to 1.5.3.
  - [Release notes](https://github.com/rust-lang/regex/releases)
  - [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)
  - [Commits](https://github.com/rust-lang/regex/compare/1.4.6...1.5.3)

* Bump pretty_assertions from 0.7.1 to 0.7.2. [dependabot[bot]]

  Bumps [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions) from 0.7.1 to 0.7.2.
  - [Release notes](https://github.com/colin-kiegel/rust-pretty-assertions/releases)
  - [Changelog](https://github.com/colin-kiegel/rust-pretty-assertions/blob/main/CHANGELOG.md)
  - [Commits](https://github.com/colin-kiegel/rust-pretty-assertions/compare/v0.7.1...v0.7.2)

* Bump regex from 1.4.5 to 1.4.6. [dependabot[bot]]

  Bumps [regex](https://github.com/rust-lang/regex) from 1.4.5 to 1.4.6.
  - [Release notes](https://github.com/rust-lang/regex/releases)
  - [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)
  - [Commits](https://github.com/rust-lang/regex/compare/1.4.5...1.4.6)

## v0.7.0 (2021-04-11)

### New

* Postprocessing support. [Nick Groenen]

  Add support for postprocessing of Markdown prior to writing converted
  notes to disk.

  Postprocessors may be used when making use of Obsidian export as a Rust
  library to do the following:

  1. Modify a note's `Context`, for example to change the destination
     filename or update its Frontmatter.
  2. Change a note's contents by altering `MarkdownEvents`.
  3. Prevent later postprocessors from running or cause a note to be
     skipped entirely.

  Future releases of Obsidian export may come with built-in postprocessors
  for users of the command-line tool to use, if general use-cases can be
  identified.

  For example, a future release might include functionality to make notes
  more suitable for the Hugo static site generator. This functionality
  would be implemented as a postprocessor that could be enabled through
  command-line flags.

### Fixes

* Also percent-encode `?` in filenames. [Nick Groenen]

  A recent Obsidian update expanded the list of allowed characters in
  filenames, which now includes `?` as well. This needs to be
  percent-encoded for proper links in static site generators like Hugo.

### Other

* Bump pretty_assertions from 0.6.1 to 0.7.1. [dependabot[bot]]

  Bumps [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions) from 0.6.1 to 0.7.1.
  - [Release notes](https://github.com/colin-kiegel/rust-pretty-assertions/releases)
  - [Changelog](https://github.com/colin-kiegel/rust-pretty-assertions/blob/main/CHANGELOG.md)
  - [Commits](https://github.com/colin-kiegel/rust-pretty-assertions/compare/v0.6.1...v0.7.1)

* Bump walkdir from 2.3.1 to 2.3.2. [dependabot[bot]]

  Bumps [walkdir](https://github.com/BurntSushi/walkdir) from 2.3.1 to 2.3.2.
  - [Release notes](https://github.com/BurntSushi/walkdir/releases)
  - [Commits](https://github.com/BurntSushi/walkdir/compare/2.3.1...2.3.2)

* Bump regex from 1.4.3 to 1.4.5. [dependabot[bot]]

  Bumps [regex](https://github.com/rust-lang/regex) from 1.4.3 to 1.4.5.
  - [Release notes](https://github.com/rust-lang/regex/releases)
  - [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)
  - [Commits](https://github.com/rust-lang/regex/compare/1.4.3...1.4.5)

## v0.6.0 (2021-02-15)

### New

* Add `--version` flag. [Nick Groenen]

### Changes

* Don't Box FilterFn in WalkOptions. [Nick Groenen]

  Previously, `filter_fn` on the `WalkOptions` struct looked like:

      pub filter_fn: Option<Box<&'static FilterFn>>,

  This boxing was unneccesary and has been changed to:

      pub filter_fn: Option<&'static FilterFn>,

  This will only affect people who use obsidian-export as a library in
  other Rust programs, not users of the CLI.

  For those library users, they no longer need to supply `FilterFn`
  wrapped in a Box.

### Fixes

* Recognize notes beginning with underscores. [Nick Groenen]

  Notes with an underscore would fail to be recognized within Obsidian
  `[[_WikiLinks]]` due to the assumption that the underlying Markdown
  parser (pulldown_cmark) would emit the text between `[[` and `]]` as
  a single event.

  The note parser has now been rewritten to use a more reliable state
  machine which correctly recognizes this corner-case (and likely some
  others).

* Support self-references. [Joshua Coles]

  This ensures links to headings within the same note (`[[#Heading]]`)
  resolve correctly.

### Other

* Avoid redundant "Release" in GitHub release titles. [Nick Groenen]

* Add failing testcase for files with underscores. [Nick Groenen]

* Add unit tests for display of ObsidianNoteReference. [Nick Groenen]

* Add some unit tests for ObsidianNoteReference::from_str. [Nick Groenen]

* Also run tests on pull requests. [Nick Groenen]

* Apply clippy suggestions following rust 1.50.0. [Nick Groenen]

* Fix infinite recursion bug with references to current file. [Joshua Coles]

* Add tests for self-references. [Joshua Coles]

  Note as there is no support for block references at the moment, the generated link goes nowhere, however it is to a reasonable ID

* Bump tempfile from 3.1.0 to 3.2.0. [dependabot[bot]]

  Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.1.0 to 3.2.0.
  - [Release notes](https://github.com/Stebalien/tempfile/releases)
  - [Changelog](https://github.com/Stebalien/tempfile/blob/master/NEWS)
  - [Commits](https://github.com/Stebalien/tempfile/commits)

* Bump eyre from 0.6.3 to 0.6.5. [dependabot[bot]]

  Bumps [eyre](https://github.com/yaahc/eyre) from 0.6.3 to 0.6.5.
  - [Release notes](https://github.com/yaahc/eyre/releases)
  - [Changelog](https://github.com/yaahc/eyre/blob/v0.6.5/CHANGELOG.md)
  - [Commits](https://github.com/yaahc/eyre/compare/v0.6.3...v0.6.5)

* Bump regex from 1.4.2 to 1.4.3. [dependabot[bot]]

  Bumps [regex](https://github.com/rust-lang/regex) from 1.4.2 to 1.4.3.
  - [Release notes](https://github.com/rust-lang/regex/releases)
  - [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)
  - [Commits](https://github.com/rust-lang/regex/compare/1.4.2...1.4.3)



## v0.5.1 (2021-01-10)

### Fixes

* Find uppercased notes when referenced with lowercase. [Nick Groenen]

  This commit fixes a bug where, if a note contained uppercase characters
  (for example `Note.md`) but was referred to using lowercase
  (`[[note]]`), that note would not be found.



## v0.5.0 (2021-01-05)

### New

* Add --no-recursive-embeds to break infinite recursion cycles. [Nick Groenen]

  It's possible to end up with "recursive embeds" when two notes embed
  each other. This happens for example when a `Note A.md` contains
  `![[Note B]]` but `Note B.md` also contains `![[Note A]]`.

  By default, this will trigger an error and display the chain of notes
  which caused the recursion.

  Using the new `--no-recursive-embeds`, if a note is encountered for a
  second time while processing the original note, rather than embedding it
  again a link to the note is inserted instead to break the cycle.

  See also: https://github.com/zoni/obsidian-export/issues/1

* Make walk options configurable on CLI. [Nick Groenen]

  By default hidden files, patterns listed in `.export-ignore` as well as
  any files ignored by git are excluded from exports. This behavior has
  been made configurable on the CLI using the new flags `--hidden`,
  `--ignore-file` and `--no-git`.

* Support links referencing headings. [Nick Groenen]

  Previously, links referencing a heading (`[[note#heading]]`) would just
  link to the file name without including an anchor in the link target.
  Now, such references will include an appropriate `#anchor` attribute.

  Note that neither the original Markdown specification, nor the more
  recent CommonMark standard, specify how anchors should be constructed
  for a given heading.

  There are also some differences between the various Markdown rendering
  implementations.

  Obsidian-export uses the [slug] crate to generate anchors which should
  be compatible with most implementations, however your mileage may vary.

  (For example, GitHub may leave a trailing `-` on anchors when headings
  end with a smiley. The slug library, and thus obsidian-export, will
  avoid such dangling dashes).

  [slug]: https://crates.io/crates/slug

* Support embeds referencing headings. [Nick Groenen]

  Previously, partial embeds (`![[note#heading]]`) would always include
  the entire file into the source note. Now, such embeds will only include
  the contents of the referenced heading (and any subheadings).

  Links and embeds of [arbitrary blocks] remains unsupported at this time.

  [arbitrary blocks]: https://publish.obsidian.md/help/How+to/Link+to+blocks

### Changes

* Print warnings to stderr rather than stdout. [Nick Groenen]

  Warning messages emitted when encountering broken links/references will
  now be printed to stderr as opposed to stdout.

### Other

* Include filter_fn field in WalkOptions debug display. [Nick Groenen]



## v0.4.0 (2020-12-23)

### Fixes

* Correct relative links within embedded notes. [Nick Groenen]

  Links within an embedded note would point to other local resources
  relative to the filesystem location of the note being embedded.

  When a note inside a different directory would embed such a note, these
  links would point to invalid locations.

  Now these links are calculated relative to the top note, which ensures
  these links will point to the right path.

### Other

* Add brief library documentation to all public types and functions. [Nick Groenen]



## v0.3.0 (2020-12-21)

### New

* Report file tree when RecursionLimitExceeded is hit. [Nick Groenen]

  This refactors the Context to maintain a list of all the files which
  have been processed so far in a chain of embeds. This information is
  then used to print a more helpful error message to users of the CLI when
  RecursionLimitExceeded is returned.

### Changes

* Add extra whitespace around multi-line warnings. [Nick Groenen]

  This makes errors a bit easier to distinguish after a number of warnings
  has been printed.

### Other

* Setup gitchangelog. [Nick Groenen]

  This adds a changelog (CHANGES.md) which is automatically generated with
  [gitchangelog].

  [gitchangelog]: https://github.com/vaab/gitchangelog



## v0.2.0 (2020-12-13)

* Allow custom filter function to be passed with WalkOptions. [Nick Groenen]

* Re-export vault_contents and WalkOptions as pub from crate root. [Nick Groenen]

* Run mdbook hook against README.md too. [Nick Groenen]

* Update installation instructions. [Nick Groenen]

  Installation no longer requires a git repository URL now that a crate is
  published.

* Add MdBook generation script and precommit hook. [Nick Groenen]

* Add more reliable non-ASCII tetscase. [Nick Groenen]

* Create FUNDING.yml. [Nick Groenen]

## v0.1.0 (2020-11-28)

* Public release. [Nick Groenen]
