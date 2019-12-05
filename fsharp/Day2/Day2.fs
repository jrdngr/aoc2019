module Day2

let setRegister input pos value=
    input
    |> List.mapi (fun i v -> if i = pos then value else v)

type Op =
    | Add of int * int * int
    | Mul of int * int * int
    | Halt

let parseOpAtPosition (input:list<int>) position =
    match input.[position] with
    | 1 -> Ok (Add (input.[input.[position + 1]], input.[input.[position + 2]], input.[position + 3]))
    | 2 -> Ok (Mul (input.[input.[position + 1]], input.[input.[position + 2]], input.[position + 3]))
    | 99 -> Ok Halt
    | _ -> Error (sprintf "Invalid op code: %i" input.[position])

let processOp input op =
    match op with
    | Add (x, y, pos) -> setRegister input pos (x + y)
    | Mul (x, y, pos) -> setRegister input pos (x * y)
    | Halt -> input

let processProgram (noun,verb) =
    let mutable registers = 
        System.IO.File.ReadAllText("Day2/input").Split([|','|])
        |> Seq.map int
        |> Seq.toList

    let mutable currentPosition = 0

    registers <- setRegister registers 1 noun
    registers <- setRegister registers 2 verb

    while parseOpAtPosition registers currentPosition <> Ok Halt do
        match parseOpAtPosition registers currentPosition with
        | Ok op -> 
            registers <- processOp registers op
            currentPosition <- currentPosition + 4
        | Error e -> failwith e
    
    (noun, verb, registers.[0])

let Run = 
    let part2 = 
        [for noun in 0..99 do for verb in 0..99 do yield noun,verb]
        |> List.map processProgram
        |> List.filter (fun (_, _, result) -> result = 19690720)

    let (n, v, _) = part2.[0]
    
    let answer = 100 * n + v 

    sprintf "%A" answer


// Part 1: 7594646
// Part 2: 3376