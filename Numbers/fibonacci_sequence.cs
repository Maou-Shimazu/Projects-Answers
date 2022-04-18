using System;
using System.Linq;

namespace fibonacci_seq
{
    class Sequencer
    {
        public static int seqSum;
        public static int[] calculateSequence(int max)
        {
            int[] sequence = { 0, 1};
            for (int i = 1; i < max+1; i++)
            {
                seqSum = sequence[i] + sequence[i - 1];
                sequence = sequence.Append(seqSum).ToArray();
            }

            return sequence;
        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {

                Console.Write("Number to calculate to: ");
                string numberStr = Console.ReadLine();
                if (numberStr.Any(ch => !char.IsLetter(ch)) == true)
                {
                    int[] ans = Sequencer.calculateSequence((int)long.Parse(numberStr));
                    foreach (int i in ans)
                    {
                        Console.Write("{0}, ", i);
                    }
                    Console.WriteLine();
                }
                else
                {
                    Console.WriteLine("Input was not a valid number!");
                }
            }
        }
    }
}
