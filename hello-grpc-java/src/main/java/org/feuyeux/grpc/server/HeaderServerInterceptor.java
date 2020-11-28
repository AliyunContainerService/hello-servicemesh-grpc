package org.feuyeux.grpc.server;

import io.grpc.*;
import io.grpc.ForwardingServerCall.SimpleForwardingServerCall;
import lombok.extern.slf4j.Slf4j;

import static org.feuyeux.grpc.Constants.*;

@Slf4j
public class HeaderServerInterceptor implements ServerInterceptor {
    @Override
    public <ReqT, RespT> ServerCall.Listener<ReqT> interceptCall(
            ServerCall<ReqT, RespT> call,
            final Metadata requestHeaders,
            ServerCallHandler<ReqT, RespT> serverCallHandler) {
        Context current = Context.current();
        log.info("Context:{}", current.toString());
        for (int i = 0; i < tracingKeys.size(); i++) {
            Metadata.Key<String> tracingKey = tracingKeys.get(i);
            String metadata = requestHeaders.get(tracingKey);
            if (metadata != null) {
                Context.Key<String> key = contextKeys.get(i);
                log.info("Metadata to context {}:{}", key, metadata);
                current = current.withValue(key, metadata);
            }
        }
        return Contexts.interceptCall(current, call, requestHeaders, serverCallHandler);
    }
}