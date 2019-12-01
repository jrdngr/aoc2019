open System

let run dayNumber =
    printfn "Running day #%s..." dayNumber
    let result = match dayNumber with
                 | "1" -> Day1.Run
                 | _ -> "Invalid day number"

    printfn "%s" result

[<EntryPoint>]
let main argv =
    match argv |> Array.toList with
    | dayNumber :: _ -> run dayNumber
    | _ -> printfn "Please provide a day number as the first argument"
    0
