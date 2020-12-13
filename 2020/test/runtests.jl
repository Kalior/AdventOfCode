using AdventOfCode2020
using Test

include("../src/day1.jl")
include("../src/day2.jl")
include("../src/day4.jl")
include("../src/day5.jl")
include("../src/day6.jl")
include("../src/day7.jl")
include("../src/day8.jl")
include("../src/day11.jl")
include("../src/day12.jl")
include("../src/day13.jl")

@testset "AdventOfCode2020.jl" begin
    @testset "Day one" begin
        @test Day1.sums_to_2020(2020, 0)
        @test Day1.sums_to_2020(2018, 0, 2)
        @test !Day1.sums_to_2020(2018, 0, 3)
        @test Day1.sums_to_2020(1010, 1010)

        input = Day1.get_input()
        @test Day1.solve_part_one(input) == 471019
        @test Day1.solve_part_two(input) == 103927824
    end

    @testset "Day two" begin
        @test Day2.valid_passphrase((1, 3, 'a', "abcde"))
        @test !Day2.valid_passphrase((1, 3, 'b', "cdefg"))
        @test Day2.valid_passphrase((2, 9, 'c', "ccccccccc"))

        @test Day2.valid_passphrase_two((1, 3, 'a', "abcde"))
        @test !Day2.valid_passphrase_two((1, 3, 'b', "cdefg"))
        @test !Day2.valid_passphrase_two((2, 9, 'c', "ccccccccc"))

        input = Day2.get_input()
        @test Day2.solve_part_one(input) == 625
        @test Day2.solve_part_two(input) == 391
    end

    @testset "Day four" begin
        input = Day4.get_input()
        @test Day4.solve_part_one(input) == 208
        @test Day4.solve_part_two(input) == 167
    end

    @testset "Day five" begin
        input = Day5.get_input()
        @test Day5.get_pass_id("FBFBBFFRLR") == 357
        @test Day5.get_pass_id("BFFFBBFRRR") == 567
        @test Day5.get_pass_id("FFFBBBFRRR") == 119
        @test Day5.get_pass_id("BBFFBBFRLL") == 820

        @test Day5.solve_part_one(input) == 896
        @test Day5.solve_part_two(input) == 659
    end

    @testset "Day six" begin
        input = Day6.get_input()
        @test Day6.solve_part_one(input) == 6703
        @test Day6.solve_part_two(input) == 3430
    end


    @testset "Day seven" begin
        input = Day7.get_input()
        @test Day7.solve_part_one(input) == 268
        @test Day7.solve_part_two(input) == 7867
    end

    @testset "Day eight" begin
        input = Day8.get_input()
        @test Day8.solve_part_one(input) == 1451
        @test Day8.solve_part_two(input) == 1160
    end

    @testset "Day eleven" begin
        input = Day11.get_input()
        @test Day11.solve_part_one(input) == 2338
        @test Day11.solve_part_two(input) == 2134
    end

    @testset "Day twelve" begin
        input = Day12.get_input()
        @test Day12.solve_part_one(input) == 521
        @test Day12.solve_part_two(input) == 22848
    end

    @testset "Day thirteen" begin
        @test Day13.time_to_sequential([17,nothing,13,19]) == 3417
        @test Day13.time_to_sequential([67,7,59,61]) == 754018
        @test Day13.time_to_sequential([67,nothing,7,59,61]) == 779210
        @test Day13.time_to_sequential([67,7,nothing,59,61]) == 1261476
        @test Day13.time_to_sequential([1789,37,47,1889]) == 1202161486
        @test Day13.time_to_sequential([41,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,37,nothing,nothing,nothing,nothing,nothing,557,nothing,29,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,13,nothing,nothing,nothing,17,nothing,nothing,nothing,nothing,nothing,23,nothing,nothing,nothing,nothing,nothing,nothing,nothing,419,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,19]) == 598411311431841
    end
end
