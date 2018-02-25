public class Fibonacci {
    public static void main(String[] args) {
        if (args.length == 0) {
            System.out.println("Not enough arguments provided");
            return;
        }

        int f = Integer.parseInt(args[0]);
        System.out.println(fib(f));
    }

    /**
     * Computes the fibonacci sequence to f
     */
    private static int fib(int f) {
        int f1 = 1;
        int f2 = 1;

        for (int i = 3; i <= f; i++) {
            int sum = f1 + f2;
            f1 = f2;
            f2 = sum;
        }

        return f2;
    }
}
