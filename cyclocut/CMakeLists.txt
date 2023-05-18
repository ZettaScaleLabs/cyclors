cmake_minimum_required(VERSION 3.0.0)
project(cdds-util VERSION 0.1.6)

include(CTest)
enable_testing()

include_directories(./include $ENV{CYCLONE_INCLUDE})
link_directories($ENV{CYCLONE_LIB})

if (BUILD_SHARED_LIBS OR NOT DEFINED BUILD_SHARED_LIBS)
    add_library(cdds-util SHARED src/cdds_blob.c)
else()
    add_library(cdds-util src/cdds_blob.c)
endif()

target_link_libraries(cdds-util PUBLIC ddsc)

add_compile_definitions(DEBUG)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

install(TARGETS cdds-util DESTINATION lib)
install(DIRECTORY include/cdds DESTINATION include)
