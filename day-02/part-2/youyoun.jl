function run(s)
    scores = Dict(
        "A X" => 3+0,
        "A Y" => 1+3,
        "A Z" => 2+6,
        "B X" => 1+0,
        "B Y" => 2+3,
        "B Z" => 3+6,
        "C X" => 2+0,
        "C Y" => 3+3,
        "C Z" => 1+6,
    )
    sum_ = 0
    for x in split(s, "\n")
        sum_ += scores[x]
    end
    return sum_
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
