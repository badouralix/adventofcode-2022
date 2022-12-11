function run(s)
    X::Int = 1
    nCycles::Int = 1
    cycleThresh = 20
    cycleThreshStep = 40
    cycleLastThresh = 220

    signalStrength::Int = 0
    for line in split(s, "\n")
        command = split(line, " ")
        if command[1] == "noop"
            nCycles += 1
        elseif command[1] == "addx"
            nCycles += 2
            X += parse(Int, command[2])
        end
        if nCycles == cycleThresh
            signalStrength += cycleThresh * X
            cycleThresh += cycleThreshStep
        elseif nCycles == cycleThresh + 1
            signalStrength += cycleThresh * (X - parse(Int, command[2]))
            cycleThresh += cycleThreshStep
        end
        if nCycles > cycleLastThresh
            break
        end
    end
    return signalStrength
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
