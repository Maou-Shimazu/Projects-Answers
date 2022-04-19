using System;
using System.Linq;

namespace countVowels
{
    class Vowels
    {
        public static char[] vowels = { 'a', 'e', 'i', 'o', 'u' };
        public static int[] count(string input)
        {
            int[] vowelCount = { 0, 0, 0, 0, 0 };

            for (int i = 0; i < vowels.Length; i++)
            {
                char currentVowel = vowels[i];
                vowelCount[i] = input.Count(v => v == currentVowel);

            }
            return vowelCount;
        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("Input a sentence: ");
                int[] vowelCount = Vowels.count(Console.ReadLine());
                for (var i = 0; i < vowelCount.Length; i++)
                {
                    Console.WriteLine($"{Vowels.vowels[i]}: {vowelCount[i]}");
                }
            }
        }
    }
}
