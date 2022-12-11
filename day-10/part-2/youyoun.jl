function run(s)
    X::Int = 1
    width = 40
    height = 6
    grid::Array{String, 1} = fill(".", width * height)

    CRTpos = 0
    for line in split(s, "\n")
        command = split(line, " ")
        cycles = 1
        move = 0
        if command[1] == "addx"
            cycles = 2
            move = parse(Int, command[2])
        end
        for _ in 1:cycles
            if mod(CRTpos, width) <= X + 1 && mod(CRTpos, 40) >= X - 1
                grid[CRTpos+1] = "#"
            end
            CRTpos += 1
        end
        X += move
    end
    reshapedGrid = reshape(grid, (height, width))
    return join([join(reshapedGrid[(i-1)*width + 1:i*width], "") for i in 1:height], "\n")
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_parse")
    println(res)
    println("_duration:$(time*1000)")
end

main()
