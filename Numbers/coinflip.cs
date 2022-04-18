using System;

namespace cs_coinflip
{
    class Looper
    {
        public static void loop(UInt64 limit, Action callback)
        {
            for (UInt64 i = 0; i < limit; i++)
            {
                callback();
            }
        }
    }

    class Flipper
    {
        public static Random choiceMaker = new Random(DateTime.Now.Millisecond);
        public static void flip()
        {
            int choice = 1 + choiceMaker.Next(2);
            switch (choice)
            {
                case 1:
                    Console.WriteLine("Heads!");
                    break;
                case 2:
                    Console.WriteLine("Tails!");
                    break;
            }
        }
    }

    class Program
    {
        public static string numStr;
        public static UInt64 num;
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("How many times would you like to flip the coin?: ");
                numStr = Console.ReadLine();
                try
                {
                    num = UInt64.Parse(numStr);
                }
                catch
                {
                    Console.WriteLine(numStr + " is not a valid number!");
                    System.Environment.Exit(1);
                }

                Looper.loop(num, Flipper.flip);

            }
        }
    }
}
