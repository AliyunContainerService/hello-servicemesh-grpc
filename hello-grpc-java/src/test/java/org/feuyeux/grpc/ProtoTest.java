package org.feuyeux.grpc;

import lombok.extern.slf4j.Slf4j;
import org.feuyeux.grpc.client.ProtoClient;
import org.feuyeux.grpc.proto.TalkRequest;
import org.feuyeux.grpc.proto.TalkResponse;
import org.feuyeux.grpc.server.ProtoServer;
import org.junit.Assert;
import org.junit.Test;

import java.io.IOException;
import java.net.Inet4Address;
import java.net.InetAddress;
import java.net.NetworkInterface;
import java.net.SocketException;
import java.util.Enumeration;

/**
 * Created by erichan feuyeux
 * on 16/8/22
 */
@Slf4j
public class ProtoTest {
    public static String getLocalIp() {
        try {
            Enumeration<NetworkInterface> allNetInterfaces = NetworkInterface.getNetworkInterfaces();
            InetAddress ip;
            while (allNetInterfaces.hasMoreElements()) {
                NetworkInterface netInterface = allNetInterfaces.nextElement();
                if ("lo".equals(netInterface.getName())) {
                    // 如果是回环网卡跳过
                    continue;
                }
                Enumeration<InetAddress> addresses = netInterface.getInetAddresses();
                while (addresses.hasMoreElements()) {
                    ip = addresses.nextElement();
                    if (ip instanceof Inet4Address) {
                        String t = ip.getHostAddress();
                        if (!"127.0.0.1".equals(t)) {
                            // 只返回不是本地的IP
                            return t;
                        }
                    }
                }
            }
            return null;
        } catch (SocketException e) {
            log.error("", e);
            return null;
        }
    }

    @Test
    public void testProto() throws InterruptedException, IOException {
        ProtoServer protoServer = new ProtoServer(17002);
        ProtoClient protoClient = new ProtoClient(getLocalIp(), 17002);
        TalkRequest talkRequest = TalkRequest.newBuilder()
                .setMeta("id=" + System.nanoTime())
                .setData("eric")
                .build();
        log.info("REQUEST:{}", talkRequest);
        TalkResponse talkResponse = protoClient.talk(talkRequest);

        Assert.assertTrue(talkResponse.getStatus() == 200);
        log.info("RESPONSE:{}", talkResponse);

        protoClient.shutdown();
        protoServer.stop();
    }
}