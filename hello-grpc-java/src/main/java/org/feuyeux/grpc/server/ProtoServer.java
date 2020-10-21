package org.feuyeux.grpc.server;

import io.grpc.*;
import lombok.extern.slf4j.Slf4j;
import org.feuyeux.grpc.client.HeaderClientInterceptor;
import org.feuyeux.grpc.proto.LandingServiceGrpc;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

@Slf4j
public class ProtoServer {
    private final Server server;
    private static ManagedChannel channel;


    public ProtoServer(final int port) throws IOException {
        this(port, new LandingServiceImpl());
    }

    public ProtoServer(final int port, LandingServiceImpl landingService) throws IOException {
        ServerServiceDefinition intercept = ServerInterceptors.intercept(landingService, new HeaderServerInterceptor());
        this.server = ServerBuilder.forPort(port)
                .addService(intercept)
                .build();
        start();
    }

    public static void main(String[] args) throws InterruptedException, IOException {
        String backend = System.getenv("GRPC_HELLO_BACKEND");
        String backPort = System.getenv("GRPC_HELLO_BACKEND_PORT");
        String currentPort = System.getenv("GRPC_HELLO_PORT");
        ProtoServer server;
        LandingServiceImpl landingService = new LandingServiceImpl();
        if (backend != null) {
            int port;
            if (backPort != null) {
                port = Integer.parseInt(backPort);
            } else {
                port = 9996;
            }
            channel = ManagedChannelBuilder.forAddress(backend, port).usePlaintext().build();
            Channel interceptChannel = ClientInterceptors.intercept(channel, new HeaderClientInterceptor());
            LandingServiceGrpc.LandingServiceBlockingStub blockingStub = LandingServiceGrpc.newBlockingStub(interceptChannel);
            LandingServiceGrpc.LandingServiceStub asyncStub = LandingServiceGrpc.newStub(interceptChannel);
            landingService.setAsyncStub(asyncStub);
            landingService.setBlockingStub(blockingStub);
        }
        if (currentPort == null) {
            server = new ProtoServer(9996, landingService);
        } else {
            server = new ProtoServer(Integer.parseInt(currentPort), landingService);
        }
        server.blockUntilShutdown();
    }

    private void start() throws IOException {
        server.start();
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            log.warn("shutting down Google RPC Server since JVM is shutting down");
            ProtoServer.this.stop();
            log.warn("Google RPC Server shut down");
        }));
        log.info("Google RPC Server is launched.");
    }

    public void blockUntilShutdown() throws InterruptedException {
        if (server != null) {
            server.awaitTermination();
        }
        if (channel != null) {
            channel.shutdown().awaitTermination(5, TimeUnit.SECONDS);
        }
    }

    public void stop() {
        server.shutdown();
    }
}
