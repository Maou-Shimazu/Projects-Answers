using System;
using System.Net;
using System.Threading;

namespace siteStatusChecker
{
    class Checker
    {
        public static void checkServer(string url, int intervalMins)
        {
            while (true)
            {
                string Message = string.Empty;
                HttpWebRequest request = (HttpWebRequest)HttpWebRequest.Create(url);

                // Set the credentials to the current user account
                request.Credentials = System.Net.CredentialCache.DefaultCredentials;
                request.Method = "GET";

                try
                {
                    using (HttpWebResponse response = (HttpWebResponse)request.GetResponse())
                    {
                        // Do nothing; we're only testing to see if we can get the response
                    }
                }
                catch (WebException ex)
                {
                    Message += ((Message.Length > 0) ? "\n" : "") + ex.Message;
                }

                switch(Message.Length == 0)
                {
                    case true:
                        Console.WriteLine("server is up!");
                        break;
                    case false:
                        Console.WriteLine("server is down!");
                        break;
                    default:
                        Console.WriteLine("unknown server status!");
                        break;
                }

                Thread.Sleep(intervalMins * 60000);

            }
        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            Console.Write("site to check: ");
            string url = Console.ReadLine();
            Console.Write("interval to check the site's status (whole number, in minutes): ");
            int intervalMins = (int)Convert.ToInt64(Console.ReadLine());
            Checker.checkServer(url, intervalMins);
        }
    }
}
