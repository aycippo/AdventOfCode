using Terminal.Gui;

namespace AdventOfCode
{
    internal class TerminalWindow : Window
    {
        public TerminalWindow()
        {
            Title = "Advent of Code Solutions (Ctrl+Q to quit)";

            // Create input components and labels
            var yearLabel = new Label()
            {
                Text = "Year:"
            };

            var _2015Button = new Button()
            {
                Text = "2015",
                // 1 + adds an indent of 1 character to all items to the right of the yearLabel
                X = 1 + Pos.Left(yearLabel),
                Y = Pos.Bottom(yearLabel),
            };

            // When year button is clicked display a message popup
            _2015Button.Clicked += () =>
            {
                MessageBox.Query("2015 Selected", "No Solutions Available", "Ok");
            };

            var _2016Button = new Button()
            {
                Text = "2016",
                X = Pos.Left(_2015Button),
                Y = Pos.Bottom(_2015Button),
            };

            // When year button is clicked display a message popup
            _2016Button.Clicked += () =>
            {
                MessageBox.Query("2016 Selected", "No Solutions Available", "Ok");
            };

            var _2017Button = new Button()
            {
                Text = "2017",
                X = Pos.Left(_2016Button),
                Y = Pos.Bottom(_2016Button),
            };

            // When year button is clicked display a message popup
            _2017Button.Clicked += () =>
            {
                MessageBox.Query("2017 Selected", "No Solutions Available", "Ok");
            };

            var _2018Button = new Button()
            {
                Text = "2018",
                X = Pos.Left(_2017Button),
                Y = Pos.Bottom(_2017Button),
            };

            // When year button is clicked display a message popup
            _2018Button.Clicked += () =>
            {
                MessageBox.Query("2018 Selected", "No Solutions Available", "Ok");
            };

            var _2019Button = new Button()
            {
                Text = "2019",
                X = Pos.Left(_2018Button),
                Y = Pos.Bottom(_2018Button),
            };

            // When year button is clicked display a message popup
            _2019Button.Clicked += () =>
            {
                MessageBox.Query("2019 Selected", "No Solutions Available", "Ok");
            };

            var _2020Button = new Button()
            {
                Text = "2020",
                X = Pos.Left(_2019Button),
                Y = Pos.Bottom(_2019Button),
            };

            // When year button is clicked display a message popup
            _2020Button.Clicked += () =>
            {
                MessageBox.Query("2020 Selected", "No Solutions Available", "Ok");
            };

            var _2021Button = new Button()
            {
                Text = "2021",
                X = Pos.Left(_2020Button),
                Y = Pos.Bottom(_2020Button),
            };

            // When year button is clicked display a message popup
            _2021Button.Clicked += () =>
            {
                MessageBox.Query("2021 Selected", "No Solutions Available", "Ok");
            };

            var _2022Button = new Button()
            {
                Text = "2022",
                X = Pos.Left(_2021Button),
                Y = Pos.Bottom(_2021Button),
            };

            // When year button is clicked display a message popup
            _2022Button.Clicked += () =>
            {
                MessageBox.Query("2022 Selected", "No Solutions Available", "Ok");
            };

            var _2023Button = new Button()
            {
                Text = "2023",
                X = Pos.Left(_2022Button),
                Y = Pos.Bottom(_2022Button),
            };

            // When year button is clicked display a message popup
            _2023Button.Clicked += () =>
            {
                MessageBox.Query("2023 Selected", "No Solutions Available", "Ok");
            };


            // Add the views to the Window
            Add(
                yearLabel, _2015Button, _2016Button,
                _2017Button, _2018Button, _2019Button,
                _2020Button, _2021Button, _2022Button,
                _2023Button
                );
        }
    }
}
