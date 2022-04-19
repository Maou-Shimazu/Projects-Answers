using System;

namespace fizz_buzz
{
    class Multiples
    {
        public static bool check(int first, int second)
        {
            return first % second == 0;
        }
        public static void multiples(int num)
        {
            if (check(num, 3) == true && check(num, 5) == true)
            {
                Console.WriteLine("FizzBuzz");
            }
            else if (check(num, 3) == true)
            {
                Console.WriteLine("Fizz");
            }
            else if (check(num, 5) == true)
            {
                Console.WriteLine("Buzz");
            }
            else
            {
                Console.WriteLine(num);
            }
        }
    }
    class Program
    {
        public static void loop(int amountOfInstances, Action<int> callback)
        {
            for (int i = 0; i < amountOfInstances; i++)
            {
                callback(i);
            }
        }

        static void Main(string[] args)
        {
            loop(100, Multiples.multiples);
        }
    }
}
