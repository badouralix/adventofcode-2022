function run(s)
    return maximum(map(x -> sum(x), map(x -> map(x -> parse(Int, x), split(x, "\n")), split(s, "\n\n"))))
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
