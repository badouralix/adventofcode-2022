using System;
using System.Collections.Generic;


namespace Aoc
{
    class Solution
    {
        private static int Solve(string input) {
            // Your code goes here
            return 0;
        }

        public static void Main(string[] args) {
            string input = args[0];
            var watch = new System.Diagnostics.Stopwatch();
            watch.Start();
            int result = Solve(input);
            watch.Stop();
            Console.WriteLine("_duration: " + watch.Elapsed.TotalMilliseconds + "\n" + result);
        }
        
    }
}
