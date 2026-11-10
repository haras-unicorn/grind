# Copies the template of a language into a new problem directory.
def main [
  language: string # The language of the solution, like rust, python or cpp.
  name: string # The name of the problem in kebab case, like aoc-2025-day-11.
] {
  let templates = $env.GRIND_TEMPLATES | from json
  let languages = $templates | columns

  if $language not-in $languages {
    let expected = $languages | str join ", "
    error make { msg: $"unknown language ($language), expected one of: ($expected)" }
  }

  if $name !~ '^[a-z0-9]+(-[a-z0-9]+)*$' {
    error make { msg: $"($name) is not in kebab case, like aoc-2025-day-11" }
  }

  let template = $templates | get $language
  let target = $template.target | path join $name

  if ($target | path exists) {
    error make { msg: $"($target) already exists" }
  }

  cp --recursive $template.source $target
  $"# ($name)\n" | save --force ($target | path join README.md)

  if $language == rust {
    rename-crate $target $name
  }

  print $"created ($target)"
}

# Renames the package in the manifest of a crate copied from the template.
def rename-crate [crate: string, name: string] {
  let manifest = $crate | path join Cargo.toml

  open --raw $manifest
  | str replace --regex '(?m)^name = ".*"$' $'name = "($name)"'
  | save --force $manifest
}
