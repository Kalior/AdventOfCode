module Day18

include("InputHelper.jl")

function get_input()
    InputHelper.parse(line -> line, "18", "\n")
end

function solve()
    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    sum(map(evaluate, input))
end

function evaluate(line)
    if line[1] == '('
        close = find_closing_parentheses(line, 1)
        lhs = evaluate(line[2:close - 1])
        i = close + 1
    else
        lhs = parse(Int, line[1])
        i = 2
    end

    op = +
    while i <= length(line)
        c = line[i]
        if c == '('
            close = find_closing_parentheses(line, i)
            rhs = evaluate(line[i + 1:close - 1])
            lhs = op(lhs, rhs)
            i = close
        elseif c == '+'
            op = +
        elseif c == '*'
            op = *
        elseif c != ' '
            rhs = parse(Int, c)
            lhs = op(lhs, rhs)
        end
        i += 1
    end
    lhs
end

function find_closing_parentheses(line, start_at)
    n_open = 0
    for (i, c) in enumerate(line[start_at:end])
        if c == '('
            n_open += 1
        elseif c == ')'
            n_open -= 1
            if n_open == 0
                return i + start_at - 1
            end
        end
    end
    -1
end

function solve_part_two(input)
    sum(map(evaluate_plusses, input))
end

function evaluate_plusses(line)
    new_line = join(["($plus_expr)" for plus_expr in split(line, " * ")], " * ")
    Main.eval(Meta.parse(new_line))
end

end
