using System;

namespace actual_str_reverse
{
    class Reverser
    {
        public static string reverse(string toReverse)
        {
            string reversed = "";
            int originalLen = toReverse.Length -1;
            while (originalLen >= 0)
            {
                reversed = reversed + toReverse[originalLen];
                originalLen--;
            }
            return reversed;
        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("To reverse: ");
                string reverse = Console.ReadLine();
                Console.WriteLine(Reverser.reverse(reverse));
            }
        }
    }
}
