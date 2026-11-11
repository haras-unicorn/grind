cmake_minimum_required(VERSION 3.28)

project(grind LANGUAGES CXX)

option(GRIND_TESTS "Build and register the test binary of every problem." ON)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# the solutions do not use modules, and scanning for them adds gcc flags to the
# compile commands that clangd does not know
set(CMAKE_CXX_SCAN_FOR_MODULES OFF)

set(GRIND_WARNINGS
  -Wall
  -Wextra
  -Wpedantic
  -Wconversion
  -Wsign-conversion
  -Wshadow
  -Wold-style-cast
  -Wcast-align
  -Wnull-dereference
  -Wdouble-promotion
  -Wformat=2
  -Wimplicit-fallthrough
  -Wmissing-declarations
)

# safety over speed: the standard library checks its bounds in every binary
set(GRIND_DEFINITIONS _GLIBCXX_ASSERTIONS)

set(GRIND_SANITIZERS
  -fsanitize=address,undefined
  -fno-sanitize-recover=all
  -fno-omit-frame-pointer
)

if(GRIND_TESTS)
  find_package(doctest REQUIRED)
  enable_testing()
endif()

# every directory in src/cpp is a problem with its solution in main.cpp
file(GLOB GRIND_PROBLEMS
  LIST_DIRECTORIES true
  CONFIGURE_DEPENDS
  "${PROJECT_SOURCE_DIR}/src/cpp/*"
)

foreach(problem_dir IN LISTS GRIND_PROBLEMS)
  if(NOT IS_DIRECTORY "${problem_dir}")
    continue()
  endif()

  cmake_path(GET problem_dir FILENAME problem)
  set(source "${problem_dir}/main.cpp")

  # NOTE: clangd takes the first compile command of a file, so this one comes
  # before the others and makes the editor see the tests next to the main. It is
  # never built: it has the tests but not the doctest main, so it would not link
  if(GRIND_TESTS)
    add_library(${problem}-editor OBJECT EXCLUDE_FROM_ALL ${source})
    target_compile_options(${problem}-editor PRIVATE ${GRIND_WARNINGS})
    target_compile_definitions(${problem}-editor
      PRIVATE ${GRIND_DEFINITIONS} GRIND_TEST
    )
    target_link_libraries(${problem}-editor PRIVATE doctest::doctest)
  endif()

  add_executable(${problem} ${source})
  target_compile_options(${problem} PRIVATE ${GRIND_WARNINGS})
  target_compile_definitions(${problem} PRIVATE ${GRIND_DEFINITIONS})
  install(TARGETS ${problem})

  if(GRIND_TESTS)
    add_executable(${problem}-test ${source})
    target_compile_options(${problem}-test
      PRIVATE ${GRIND_WARNINGS} ${GRIND_SANITIZERS}
    )
    target_compile_definitions(${problem}-test
      PRIVATE ${GRIND_DEFINITIONS} GRIND_TEST DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
    )
    target_link_options(${problem}-test PRIVATE ${GRIND_SANITIZERS})
    target_link_libraries(${problem}-test PRIVATE doctest::doctest)
    add_test(NAME ${problem} COMMAND ${problem}-test)
  endif()
endforeach()
