using System;

namespace bin_to_dec
{
    class BinaryConversions
    {
        public static double dec = 0;
        public static double binToDec(string bin)
        {
            for (var i = 0; i < bin.Length; i++)
            {
                dec += bin[i] * Math.Pow(2, bin.Length - i - 1);
            }

            return dec;

        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("Binary number: ");
                string binStr = Console.ReadLine();
                Console.WriteLine("{0}", BinaryConversions.binToDec(binStr));
            }
        }
    }
}
