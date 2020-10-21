package org.feuyeux.grpc.client;

import io.grpc.*;
import lombok.extern.slf4j.Slf4j;

import static org.feuyeux.grpc.Constants.*;

@Slf4j
public class HeaderClientInterceptor implements ClientInterceptor {
    @Override
    public <ReqT, RespT> ClientCall<ReqT, RespT> interceptCall(MethodDescriptor<ReqT, RespT> method, CallOptions callOptions, Channel next) {
        return new ForwardingClientCall
                .SimpleForwardingClientCall<ReqT, RespT>(next.newCall(method, callOptions)) {
            @Override
            public void start(Listener<RespT> responseListener, Metadata headers) {
                for (int i = 0; i < tracingKeys.size(); i++) {
                    String metadata = contextKeys.get(i).get();
                    Metadata.Key<String> key = tracingKeys.get(i);
                    log.info("Context to metadata {}:{}", key, metadata);
                    if (metadata != null) {
                        headers.put(key, metadata);
                    }
                }

                super.start(new ForwardingClientCallListener.SimpleForwardingClientCallListener<RespT>(responseListener) {
                    @Override
                    public void onHeaders(Metadata headers) {
                        log.info("header received from server:" + headers);
                        super.onHeaders(headers);
                    }
                }, headers);
            }
        };
    }
}