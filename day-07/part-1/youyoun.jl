
function run(s)
    limit = 100000
    lines = split(s, "\n")
    directories = ["/"]
    sizes::Dict{String, Int} = Dict("/" => 0)
    for line in lines
        if occursin("\$ cd", line)
            chdir = match(r"\$ cd (.*)", line).captures[1]
            if chdir == "/"
                directories = ["/"]
            elseif chdir == ".."
                pop!(directories)
            else
                dir_path = directories[lastindex(directories)] * "/" * chdir
                push!(directories, dir_path)
                sizes[dir_path] = 0
            end
        else
            if occursin("\$ ls", line)
                continue
            else
                parsed_ls = match(r"(dir|\w+) (.*)", line).captures
                if parsed_ls[1] == "dir"
                    continue
                else
                    for dir in directories
                        sizes[dir] += parse(Int, parsed_ls[1])
                    end
                end
            end
        end
    end
    s = 0
    for (_, size) in sizes
        if size <= limit
            s += size
        end
    end
    return s
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time * 1000)")
    println(res)
end

main()
