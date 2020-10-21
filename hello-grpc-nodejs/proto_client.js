let grpc = require('grpc')
let sleep = require('sleep')
let messages = require('./common/landing_pb')
let services = require('./common/landing_grpc_pb')

const {createLogger, format, transports} = require('winston')
const {combine, timestamp, printf} = format
const formatter = printf(({level, message, timestamp}) => {
    return `${timestamp} [${level}] ${message}`
})

const logger = createLogger({
    level: 'info',
    format: combine(
        timestamp(),
        formatter
    ),
    transports: [new transports.Console()],
})

function grpcServer() {
    let server = process.env.GRPC_SERVER
    if (typeof server !== 'undefined' && server !== null) {
        return server
    } else {
        return "localhost"
    }
}

function talk(client, request) {
    logger.info("Talk:" + request)
    const metadata = new grpc.Metadata()
    metadata.add("k1", "v1")
    metadata.add("k2", "v2")
    client.talk(request, metadata, function (err, response) {
        printResponse("Talk:", response)
    })
}

function talkMoreAnswerOne(client, requests) {
    const metadata = new grpc.Metadata()
    metadata.add("k1", "v1")
    metadata.add("k2", "v2")
    let call = client.talkMoreAnswerOne(function (err, response) {
        printResponse("TalkMoreAnswerOne:", response)
    })
    requests.forEach(request => {
        logger.info("TalkMoreAnswerOne:" + request)
        call.write(request, metadata)
    })
    call.end()
}

function talkOneAnswerMore(client, request) {
    logger.info("TalkOneAnswerMore:" + request)
    const metadata = new grpc.Metadata()
    metadata.add("k1", "v1")
    metadata.add("k2", "v2")
    let call = client.talkOneAnswerMore(request, metadata)

    call.on('data', function (response) {
        printResponse("TalkOneAnswerMore:", response)
    })
    call.on('end', function () {
        logger.debug("DONE")
    })
}
function talkBidirectional(client, requests) {
    const metadata = new grpc.Metadata()
    metadata.add("k1", "v1")
    metadata.add("k2", "v2")
    let call = client.talkBidirectional()
    call.on('data', function (response) {
        printResponse("TalkBidirectional:", response)
    })
    requests.forEach(request => {
        logger.info("TalkBidirectional:" + request)
        sleep.msleep(5)
        call.write(request, metadata)
    })
    call.end()
}

function randomId(max) {
    return Math.floor(Math.random() * Math.floor(max)).toString()
}

function printResponse(methodName, response) {
    response.getResultsList().forEach(result => {
        let kv = result.getKvMap()
        logger.info(methodName + " " + response.getStatus() + " " + result.getId() +
            " [" + kv.get("meta") + " " + result.getType() + " " + kv.get("id") + "," +
            kv.get("idx") + ":" + kv.get("data") + "]")
    })
}

function main() {
    let address = grpcServer() + ":9996"
    let c = new services.LandingServiceClient(address, grpc.credentials.createInsecure())
    //
    logger.info("Unary RPC")
    let request = new messages.TalkRequest()
    request.setData("0")
    request.setMeta("NODEJS")
    talk(c, request)

    sleep.msleep(50)
    //
    logger.info("Server streaming RPC")
    request = new messages.TalkRequest()
    request.setData("0,1,2")
    request.setMeta("NODEJS")
    talkOneAnswerMore(c, request)

    sleep.msleep(50)
    //
    logger.info("Client streaming RPC")
    let r1 = new messages.TalkRequest()
    r1.setData(randomId(5))
    r1.setMeta("NODEJS")
    let r2 = new messages.TalkRequest()
    r2.setData(randomId(5))
    r2.setMeta("NODEJS")
    let r3 = new messages.TalkRequest()
    r3.setData(randomId(5))
    r3.setMeta("NODEJS")
    let rs = [r1, r2, r3]
    talkMoreAnswerOne(c, rs)

    sleep.msleep(50)
    //
    logger.info("Bidirectional streaming RPC")
    talkBidirectional(c, rs)
}

main()