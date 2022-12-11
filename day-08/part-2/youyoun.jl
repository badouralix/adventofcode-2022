function run(s)
    lines = split(s, "\n")
    n = length(lines)
    grid =  zeros(Int, n, n)
    for i in 1:n
        for j in 1:n
            grid[i, j] = parse(Int, lines[i][j])
        end
    end

    maxScenicScore = 4
    for i in 2:n-1
        for j in 2:n-1
            scenicScore = 1

            directions = [grid[i, j] .> grid[i-1:-1:1, j], grid[i, j] .> grid[i+1:n, j], grid[i, j] .> grid[i, j-1:-1:1], grid[i, j] .> grid[i, j+1:n]]
            for top in directions
                s = 0
                for k in 1:length(top)
                    s += 1
                    if top[k] == 0
                        break
                    end
                end
                scenicScore *= s
            end

            if scenicScore > maxScenicScore
                maxScenicScore = scenicScore
            end
        end
    end
    return maxScenicScore
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
