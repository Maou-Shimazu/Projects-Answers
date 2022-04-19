using System;

namespace rotCipherCs
{
    class Rot
    {
        public static char[] letters = { 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z' };
        public static string shifted;
        public static string rotate(string plain, int shift)
        {
            shifted = "";
            foreach (char c in plain.ToCharArray())
            {
                if (c != ' ') // cant shift a space!!
                {
                    if (Array.IndexOf(letters, c) + shift < letters.Length)
                    {
                        shifted += letters[Array.IndexOf(letters, c) + shift];
                    }
                    else if (Array.IndexOf(letters, c) + shift > letters.Length)
                    {
                        shift -= letters.Length;
                        shifted += letters[Array.IndexOf(letters, c) + shift];
                    }
                }
                else
                {
                    shifted += c;
                }
            }

            return shifted;

        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("Text to shift: ");
                string text = Console.ReadLine();
                Console.Write("Shift (make negative if decoding): ");
                int shift = (int)Convert.ToInt64(Console.ReadLine());
                Console.WriteLine($"Output: {Rot.rotate(text, shift)}");
            }
        }
    }
}
