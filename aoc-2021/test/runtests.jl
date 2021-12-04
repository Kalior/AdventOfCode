using Test
using BenchmarkTools

include("../src/day1.jl")
include("../src/day2.jl")
include("../src/day3.jl")
include("../src/day4.jl")

@testset "2021" begin
    @testset "Day one" begin
        input = Day1.parse_input()
        @test Day1.solve_part_one(input) == 1681
        @test Day1.solve_part_two(input) == 1704
        #@benchmark Day1.solve() setup = (input = Day1.parse_input())
    end

    @testset "Day two" begin
        input = Day2.parse_input()
        @test Day2.solve_part_one(input) == 1499229
        @test Day2.solve_part_two(input) == 1340836560
        #@benchmark Day2.solve() setup = (input = Day2.parse_input())
    end

    @testset "Day three" begin
        input = Day3.parse_input()
        @test Day3.solve_part_one(input) == 1131506
        @test Day3.solve_part_two(input) == 7863147
        #@benchmark Day3.solve() setup = (input = Day3.parse_input())
    end

    @testset "Day four" begin
        numbers, boards = Day4.parse_input()
        @test Day4.solve_part_one(boards, numbers) == (17, 45031)
        @test Day4.solve_part_two(boards, numbers) == (87, 2568)
        #@benchmark Day4.solve() setup = (input = Day4.parse_input())
    end
end
