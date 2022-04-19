using System;

namespace pigLatin
{
    class pigLatin
    {
        public static string pigLatinIfied;
        public static string endStr;
        public static int inSection;
        public static string makePigLatin(string toMakePigLatin)
        {
            inSection = 1;
            pigLatinIfied = "";

            foreach (char l in toMakePigLatin.ToCharArray())
            {
                if (inSection == 1)
                {
                    if (isVowel(l) == true)
                    {
                        pigLatinIfied += l;
                    }
                    else if (isVowel(l) == false)
                    {
                        endStr = $"-{l}ay";
                        inSection = 2;
                    }
                }
                else if (inSection == 2)
                {
                    pigLatinIfied += l;
                }
            }

            return pigLatinIfied + endStr;

        }

        public static bool isVowel(char l)
        {
            char[] vowels = { 'a', 'e', 'i', 'o', 'u' };
            for (int i = 0; i < vowels.Length; i++)
            {
                if (l == vowels[i])
                {
                    return true;
                }
            }
            return false;
        }

    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("Word to pig-latin-ify: ");
                Console.WriteLine($"Pig-latin-ified: {pigLatin.makePigLatin(Console.ReadLine())}");
            }
        }
    }
}
