using System;
using System.Linq;

namespace cs_calc
{
    class Evaluator
    {
        public static string eval(String expression)
        {
            try
            {
                System.Data.DataTable table = new System.Data.DataTable();
                return Convert.ToDouble(table.Compute(expression, String.Empty)).ToString();
            }
            catch
            {
                return new string("Invalid equation!");
            }
        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.Write("calcRepl> ");
                string equation = Console.ReadLine();

                if (equation.Any(ch => !char.IsLetter(ch)) == true) {
                    Console.WriteLine(Evaluator.eval(equation));
                }
                else
                {
                    Console.WriteLine("Input was not a valid equation! Try removing letters, as those aren't valid in this calculator.");
                }
            }
        }
    }
}
