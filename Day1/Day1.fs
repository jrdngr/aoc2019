module Day1

    open System
    
    let input = 
        System.IO.File.ReadLines("Day1/day1input") 
        |> Seq.map int
    
    let fuelByWeight weight = (weight / 3) - 2

    let moduleFuel =
        input
        |> Seq.sumBy fuelByWeight

    // 3252897    

    let rec totalFuel currentTotal fuelWeight =
        if fuelWeight <= 0 then currentTotal
        else totalFuel (currentTotal + fuelWeight) (fuelByWeight fuelWeight)
    
    let Run =
        input
        |> Seq.map fuelByWeight
        |> Seq.sumBy (totalFuel 0)
        
// 4879304 (wrong because I didn't run it per module, I ran it on the output from part 1)
// 4876469