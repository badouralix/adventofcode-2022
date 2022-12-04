function run(s)
    n_overlap = 0
    for assignment in split(s, "\n")
        start_1, end_1, start_2, end_2 =  map(x -> parse(Int, x), match(r"(\w+)-(\w+),(\w+)-(\w+)", assignment).captures)
        if (end_1 - start_2) * (end_2 - start_1) >= 0
             n_overlap += 1
         end
    end
    return n_overlap
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time*1000)")
    println(res)
end

main()
