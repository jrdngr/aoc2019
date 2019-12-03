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

let manhattanDistance (x1,y1) (x2,y2) =
    abs(y2 - y1) + abs(x2 - x1)

let distanceFromCenter (x,y) = manhattanDistance (x,y) (0,0)

type Wire = 
    | Wire1
    | Wire2

type Cell =
    | Wire1Present
    | Wire2Present
    | CrossedWires

let wireToCell wire =
    match wire with 
    | Wire1 -> Wire1Present
    | Wire2 -> Wire2Present

let mutable grid: Map<int*int,Cell> = Map.empty

let processCell wire position =
    if Map.containsKey position grid then
        grid <- match Map.find position grid with
                | Wire1Present when wire = Wire2 -> grid.Add(position, CrossedWires)
                | Wire2Present when wire = Wire1 -> grid.Add(position, CrossedWires)
                | _ -> grid
    else 
        grid <- grid.Add(position, wireToCell wire)

let targetPosition (startX, startY) segment=
   match segment with
   | Up dist    -> (startX, startY + dist)
   | Down dist  -> (startX, startY - dist)
   | Right dist -> (startX + dist, startY)
   | Left dist  -> (startX - dist, startY)

let addSegment wire (startX, startY) segment =
   let endX, endY = targetPosition (startX, startY) segment
   Seq.zip [for x in startX..endX do yield x] [for y in startY..endY do yield y]
   |> Seq.iter (processCell wire)
   (endX, endY)

let Run = sprintf "%A" wire2