using Terminal.Gui;

namespace AdventOfCode
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Application.Init();
            Application.Run<TerminalWindow>();
            Application.Shutdown();
        }
    }
}