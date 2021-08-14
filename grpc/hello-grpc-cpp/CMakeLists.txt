cmake_minimum_required(VERSION 3.5.1)
project(Hello-GRPC-CPP C CXX)

set(CMAKE_PREFIX_PATH ${CMAKE_PREFIX_PATH} "/Users/han/.local")

include(./cmake/common.cmake)

# Proto file
get_filename_component(hello_proto "protos/helloworld.proto" ABSOLUTE)
get_filename_component(hello_proto_path "${hello_proto}" PATH)

# Generated sources
set(PB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/common")
set(hello_proto_srcs "${PB_PATH}/helloworld.pb.cc")
set(hello_proto_hdrs "${PB_PATH}/helloworld.pb.h")
set(hello_grpc_srcs "${PB_PATH}/helloworld.grpc.pb.cc")
set(hello_grpc_hdrs "${PB_PATH}/helloworld.grpc.pb.h")

add_custom_command(
        OUTPUT "${hello_proto_srcs}" "${hello_proto_hdrs}" "${hello_grpc_srcs}" "${hello_grpc_hdrs}"
        COMMAND ${_PROTOBUF_PROTOC}
        ARGS --grpc_out "${PB_PATH}"
        --cpp_out "${PB_PATH}"
        -I "${hello_proto_path}"
        --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
        "${hello_proto}"
        DEPENDS "${hello_proto}")

# Include generated *.pb.h files
include_directories("${PB_PATH}")

# hello_grpc_proto
add_library(hello_grpc_proto
        ${hello_grpc_srcs}
        ${hello_grpc_hdrs}
        ${hello_proto_srcs}
        ${hello_proto_hdrs})
target_link_libraries(hello_grpc_proto
        ${_REFLECTION}
        ${_GRPC_GRPCPP}
        ${_PROTOBUF_LIBPROTOBUF})

# Targets [async_](client|server)
#https://cmake.org/cmake/help/latest/command/add_executable.html
add_executable(proto_client "client/proto_client.cpp")
add_executable(greeter_async_client "client/greeter_async_client.cc")
add_executable(greeter_async_client2 "client/greeter_async_client2.cc")
add_executable(proto_server "server/proto_server.cpp")
add_executable(greeter_async_server "server/greeter_async_server.cc")

#https://cmake.org/cmake/help/latest/command/target_link_libraries.html
target_link_libraries(proto_client hello_grpc_proto ${_REFLECTION} ${_GRPC_GRPCPP} ${_PROTOBUF_LIBPROTOBUF})
target_link_libraries(greeter_async_client hello_grpc_proto ${_REFLECTION} ${_GRPC_GRPCPP} ${_PROTOBUF_LIBPROTOBUF})
target_link_libraries(greeter_async_client2 hello_grpc_proto ${_REFLECTION} ${_GRPC_GRPCPP} ${_PROTOBUF_LIBPROTOBUF})
target_link_libraries(proto_server hello_grpc_proto ${_REFLECTION} ${_GRPC_GRPCPP} ${_PROTOBUF_LIBPROTOBUF})
target_link_libraries(greeter_async_server hello_grpc_proto ${_REFLECTION} ${_GRPC_GRPCPP} ${_PROTOBUF_LIBPROTOBUF})