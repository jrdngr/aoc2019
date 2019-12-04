module Day4

open System.Collections.Generic

let lastDigit num = num % 10
let removeLastDigit num = num / 10

let nextDigit num =
    (lastDigit num, removeLastDigit num)

let digits num =
    let digits = new List<int>()

    let mutable remaining = num
    while remaining > 0 do
        digits.Add(remaining % 10)
        remaining <- remaining / 10

    digits.Reverse()

    digits

let isSixDigits num =
    num >= 100_000 && num <= 999_999

let inRange num =
    num >= 359_282 && num <= 820_401

let hasDouble num = true

let digitsIncreasing = true

let isValid num =
    num

let Run = sprintf "%A" (digits 152356)
