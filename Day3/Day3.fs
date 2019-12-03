module Day3

let input = 
    System.IO.File.ReadLines("Day3/input") 
    |> Seq.map (fun line -> line.Split([|','|]))
    |> Seq.toList

let wire1 = input.[0]
let wire2 = input.[1]

let Run = sprintf "%A" wire2