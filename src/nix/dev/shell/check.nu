# Checks one problem with the lint, type, test and spelling steps of the checks
# of the whole repository, but only for that problem. It does not check the
# formatting, because `dev format` does that for everything quickly.
def main [
  language: string # The language of the solution, like rust, python or cpp.
  name: string # The name of the problem, like aoc-2025-day-11.
] {
  let languages = $env.GRIND_TEMPLATES | from json
  let known = $languages | columns

  if $language not-in $known {
    let expected = $known | str join ", "
    error make { msg: $"unknown language ($language), expected one of: ($expected)" }
  }

  let problem = $languages | get $language | get target | path join $name

  if not ($problem | path exists) {
    error make { msg: $"($problem) does not exist" }
  }

  match $language {
    "rust" => { check-rust $name }
    "python" => { check-python $problem }
    "cpp" => { check-cpp $name }
  }

  run "cspell" ["lint" $problem "--no-progress"]
}

def check-rust [name: string] {
  run "cargo" ["clippy" "--package" $name "--all-targets" "--" "-D" "warnings"]
  run "cargo" ["test" "--package" $name]
}

def check-python [problem: string] {
  run "ruff" ["check" $problem]
  run "basedpyright" [$problem]
  run "python" ["-m" "pytest" $problem]
}

# Builds in its own directory, so the warnings as errors do not stick to the
# build directory used for solving.
def check-cpp [name: string] {
  let build = $env.GRIND_CHECK_BUILD

  run "cmake" ["-S" "." "-B" $build "-G" "Ninja" "-DCMAKE_COMPILE_WARNING_AS_ERROR=ON"]
  run "cmake" ["--build" $build "--target" $name $"($name)-test"]
  run "ctest" ["--test-dir" $build "-R" $"^($name)$" "--output-on-failure"]
}

# An external command does not stop the script when it fails, so this does.
def run [program: string, arguments: list<string>] {
  print $"==> ($program) ($arguments | str join ' ')"

  ^$program ...$arguments

  if $env.LAST_EXIT_CODE != 0 {
    error make { msg: $"($program) failed with exit code ($env.LAST_EXIT_CODE)" }
  }
}
