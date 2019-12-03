module Day3

type PathSegment =
    | Up of int
    | Down of int
    | Right of int
    | Left of int

let parseSegment (segment:string) =
    let direction = segment.[0]
    let distance = segment.[1..] |> int
    match direction with
    | 'U' -> Up distance
    | 'D' -> Down distance
    | 'R' -> Right distance
    | 'L' -> Left distance
    | _   -> failwith (sprintf "Invalid direction: %c" direction)


let input = 
    System.IO.File.ReadLines("Day3/input") 
    |> Seq.map (fun line -> line.Split([|','|]) |> Array.toSeq)
    |> Seq.toList

let wire1 = input.[0] |> Seq.map parseSegment
let wire2 = input.[1] |> Seq.map parseSegment





let Run = sprintf "%A" wire2