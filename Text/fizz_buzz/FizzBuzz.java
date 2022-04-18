class FizzBuzz {
    public static void main(String[] args) {
        for (int i = 0; i < 101; i++)
            System.out.println(i % 15 == 0? "fizz buzz" : i % 5 == 0? "fizz" : i % 3 == 0? "buzz" : String.valueOf(i));
    }
}
