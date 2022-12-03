function run(s)
    sum_ = 0
    for backpack in split(s, "\n")
        n = length(backpack)
        c1 = backpack[1:n÷2+1]
        c2 = backpack[n÷2+1:n]
        for c in c1
            if c in c2
                if islowercase(c)
                    sum_ += Int(c) - 96
                else
                    sum_ += Int(c) - 64 + 26
                end
                break
            end
        end
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
