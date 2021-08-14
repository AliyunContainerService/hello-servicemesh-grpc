#include <iostream>
#include <memory>
#include <string>

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

#include "helloworld.grpc.pb.h"
#include "landing.grpc.pb.h"

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::Status;
using helloworld::Greeter;
using helloworld::HelloReply;
using helloworld::HelloRequest;
using org::feuyeux::grpc::LandingService;
using org::feuyeux::grpc::TalkRequest;
using org::feuyeux::grpc::TalkResponse;
using org::feuyeux::grpc::TalkResult;
using org::feuyeux::grpc::ResultType;
using grpc::ServerWriter;
using grpc::ServerReader;
using grpc::ServerReaderWriter;

class GreeterServiceImpl final : public Greeter::Service {
    Status SayHello(ServerContext *context, const HelloRequest *request, HelloReply *reply) override {
        std::string prefix("Hello ");
        reply->set_message(prefix + request->name());
        return Status::OK;
    }
};

class LandingServiceImpl final : public LandingService::Service {
    Status Talk(ServerContext *context, const TalkRequest *request, TalkResponse *response) override {
        std::cout << "Talk received: " << request->data() << std::endl;
        response->set_status(200);
        TalkResult* talkResult;
        talkResult=response->add_results();
        talkResult->set_id(1);
        talkResult->set_type(ResultType::OK);
        return Status::OK;
    }

    Status
    TalkOneAnswerMore(ServerContext *context, const TalkRequest *request, ServerWriter<TalkResponse> *writer) override {
        return Status::OK;
    }

    Status
    TalkMoreAnswerOne(ServerContext *context, ServerReader<TalkRequest> *reader, TalkResponse *response) override {
        return Status::OK;
    }

    Status TalkBidirectional(ServerContext *context, ServerReaderWriter<TalkResponse, TalkRequest> *stream) override {
        return Status::OK;
    }
};

void RunServer() {
    std::string server_address("0.0.0.0:50051");

    grpc::EnableDefaultHealthCheckService(true);
    grpc::reflection::InitProtoReflectionServerBuilderPlugin();
    ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());

    GreeterServiceImpl service;
    builder.RegisterService(&service);

    LandingServiceImpl landingService;
    builder.RegisterService(&landingService);

    std::unique_ptr<Server> server(builder.BuildAndStart());
    std::cout << "Server listening on " << server_address << std::endl;
    server->Wait();
}

int main(int argc, char **argv) {
    RunServer();

    return 0;
}
