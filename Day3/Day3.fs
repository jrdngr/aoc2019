module Day3

// This is terrible

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
    printfn "%A, %A" (fst position) (snd position)
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
    let xDiff = endX - startX
    let yDiff = endY - startY
    let steps = max xDiff yDiff
    let xs = if xDiff > 0 then [for x in startX..endX do yield x] else [for _ in 0..steps do yield endX]
    let ys = if yDiff > 0 then [for y in startY..endY do yield y] else [for _ in 0..steps do yield endY]
    Seq.zip xs ys
    |> Seq.iter (processCell wire)
    (endX, endY)

let getDistance w1 w2 =
    printfn "%A" w1
    Seq.fold (fun pos seg -> addSegment Wire1 pos seg) (0, 0) w1 |> ignore
    printfn "%A" w2
    Seq.fold (fun pos seg -> addSegment Wire2 pos seg) (0, 0) w2 |> ignore
    let crosses = Map.filter (fun _ value -> value = CrossedWires) grid

    crosses

let Test1 = 
    // distance = 6
    let w1 = "R8,U5,L5,D3".Split([|','|]) |> Array.map parseSegment
    let w2 = "U7,R6,D4,L4".Split([|','|]) |> Array.map parseSegment
    let crosses = getDistance w1 w2
    sprintf "%A" crosses

// let Test2 =
//     // distance = 159
//     let w1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".Split([|','|]) |> Array.map parseSegment
//     let w2 = "U62,R66,U55,R34,D71,R55,D58,R83".Split([|','|]) |> Array.map parseSegment
//     let crosses = getDistance w1 w2
//     sprintf "%A" (distanceFromCenter (158,-12))

// let Test3 =
//     // distance = 135
//     let w1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".Split([|','|]) |> Array.map parseSegment
//     let w2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".Split([|','|]) |> Array.map parseSegment
//     let crosses = getDistance w1 w2
//     sprintf "%A" crosses

let Run =
    printfn "%A" Test1
    // printfn "%A" Test2
    // let crosses = getDistance wire1 wire2
    // sprintf "%A" crosses
    ""
