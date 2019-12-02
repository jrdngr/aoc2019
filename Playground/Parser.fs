module Parser

open System

// https://fsharpforfunandprofit.com/posts/understanding-parser-combinators/

type Result<'a> =
    | Success of 'a
    | Failure of string

type Parser<'T> = Parser of (string -> Result<'T * string>)

let pchar charToMatch =
    let innerFn str = 
        if String.IsNullOrEmpty(str) then
            Failure "No more input"
        else
            let first = str.[0]    
            if first = charToMatch then
                let remaining = str.[1..]
                Success (charToMatch, remaining)
            else
                let msg = sprintf "Expecting '%c'. Got '%c'" charToMatch first
                Failure msg
    Parser innerFn

let run parser input =
    let (Parser innerFn) = parser
    innerFn input

let andThen parser1 parser2 =
    let innerFn input =
        let result1 = run parser1 input

        match result1 with
        | Failure err -> Failure err
        | Success (value1, remaining1) ->
            let result2 = run parser2 remaining1

            match result2 with
            | Failure err -> Failure err
            | Success (value2, remaining2) ->
                let newValue = (value1, value2)
                Success (newValue, remaining2)
    Parser innerFn

let ( .>>. ) = andThen

let orElse parser1 parser2 =
    let innerFn input =
        let result1 = run parser1 input

        match result1 with
        | Success _ -> result1
        | Failure _ -> run parser2 input

    Parser innerFn

let ( <|> ) = orElse

let choice listOfParsers =
    List.reduce ( <|> ) listOfParsers

let anyOf listOfChars =
    listOfChars
    |> List.map pchar
    |> choice

let Run = 
    let debug = printfn "%A"

    // let parseA = pchar 'A'
    // let parseB = pchar 'B'
    // let parseC = pchar 'C'

    // let parseAthenB = parseA .>>. parseB
    // debug <| run parseAthenB "ABC"
    // debug <| run parseAthenB "ZBC"
    // debug <| run parseAthenB "AZC"

    // let parseAorElseB = parseA <|> parseB
    // debug <| run parseAorElseB "AZZ"
    // debug <| run parseAorElseB "BZZ"
    // debug <| run parseAorElseB "CZZ"

    // let bOrElseC = parseB <|> parseC
    // let aAndThenBorC = parseA .>>. bOrElseC
    // debug <| run aAndThenBorC "ABZ"
    // debug <| run aAndThenBorC "ACZ"
    // debug <| run aAndThenBorC "QBZ"
    // debug <| run aAndThenBorC "AQZ"

    let parseLowercase = anyOf ['a'..'z']
    let parseDigit = anyOf ['0'..'9']

    debug <| run parseLowercase "aBC"
    debug <| run parseLowercase "ABC"

    debug <| run parseDigit "1ABC"
    debug <| run parseDigit "9ABC"
    debug <| run parseDigit "|ABC"

    "Parser"