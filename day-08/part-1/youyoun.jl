function run(s)
    lines = split(s, "\n")
    n = length(lines)
    grid =  zeros(Int, n, n)
    for i in 1:n
        for j in 1:n
            grid[i, j] = parse(Int, lines[i][j])
        end
    end
    s = n * 2 + (n-2) * 2
    for i in 2:n-1
        for j in 2:n-1
            if grid[i, j] > maximum(grid[1:i-1, j]) || grid[i, j] > maximum(grid[i+1:n, j]) || grid[i, j] > maximum(grid[i, 1:j-1]) || grid[i, j] > maximum(grid[i, j+1:n])
                s += 1
            end
        end
    end
    return s
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
