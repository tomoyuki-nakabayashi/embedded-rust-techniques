cmake_minimum_required(VERSION 3.8.2)

macro(set_conf_file)
  if(EXISTS ${APPLICATION_SOURCE_DIR}/prj_${BOARD}.conf)
    set(CONF_FILE "${APPLICATION_SOURCE_DIR}/prj_${BOARD}.conf")
  elseif(EXISTS ${APPLICATION_SOURCE_DIR}/boards/${BOARD}.conf)
    set(CONF_FILE
      "prj.conf ${APPLICATION_SOURCE_DIR}/boards/${BOARD}.conf")
  else()
    set(CONF_FILE "prj.conf")
  endif()
endmacro()

include($ENV{ZEPHYR_BASE}/cmake/app/boilerplate.cmake NO_POLICY_SCOPE)
project(hello)

FILE(GLOB app_sources src/*.c)
target_sources(app PRIVATE ${app_sources})

include($ENV{ZEPHYR_BASE}/samples/net/common/common.cmake)

# Include External Rust Library
include(ExternalProject)

set(rust_prj hello)
set(rust_src_dir   ${CMAKE_CURRENT_SOURCE_DIR}/${rust_prj})
set(rust_build_dir ${CMAKE_CURRENT_BINARY_DIR}/${rust_prj})

ExternalProject_Add(
  rust_project                 # Name for custom target
  PREFIX     ${rust_build_dir} # Root dir for entire project
  SOURCE_DIR ${rust_src_dir}
  BINARY_DIR ${rust_src_dir}   # This particular build system is invoked from the root
  CONFIGURE_COMMAND ""         # Skip configuring the project, e.g. with autoconf
  BUILD_COMMAND make
  INSTALL_COMMAND ""      # This particular build system has no install command
  BUILD_BYPRODUCTS ${rust_src_dir}/lib/librust.a
)

add_library(rust_lib STATIC IMPORTED GLOBAL)
add_dependencies(
  rust_lib
  rust_project
)

set_target_properties(rust_lib PROPERTIES IMPORTED_LOCATION ${rust_src_dir}/lib/librust.a)
set_target_properties(rust_lib PROPERTIES INTERFACE_INCLUDE_DIRECTORIES ${rust_src_dir}/lib)

target_link_libraries(app PUBLIC rust_lib)
