import java.util.Scanner;

class Binary {
    public static void main(String[] args) {
        System.out.print("Enter a binary number: "); /* This is such a lazy solution. */
        System.out.println(Integer.parseInt(new Scanner(System.in).nextLine(), 2));
    }
}
