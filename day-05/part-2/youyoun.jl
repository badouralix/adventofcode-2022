try
    using DataStructures
catch e
    using Pkg
    Pkg.add("DataStructures")
    using DataStructures
end

function parseInput(s)
    cargos, moves = split(s, "\n\n")
    cargos = split(cargos, "\n")
    moves = split(moves, "\n")
    return cargos, moves
end

function getCargosStacks(cargosString)
    n_cargos = (length(cargosString[lastindex(cargosString)]) + 2) ÷ 4
    stacks = [Stack{Char}() for _ in 1:n_cargos]
    cargosIdx = 2:4:n_cargos * 4
    for cargoLine in Iterators.reverse(cargosString[1:lastindex(cargosString) - 1])
        for idx in 1:n_cargos
            try
                letter = cargoLine[cargosIdx[idx]]
                if letter != ' '
                    push!(stacks[idx], letter)
                end
            catch e
                continue
            end
        end
    end
    return stacks
end

function run(s)
    # Your code here
    cargos, moves = parseInput(s)
    stacks = getCargosStacks(cargos)
    for move in moves
        n_crates, start_cargo, end_cargo = map(x -> parse(Int, x), match(r"move (\w+) from (\w+) to (\w+)", move).captures)
        transitionStack = Stack{Char}()
        for _ in 1:n_crates
            push!(transitionStack, pop!(stacks[start_cargo]))
        end
        while !isempty(transitionStack)
            push!(stacks[end_cargo], pop!(transitionStack))
        end
    end
    payload = ""
    for s in stacks
        payload = payload * pop!(s)
    end
    return payload
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
