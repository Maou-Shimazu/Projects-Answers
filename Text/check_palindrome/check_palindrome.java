import java.util.Scanner;

class check_palindrome {
    public static void main(String[] args) {
        System.out.println("Enter a word: ");
        final String s = new Scanner(System.in).nextLine();
        if (s.equals(new StringBuilder(s).reverse().toString()))
            System.out.println("The word is a palindrome.");
        else
            System.out.println("The word is not a palindrome.");
    }
}
