import java.util.Scanner;

class PigLatin {
    public static void main(String[] args) {
        System.out.print("Enter a word: ");
        System.out.println(pigLatin(new Scanner(System.in).nextLine()));
    }

    private static String pigLatin(final String s) {
        if ("aouei".indexOf(s.charAt(0)) == -1) {
            final StringBuilder ret = new StringBuilder();
            for (final char c: s.toCharArray()) {
                if ("aoeui".indexOf(c) != -1)
                    break;
                ret.append(c);
            }

            return s.substring(ret.length()) + ret + "ay";
        }

        return s + "yay";
    }
}
