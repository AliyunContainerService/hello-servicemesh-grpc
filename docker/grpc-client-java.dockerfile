FROM openjdk:8-jdk-alpine
COPY hello-grpc-java-client.jar grpc-client.jar
# ENTRYPOINT ["java","-jar","/grpc-client.jar"]