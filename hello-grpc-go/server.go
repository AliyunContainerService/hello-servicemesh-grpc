/*
 * grpc demo
 */

package main

import (
	"github.com/feuyeux/hello-grpc-go/common/pb"
	"github.com/feuyeux/hello-grpc-go/server"
	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
	"net"
	"os"
)

const (
	port = ":9996"
)

func main() {
	backend := os.Getenv("GRPC_HELLO_BACKEND")
	backPort := os.Getenv("GRPC_HELLO_BACKEND_PORT")
	currentPort := os.Getenv("GRPC_HELLO_PORT")
	var lis net.Listener
	var err error

	if len(currentPort) > 0 {
		lis, err = net.Listen("tcp", ":"+currentPort)
		if err != nil {
			log.Fatalf("Failed to listen: %v", err)
			return
		}
	} else {
		lis, err = net.Listen("tcp", port)
		if err != nil {
			log.Fatalf("Failed to listen: %v", err)
			return
		}
	}
	s := grpc.NewServer()
	var srv *server.ProtoServer
	if len(backend) > 0 {
		log.Infof("Start GRPC Server backend:%v", backend)
		var address string
		if len(backPort) > 0 {
			address = backend + ":" + backPort
		} else {
			address = backend + port
		}
		conn, err := grpc.Dial(address, grpc.WithInsecure())
		if err != nil {
			log.Fatalf("Did not connect: %v", err)
		}
		defer conn.Close()
		c := pb.NewLandingServiceClient(conn)
		srv = &server.ProtoServer{BackendClient: c}
	} else {
		log.Info("Start GRPC Server")
		srv = &server.ProtoServer{}
	}
	pb.RegisterLandingServiceServer(s, srv)
	// Register reflection service on gRPC server.
	reflection.Register(s)
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
