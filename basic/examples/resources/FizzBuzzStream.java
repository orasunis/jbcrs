import java.util.stream.IntStream;

/**
 * FizzBuzz using java streams
 * (I didn't have a better idea using BootstrapMethods & InvokeDynamics)
 */
public class FizzBuzzStream {
    public static void main(String[] args) {
        IntStream.rangeClosed(1, 100)
            .mapToObj(i -> {
                if (i % 15 == 0) {
                    return "FizzBuzz";
                } else if (i % 3 == 0) {
                    return "Fizz";
                } else if (i % 5 == 0) {
                    return "Buzz";
                } else {
                    return String.valueOf(i);
                }
            })
            .forEach(System.out::println);
    }
}
