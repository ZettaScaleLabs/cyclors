cmake_minimum_required(VERSION 3.12)
project(Objcopy)

if(${CMAKE_SYSTEM_NAME} STREQUAL "Linux")
    # Include the CMakeFindBinUtils module to find objcopy
    include(CMakeFindBinUtils)
else()
    find_program(CMAKE_OBJCOPY llvm-objcopy)
endif()

# Ensure objcopy is available
if(NOT CMAKE_OBJCOPY)
    message(FATAL_ERROR "CMAKE_OBJCOPY is not defined. Ensure objcopy is installed on your system!")
else()
    message(STATUS "CMAKE_OBJCOPY found: ${CMAKE_OBJCOPY}")
endif()

# Ensure LIB_PATH is defined and exists
if(NOT DEFINED LIB_PATH)
    message(FATAL_ERROR "LIB_PATH not specified!")
else()
    if(NOT EXISTS ${LIB_PATH})
        message(FATAL_ERROR "Library not found: ${LIB_PATH}")
    else()
        message(STATUS "LIB_PATH: ${LIB_PATH}")
    endif()
endif()

# Ensure SYMBOL_FILE_PATH is defined and exists
if(NOT DEFINED SYMBOL_FILE_PATH)
    message(FATAL_ERROR "SYMBOL_FILE_PATH not specified!")
else()
    if(NOT EXISTS ${SYMBOL_FILE_PATH})
        message(FATAL_ERROR "Symbol file not found: ${SYMBOL_FILE_PATH}")
    else()
        message(STATUS "SYMBOL_FILE_PATH: ${SYMBOL_FILE_PATH}")
    endif()
endif()

# Custom target to mangle the library
add_custom_target(mangle_library ALL
    # Run objcopy and redirect stderr to a file
    COMMAND ${CMAKE_COMMAND} -E echo "Running objcopy --redefine-syms on ${LIB_PATH} with symbols from ${SYMBOL_FILE_PATH}..."
    COMMAND ${CMAKE_OBJCOPY} --redefine-syms=${SYMBOL_FILE_PATH} ${LIB_PATH} 2> ${LIB_PATH}.objcopy.stderr
)
