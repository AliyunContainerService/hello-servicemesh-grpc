# encoding: utf-8
import logging
import os
import time
import uuid

import grpc
from concurrent import futures

from landing_pb2 import landing_pb2
from landing_pb2 import landing_pb2_grpc

logger = logging.getLogger('grpc-server')
logger.setLevel(logging.INFO)
console = logging.StreamHandler()
console.setLevel(logging.INFO)
formatter = logging.Formatter('%(asctime)s [%(levelname)s] - %(message)s')
console.setFormatter(formatter)
logger.addHandler(console)

hellos = ["Hello", "Bonjour", "Hola", "こんにちは", "Ciao", "안녕하세요"]
tracing_keys = [
    "x-request-id",
    "x-b3-traceid",
    "x-b3-spanid",
    "x-b3-parentspanid",
    "x-b3-sampled",
    "x-b3-flags",
    "x-ot-span-context"
]


def build_result(data):
    result = landing_pb2.TalkResult()
    result.id = int((time.time()))
    result.type = landing_pb2.OK
    result.kv["id"] = str(uuid.uuid1())
    result.kv["idx"] = data
    index = int(data)
    result.kv["data"] = hellos[index]
    result.kv["meta"] = "PYTHON"
    return result


def propaganda_headers(method_name, context):
    metadata = context.invocation_metadata()
    metadata_dict = {}
    for c in metadata:
        logger.debug("%s HEADER %s:%s", method_name, c.key, c.value)
        if c.key in tracing_keys:
            logger.debug("%s TRACING HEADER %s:%s", method_name, c.key, c.value)
            metadata_dict[c.key] = c.value
    # Converting dictionary into list of tuple
    return list(metadata_dict.items())


class LandingServiceServer(landing_pb2_grpc.LandingServiceServicer):
    def __init__(self, next_one):
        self.next_one = next_one

    def talk(self, request, context):
        logger.info("TALK REQUEST: data=%s,meta=%s", request.data, request.meta)
        if self.next_one:
            headers = propaganda_headers("TALK", context)
            try:
                return self.next_one.talk(request=request, metadata=headers)
            except Exception as e:
                logger.error("Unexpected Error: {}".format(e))
        else:
            response = landing_pb2.TalkResponse()
            response.status = 200
            result = build_result(request.data)
            response.results.append(result)
            return response

    def talkOneAnswerMore(self, request, context):
        logger.info("TalkOneAnswerMore REQUEST: data=%s,meta=%s", request.data, request.meta)
        if self.next_one:
            headers = propaganda_headers("TalkOneAnswerMore", context)
            responses = self.next_one.talkOneAnswerMore(request=request, metadata=headers)
            for response in responses:
                yield response
        else:
            datas = request.data.split(",")
            for data in datas:
                response = landing_pb2.TalkResponse()
                response.status = 200
                result = build_result(data)
                response.results.append(result)
                yield response

    def talkMoreAnswerOne(self, request_iterator, context):
        if self.next_one:
            headers = propaganda_headers("TalkMoreAnswerOne", context)
            return self.next_one.talkMoreAnswerOne(request_iterator=request_iterator, metadata=headers)
        else:
            response = landing_pb2.TalkResponse()
            response.status = 200
            for request in request_iterator:
                logger.info("TalkMoreAnswerOne REQUEST: data=%s,meta=%s", request.data, request.meta)
                response.results.append(build_result(request.data))
            return response

    def talkBidirectional(self, request_iterator, context):
        if self.next_one:
            headers = propaganda_headers("TalkBidirectional", context)
            responses = self.next_one.talkBidirectional(request_iterator=request_iterator, metadata=headers)
            for response in responses:
                yield response
        else:
            for request in request_iterator:
                logger.info("TalkBidirectional REQUEST: data=%s,meta=%s", request.data, request.meta)
                response = landing_pb2.TalkResponse()
                response.status = 200
                result = build_result(request.data)
                response.results.append(result)
                yield response


def serve():
    backend = os.getenv("GRPC_HELLO_BACKEND")
    back_port = os.getenv("GRPC_HELLO_BACKEND_PORT")
    current_port = os.getenv("GRPC_HELLO_PORT")
    if backend:
        if back_port:
            address = backend + ":" + back_port
        else:
            address = backend + ":9996"
        logger.info("BACKEND:" + address)
        channel = grpc.insecure_channel(address)
        stub = landing_pb2_grpc.LandingServiceStub(channel)
        service_server = LandingServiceServer(stub)
    else:
        service_server = LandingServiceServer(None)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    landing_pb2_grpc.add_LandingServiceServicer_to_server(service_server, server)
    if current_port:
        port = "[::]:" + current_port
    else:
        port = '[::]:9996'
    server.add_insecure_port(port)
    server.start()
    logger.info("Start GRPC Server:" + port)
    try:
        server.wait_for_termination()
    except KeyboardInterrupt:
        server.stop(0)
    channel.close()


if __name__ == '__main__':
    serve()
