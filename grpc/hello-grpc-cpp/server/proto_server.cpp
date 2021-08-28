#include <iostream>
#include <memory>
#include <string>

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

#include "helloworld.grpc.pb.h"
#include "landing.grpc.pb.h"
#include <glog/logging.h>

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
using std::string;
using google::protobuf::Map;

class LandingServiceImpl final : public LandingService::Service {
    std::vector<string> HELLO_LIST{"Hello", "Bonjour", "Hola", "こんにちは", "Ciao","안녕하세요"};

    Status Talk(ServerContext *context, const TalkRequest *request, TalkResponse *response) override {
        LOG(INFO) << "Talk received, data: " << request->data()<<", meta: " << request->meta();
        response->set_status(200);
        TalkResult* talkResult;
        talkResult=response->add_results();
        talkResult->set_id(1);
        talkResult->set_type(ResultType::OK);
        return Status::OK;
    }

    Status
    TalkOneAnswerMore(ServerContext *context, const TalkRequest *request, ServerWriter<TalkResponse> *writer) override {
        for (const string& hello : HELLO_LIST) {
            TalkResponse response;
            response.set_status(200);
            TalkResult *talkResult;
            talkResult=response.add_results();
            talkResult->set_id(1);
            talkResult->set_type(ResultType::OK);
            Map<string, string> *pMap = talkResult->mutable_kv();
            (*pMap)["data"]=hello;
            writer->Write(response);
        }
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
    LOG(INFO) << "Hello gRPC C++ Server is starting...";
    std::string server_address("0.0.0.0:9996");

    grpc::EnableDefaultHealthCheckService(true);
    grpc::reflection::InitProtoReflectionServerBuilderPlugin();
    ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());

    LandingServiceImpl landingService;
    builder.RegisterService(&landingService);

    std::unique_ptr<Server> server(builder.BuildAndStart());
    LOG(INFO) << "Server listening on " << server_address;
    server->Wait();
}

int main(int argc, char **argv) {
    google::InitGoogleLogging(argv[0]);
    google::SetLogDestination(google::INFO, "/Users/han/hello_grpc/");
    FLAGS_colorlogtostderr=true;
    FLAGS_alsologtostderr = 1;
    RunServer();
    LOG(WARNING) << "Hello gRPC C++ Server is stopping";
    google::ShutdownGoogleLogging();
    return 0;
}
