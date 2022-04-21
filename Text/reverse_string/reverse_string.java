import java.util.Scanner;

public class reverse_string {
    public static void main(String[] args) {
        System.out.println("Enter the string you want reversed...");
        System.out.println(new StringBuilder(new Scanner(System.in).nextLine()).reverse());
    }
}
