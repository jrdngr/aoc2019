module Day2

let input = 
    System.IO.File.ReadAllText("Day2/input").Split([|','|])
    |> Seq.map int

let Run = 
    "Day 2"