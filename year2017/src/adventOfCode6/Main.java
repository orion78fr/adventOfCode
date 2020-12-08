package adventOfCode6;

import java.util.Arrays;
import java.util.Set;
import java.util.TreeSet;

public class Main {
    static int[] memBanks = new int[]{10, 3, 15, 10, 5, 15, 5, 15, 9, 2, 5, 8, 5, 2, 3, 6};

    public static void main(String[] args) {
        Set<int[]> encounteredStates = new TreeSet<>((u, v) -> {
            for (int i = 0; i < u.length; i++) {
                if (u[i] < v[i]) {
                    return -1;
                } else if (u[i] > v[i]) {
                    return 1;
                }
            }
            return 0;
        });

        while (!encounteredStates.contains(memBanks)) {
            encounteredStates.add(Arrays.copyOf(memBanks, memBanks.length));

            redistribute();
        }
        System.out.println("" + encounteredStates.size());

        int[] toEncounterAgain = Arrays.copyOf(memBanks, memBanks.length);
        redistribute();
        int iter = 1;
        while (!Arrays.equals(toEncounterAgain, memBanks)) {
            redistribute();
            iter++;
        }

        System.out.println(iter);
    }

    private static void redistribute() {
        // Find biggest entry (when tied, take the first)

        int biggest = Integer.MIN_VALUE;
        int biggestIndex = -1;
        for (int i = 0; i < memBanks.length; i++) {
            if (memBanks[i] > biggest) {
                biggestIndex = i;
                biggest = memBanks[i];
            }
        }

        // Redistribute its entry
        int valueToRedistribute = biggest;
        memBanks[biggestIndex] = 0;
        biggestIndex++;

        while (valueToRedistribute != 0) {
            memBanks[biggestIndex % memBanks.length]++;
            biggestIndex++;
            valueToRedistribute--;
        }
    }
}
