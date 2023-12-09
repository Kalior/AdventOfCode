app "adventofcode"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.7.0/bkGby8jb0tmZYsy2hg1E_B2QrCgcSTxdUlHtETwm5m4.tar.br" }
    imports [
        pf.Stdout,
        pf.Stderr,
        pf.File,
        pf.Path,
        pf.Task.{ Task },
    ]
    provides [main] to pf


    
Card : {left: Set Nat, right: Set Nat}

unwrap = \value, message ->
    when value is 
        Ok v -> v
        _ -> crash message

debug = \value ->
    dbg value
    value

parseNumbers: Str -> Set Nat
parseNumbers = \rawCards ->
    rawCards |> Str.trim |> Str.replaceEach "  " " " |> Str.split " " |> List.keepOks Str.toNat |> Set.fromList

parseCard: Str -> Card
parseCard = \card ->
    {before: _, after: cards} = Str.splitFirst card ": " |> unwrap "Couldn't split on :"
    {before: winningNumbers, after: drawNumbers} = Str.splitFirst cards " | " |> unwrap "Couldn't split on |"
    

    {left: parseNumbers winningNumbers, right: parseNumbers drawNumbers}

parse: Str -> List Card
parse = \lines -> 
    lines |> Str.trim |> Str.split "\n" |> List.map parseCard

calcScore = \len -> 
    when len is
        0 -> 0
        x -> Num.powInt 2 (x - 1)

solvePartOne: List Card -> Nat
solvePartOne = \cards ->
    List.map cards (\card -> (Set.intersection card.left card.right) |> Set.len |> calcScore) |> List.sum

reduce = \acc, nWins, index ->
    nRepeats = List.get acc index |> unwrap "couldn't get index"
    
    List.range {start: After index, end: Length nWins}
        |> List.walk acc (\a, i -> List.update a i (\x -> x + nRepeats))

solvePartTwo: List Card -> Nat
solvePartTwo = \cards ->
    nWinsPerCard = List.map cards (\card -> (Set.intersection card.left card.right) |> Set.len)

    nCards = List.repeat 1 (List.len cards)

    List.walkWithIndex nWinsPerCard nCards reduce |> List.sum
    

solve =
    lines <- File.readUtf8 (Path.fromStr "../inputs/day4") |> Task.await
    
    cards = parse lines

    sum = solvePartOne cards

    sum2 = solvePartTwo cards

    
    Stdout.line
        """
        Part 1: \(Num.toStr sum)
        Part 2: \(Num.toStr sum2)
        """

main =
    solve
    |> Task.onErr \e ->
        dbg e
        Stderr.line "Something went wrong!"
    
