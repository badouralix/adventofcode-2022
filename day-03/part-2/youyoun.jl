function run(s)
    sum_ = 0
    parsedS = split(s, "\n")
    N = length(parsedS)
    for i in 1:3:N
        c1 = parsedS[i]
        c2 = parsedS[i+1]
        c3 = parsedS[i+2]
        for c in c1
            if c in c2 && c in c3
                if islowercase(c)
                    sum_ += Int(c) - 96
                else
                    sum_ += Int(c) - 64 + 26
                end
                break
            end
        end
#         break
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
