FROM openjdk:8-jdk-alpine
COPY hello-grpc-java.jar grpc-server.jar
ENTRYPOINT ["java","-jar","/grpc-server.jar"]