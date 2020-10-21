package org.feuyeux.grpc.client;

import io.grpc.*;
import io.grpc.stub.StreamObserver;
import lombok.extern.slf4j.Slf4j;
import org.feuyeux.grpc.HelloUtils;
import org.feuyeux.grpc.proto.LandingServiceGrpc;
import org.feuyeux.grpc.proto.TalkRequest;
import org.feuyeux.grpc.proto.TalkResponse;

import java.util.*;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

@Slf4j
public class ProtoClient {
    private final ManagedChannel channel;
    private final LandingServiceGrpc.LandingServiceBlockingStub blockingStub;
    private final LandingServiceGrpc.LandingServiceStub asyncStub;

    public ProtoClient(String host, int port) {
        channel = ManagedChannelBuilder.forAddress(host, port).usePlaintext().build();
        //channel = ManagedChannelBuilder.forAddress("39.97.228.93", 9999).usePlaintext().build();
        ClientInterceptor interceptor = new HeaderClientInterceptor();
        Channel interceptChannel = ClientInterceptors.intercept(channel, interceptor);
        blockingStub = LandingServiceGrpc.newBlockingStub(interceptChannel);
        asyncStub = LandingServiceGrpc.newStub(interceptChannel);
    }

    public static void main(String[] args) throws InterruptedException {
        ProtoClient protoClient = new ProtoClient(getGrcServer(), 9996);
        try {
            log.info("Unary RPC");
            TalkRequest talkRequest = TalkRequest.newBuilder()
                    .setMeta("JAVA")
                    .setData("0")
                    .build();
            log.info("Request data:{},meta:{}", talkRequest.getData(), talkRequest.getMeta());
            TalkResponse response = protoClient.talk(talkRequest);
            printResponse(response);
            log.info("Server streaming RPC");
            talkRequest = TalkRequest.newBuilder()
                    .setMeta("JAVA")
                    .setData("0,1,2")
                    .build();
            log.info("Request data:{},meta:{}", talkRequest.getData(), talkRequest.getMeta());
            List<TalkResponse> talkResponses = protoClient.talkOneAnswerMore(talkRequest);
            talkResponses.forEach(r -> printResponse(r));

            log.info("Client streaming RPC");
            List<TalkRequest> requests = Arrays.asList(TalkRequest.newBuilder()
                            .setMeta("JAVA")
                            .setData(HelloUtils.getRandomId())
                            .build(),
                    TalkRequest.newBuilder()
                            .setMeta("JAVA")
                            .setData(HelloUtils.getRandomId())
                            .build(),
                    TalkRequest.newBuilder()
                            .setMeta("JAVA")
                            .setData(HelloUtils.getRandomId())
                            .build());
            protoClient.talkMoreAnswerOne(requests);

            log.info("Bidirectional streaming RPC");
            protoClient.talkBidirectional(requests);
        } finally {
            protoClient.shutdown();
        }
    }

    private static void printResponse(TalkResponse response) {
        response.getResultsList().forEach(result -> {
                    Map<String, String> kv = result.getKvMap();
                    log.info("{} {} [{} {} {},{}:{}]", response.getStatus(), result.getId(),
                            kv.get("meta"), result.getType(), kv.get("id"), kv.get("idx"), kv.get("data"));
                }

        );
    }

    private static String getGrcServer() {
        String server = System.getenv("GRPC_SERVER");
        if (server == null) {
            return "localhost";
        }
        return server;
    }

    public void shutdown() throws InterruptedException {
        channel.shutdown().awaitTermination(5, TimeUnit.SECONDS);
    }

    public TalkResponse talk(TalkRequest talkRequest) {
        return blockingStub.talk(talkRequest);
    }

    public List<TalkResponse> talkOneAnswerMore(TalkRequest request) {
        List<TalkResponse> talkResponseList = new ArrayList<>();
        Iterator<TalkResponse> talkResponses = blockingStub.talkOneAnswerMore(request);
        talkResponses.forEachRemaining(talkResponseList::add);
        return talkResponseList;
    }

    public void talkMoreAnswerOne(List<TalkRequest> requests) throws InterruptedException {
        final CountDownLatch finishLatch = new CountDownLatch(1);
        StreamObserver<TalkResponse> responseObserver = new StreamObserver<TalkResponse>() {
            @Override
            public void onNext(TalkResponse talkResponse) {
                printResponse(talkResponse);
            }

            @Override
            public void onError(Throwable t) {
                log.error("", t);
                finishLatch.countDown();
            }

            @Override
            public void onCompleted() {
                finishLatch.countDown();
            }
        };
        final StreamObserver<TalkRequest> requestObserver = asyncStub.talkMoreAnswerOne(responseObserver);
        try {
            requests.forEach(request -> {
                if (finishLatch.getCount() > 0) {
                    log.info("Request data:{},meta:{}", request.getData(), request.getMeta());
                    requestObserver.onNext(request);
                    try {
                        TimeUnit.MICROSECONDS.sleep(5);
                    } catch (InterruptedException ignored) {
                    }
                }
            });
        } catch (Exception e) {
            requestObserver.onError(e);
            throw e;
        }
        // Mark the end of requests
        requestObserver.onCompleted();

        // Receiving happens asynchronously
        if (!finishLatch.await(1, TimeUnit.MINUTES)) {
            log.warn("can not finish within 1 minutes");
        }
    }

    public void talkBidirectional(List<TalkRequest> requests) throws InterruptedException {
        final CountDownLatch finishLatch = new CountDownLatch(1);
        StreamObserver<TalkResponse> responseObserver = new StreamObserver<TalkResponse>() {
            @Override
            public void onNext(TalkResponse talkResponse) {
                printResponse(talkResponse);
            }

            @Override
            public void onError(Throwable t) {
                log.error("", t);
                finishLatch.countDown();
            }

            @Override
            public void onCompleted() {
                finishLatch.countDown();
            }
        };
        final StreamObserver<TalkRequest> requestObserver = asyncStub.talkBidirectional(responseObserver);
        try {
            requests.forEach(request -> {
                if (finishLatch.getCount() > 0) {
                    log.info("Request data:{},meta:{}", request.getData(), request.getMeta());
                    requestObserver.onNext(request);
                    try {
                        TimeUnit.SECONDS.sleep(1);
                    } catch (InterruptedException ignored) {
                    }
                }
            });
        } catch (Exception e) {
            requestObserver.onError(e);
            throw e;
        }
        // Mark the end of requests
        requestObserver.onCompleted();

        // Receiving happens asynchronously
        if (!finishLatch.await(1, TimeUnit.MINUTES)) {
            log.warn("can not finish within 1 minutes");
        }
    }
}
