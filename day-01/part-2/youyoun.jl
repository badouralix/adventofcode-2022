function run(s)
    return sum(partialsort(map(x -> sum(x), map(x -> map(x -> parse(Int, x), split(x, "\n")), split(s, "\n\n"))), 1:3, rev=true))
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
