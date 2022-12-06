function charCounter(s)
    res = Dict{Char, Int}()
    for c in s
        if haskey(res, c)
            res[c] += 1
        else
            res[c] = 1
        end
    end
    return res
end

function run(s)
    packet_len = 14
    for i in 1:length(s) - packet_len
        marker = s[i:i + packet_len - 1]
        counter = charCounter(marker)
        if length(counter) == packet_len
            return i + packet_len - 1
        end
    end
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time * 1000)")
    println(res)
end

main()
