package adventOfCode3;

import java.util.HashMap;
import java.util.Map;

public class Part2 {
    public enum Direction {
        RIGHT(1, 0),
        UP(0, -1),
        LEFT(-1, 0),
        DOWN(0, 1);

        private Direction turn;
        private final Point point;

        static {
            RIGHT.turn = UP;
            UP.turn = LEFT;
            LEFT.turn = DOWN;
            DOWN.turn = RIGHT;
        }

        Direction(int x, int y) {
            point = new Point(x, y);
        }

        public Direction getTurn() {
            return turn;
        }

        public Point getPoint() {
            return point;
        }
    }

    public record Point(int x, int y) {
        public Point translate(Point p) {
            return new Point(x + p.x, y + p.y);
        }
    }

    public static void main(String[] args) {
        Point[] neighbours = new Point[]{
                new Point(0, 1),
                new Point(0, -1),
                new Point(1, 0),
                new Point(-1, 0),
                new Point(1, 1),
                new Point(-1, 1),
                new Point(1, -1),
                new Point(-1, -1),
        };
        int input = 312051;

        Map<Point, Integer> m = new HashMap<>();

        m.put(new Point(0,0), 1);

        Point curPoint = new Point(1, 0);
        int s = 0;
        Direction curDir = Direction.RIGHT;
        while (s < input) {
            s = 0;

            for (Point neighbour : neighbours) {
                s += m.getOrDefault(curPoint.translate(neighbour), 0);
            }

            m.put(curPoint, s);

            Point turnPoint = curPoint.translate(curDir.getTurn().getPoint());
            if (!m.containsKey(turnPoint)) {
                curPoint = turnPoint;
                curDir = curDir.getTurn();
            } else {
                curPoint = curPoint.translate(curDir.getPoint());
            }
        }

        System.out.println(s);
    }
}
