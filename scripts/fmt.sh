#!/usr/bin/env bash

input_fmt='markdown+abbreviations+autolink_bare_uris+markdown_attribute+mmd_header_identifiers+mmd_link_attributes+tex_math_double_backslash+emoji+task_lists'
output_fmt='markdown+raw_tex-native_spans-simple_tables-multiline_tables+emoji+task_lists'
ROOT="$(git rev-parse --show-toplevel)"

# Format markdown
find "$ROOT" -type f  -name '*.md' -exec pandoc -f "$input_fmt"  -t "$output_fmt"  --wrap=auto --atx-headers -o {} {} \;
# Format Rust
cargo +nightly fmt --all
