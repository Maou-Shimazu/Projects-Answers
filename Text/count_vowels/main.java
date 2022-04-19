import java.util.Scanner;

class Vowels {
    public static void main(String[] args) {
        System.out.print("Enter a word: ");
        System.out.println(new Scanner(System.in).nextLine().chars().mapToObj(i -> (char) i).filter(c -> "aoeui".indexOf(c) != -1).count());
    }
}
