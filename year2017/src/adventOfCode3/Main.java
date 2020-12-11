package adventOfCode3;

public class Main {
    static int x = 0;
    static int y = 0;
    static int currentVal = 1;
    static int iter = 0;

    public static void main(String[] args) {
        int in = 312051;

        while (currentVal != in) {
            int step = iter / 2 + 1;
            if (currentVal + step >= in) {
                increment(in - currentVal);
            } else {
                increment(step);
            }
        }

        System.out.println("" + (Math.abs(x) + Math.abs(y)));
    }

    private static void increment(int increment) {
        currentVal += increment;
        switch (iter % 4) {
            case 0: // RIGHT
                x += increment;
                break;
            case 1: // UP
                y -= increment;
                break;
            case 2: // LEFT
                x -= increment;
                break;
            case 3: // DOWN
                y += increment;
                break;
            default:
                throw new RuntimeException(" LOL ");
        }
        iter++;
    }
}
