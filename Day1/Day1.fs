module Day1

    open System 
    
    let input = 
        System.IO.File.ReadLines("Day1/input") 
        |> Seq.map int
    
    let fuelByWeight weight = (weight / 3) - 2

    let rec totalFuel currentTotal fuelWeight =
        if fuelWeight <= 0 then currentTotal
        else totalFuel (currentTotal + fuelWeight) (fuelByWeight fuelWeight)
    
    let Run =
        input
        |> Seq.map fuelByWeight
        |> Seq.sumBy (totalFuel 0)
        |> string
        
// Part 1: 3252897    
// Part 2: 4876469