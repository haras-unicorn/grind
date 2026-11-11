cmake_minimum_required(VERSION 3.28)

project(grind LANGUAGES CXX)

option(GRIND_TESTS "Build and register the test binary of every problem." ON)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

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
      PRIVATE ${GRIND_DEFINITIONS} GRIND_TEST
    )
    target_link_options(${problem}-test PRIVATE ${GRIND_SANITIZERS})
    target_link_libraries(${problem}-test PRIVATE doctest::doctest)
    add_test(NAME ${problem} COMMAND ${problem}-test)
  endif()
endforeach()
