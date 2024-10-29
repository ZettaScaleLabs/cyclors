cmake_minimum_required(VERSION 3.12)
project(NM)

# Include the CMakeFindBinUtils module to find nm
include(CMakeFindBinUtils)

# Ensure nm is available
if(NOT CMAKE_NM)
    message(FATAL_ERROR "CMAKE_NM is not defined. Ensure nm is installed on your system!")
else()
    message(STATUS "CMAKE_NM found: ${CMAKE_NM}")
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

# Custom target to run nm on the library
add_custom_target(read_symbols ALL
    COMMAND ${CMAKE_COMMAND} -E echo "Running nm on ${LIB_PATH}..."
    
    # Run nm and redirect stderr to a file
    COMMAND ${CMAKE_NM} --defined-only --print-file-name ${LIB_PATH} 1> ${LIB_PATH}.nm 2> ${LIB_PATH}.stderr

    # Check if stderr is empty (i.e., no errors were produced)
    COMMAND ${CMAKE_COMMAND} -E echo "Checking for nm errors..."
    COMMAND ${CMAKE_CURRENT_SOURCE_DIR}/check_errors.sh ${LIB_PATH}.stderr
   
    # Clean up stderr.txt after checking for errors
    COMMAND ${CMAKE_COMMAND} -E remove ${LIB_PATH}.stderr
    COMMENT "Reading library symbols with nm..."
)
