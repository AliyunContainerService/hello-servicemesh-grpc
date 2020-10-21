package org.feuyeux.grpc;

import java.util.List;
import java.util.Random;
import java.util.stream.IntStream;

import static java.util.stream.Collectors.toList;

public class HelloUtils {
    private static final Random random = new Random();

    public static List<String> getRandomIds(int max) {
        return IntStream.range(0, max)
                .mapToObj(i -> getRandomId())
                .collect(toList());
    }

    public static String getRandomId() {
        return String.valueOf(random.nextInt(5));
    }
}
