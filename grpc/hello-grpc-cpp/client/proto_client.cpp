#include <iostream>
#include <memory>
#include <string>

#include <grpcpp/grpcpp.h>
#include "helloworld.grpc.pb.h"
#include "landing.grpc.pb.h"
#include <glog/logging.h>

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;
using helloworld::Greeter;
using helloworld::HelloReply;
using helloworld::HelloRequest;
using org::feuyeux::grpc::TalkRequest;
using org::feuyeux::grpc::TalkResponse;
using org::feuyeux::grpc::LandingService;
using org::feuyeux::grpc::TalkResult;
using org::feuyeux::grpc::ResultType;
using grpc::ClientReader;
using google::protobuf::RepeatedPtrField;
using google::protobuf::Map;
using std::string;

class LandingClient {
public:
    LandingClient(std::shared_ptr<Channel> channel) : client(LandingService::NewStub(channel)) {}

    void Talk() {
        TalkRequest talkRequest;
        talkRequest.set_data("hello");
        talkRequest.set_meta("c++");

        TalkResponse talkResponse;
        ClientContext context;
        Status status = client->Talk(&context, talkRequest, &talkResponse);
        if (status.ok()) {
            std::cout << "Talk status:" << talkResponse.status() << std::endl;
            const TalkResult &talkResult = talkResponse.results(0);
            ResultType resultType = talkResult.type();
            std::cout << "Talk result:" << resultType << std::endl;
        } else {
            std::cout << "Talk error:" << status.error_code() << ": " << status.error_message()
                      << std::endl;
        }
    }

    void TalkOneAnswerMore() {
        TalkRequest talkRequest;
        talkRequest.set_data("1,2,3");
        talkRequest.set_meta("c++");
        ClientContext context;
        TalkResponse talkResponse;
        const std::unique_ptr<::grpc::ClientReader<TalkResponse>> &response(
                client->TalkOneAnswerMore(&context, talkRequest));
        while (response->Read(&talkResponse)) {
            const RepeatedPtrField<TalkResult> &talkResults = talkResponse.results();
            for (TalkResult talkResult: talkResults) {
                const Map<string, string> &kv = talkResult.kv();
                string data(kv.at("data"));
                LOG(INFO) << data;
            }
        }
    }
private:
    std::unique_ptr<LandingService::Stub> client;
};

int main(int argc, char **argv) {
    /*日志文件名 <program name>.<host name>.<user name>.log.<Severity level>.<date>-<time>.<pid> */
    google::InitGoogleLogging(argv[0]);
    google::SetLogDestination(google::INFO, "/Users/han/hello_grpc/");
    FLAGS_colorlogtostderr = true;
    FLAGS_alsologtostderr = 1;

    LOG(INFO) << "Hello gRPC C++ Client is starting...";

    // Instantiate the client. It requires a channel, out of which the actual RPCs
    // are created. This channel models a connection to an endpoint specified by
    // the argument "--target=" which is the only expected argument.
    // We indicate that the channel isn't authenticated (use of
    // InsecureChannelCredentials()).
    std::string target_str;
    std::string arg_str("--target");
    if (argc > 1) {
        std::string arg_val = argv[1];
        size_t start_pos = arg_val.find(arg_str);
        if (start_pos != std::string::npos) {
            start_pos += arg_str.size();
            if (arg_val[start_pos] == '=') {
                target_str = arg_val.substr(start_pos + 1);
            } else {
                std::cout << "The only correct argument syntax is --target="
                          << std::endl;
                return 0;
            }
        } else {
            std::cout << "The only acceptable argument is --target=" << std::endl;
            return 0;
        }
    } else {
        target_str = "localhost:9996";
    }
    const std::shared_ptr<Channel> &channel = grpc::CreateChannel(target_str, grpc::InsecureChannelCredentials());
    //
    LandingClient landingClient(channel);
    landingClient.Talk();
    landingClient.TalkOneAnswerMore();
    LOG(WARNING) << "Hello gRPC C++ Client is stopping";
    google::ShutdownGoogleLogging();
    return 0;
}
