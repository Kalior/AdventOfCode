interface Day1
    exposes [
        solveDay1,
    ]
    imports [
        pf.Stdout,
        pf.Stderr,
        pf.File,
        pf.Path,
        pf.Task.{ Task },
    ]
    

parseCalibration = \line ->
    characters = Str.graphemes line

    first = List.findFirst characters (\c -> Str.toI32 c |> Result.isOk) |> Result.withDefault "0"
    last = List.findLast characters (\c -> Str.toI32 c |> Result.isOk) |> Result.withDefault "0"

    Str.toI32 "\(first)\(last)" |> Result.withDefault 0

replaceNumbers = \line ->
    Str.replaceEach line "one" "one1one" 
    |> Str.replaceEach "two" "two2two" 
    |> Str.replaceEach "three" "three3three"
    |> Str.replaceEach "four" "four4four"
    |> Str.replaceEach "five" "five5five"
    |> Str.replaceEach "six" "six6six"
    |> Str.replaceEach "seven" "seven7seven"
    |> Str.replaceEach "eight" "eight8eight"
    |> Str.replaceEach "nine" "nine9nine"

solveDay1 =
    input <- File.readUtf8 (Path.fromStr "inputs/day1") |> Task.await

    sum = Str.trim input |> Str.split "\n" |> List.map parseCalibration |> List.sum

    sum2 = Str.trim input |> Str.split "\n" |> List.map replaceNumbers |> List.map parseCalibration |> List.sum

    
    Stdout.line
        """
        Part 1: \(Num.toStr sum)
        Part 2: \(Num.toStr sum2)
        """
