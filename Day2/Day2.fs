module Day2

let input = 
    System.IO.File.ReadAllText("Day2/input").Split([|','|])
    |> Seq.map int
    |> Seq.toList

type Op =
    | Add of int * int * int
    | Mul of int * int * int
    | Halt

let parseOpAtPosition (input:list<int>) position =
    match input.[position] with
    | 1 -> Add (input.[position + 1], input.[position + 2], input.[position + 3])
    | 2 -> Mul (input.[position + 1], input.[position + 2], input.[position + 3])
    | 99 -> Halt
    | _ -> raise (System.ArgumentException("Nope"))

let processOp input op =
    match op with
    | Add (x, y, res) -> 
        input 
        |> List.map (fun ())
    | Mul (x, y, res) -> Set (x * y, res)
    | Halt -> Finish


let Run = 
    "Day 2"
