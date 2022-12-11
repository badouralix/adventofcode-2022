struct Point
    x::Int
    y::Int
end

addCoordinate(A::Point, B::Point) = Point(A.x + B.x, A.y + B.y)
subCoordinate(A::Point, B::Point) = Point(A.x - B.x, A.y - B.y)
Base.hash(x::Point, h::UInt) = hash(x.x, hash(x.y, hash(:Point, h)))
Base.isequal(A::Point, B::Point) = A.x == B.x && A.y == B.y 

DIRECTIONS = Dict(
    "R" => Point(0, 1),
    "U" => Point(1, 0),
    "L" => Point(0, -1),
    "D" => Point(-1, 0)
)

function run(s)
    visitedPoint::Set = Set()
    headPos::Point = Point(0, 0)
    tailPos::Point = Point(0, 0)
    for line in split(s, "\n")
        direction, steps = split(line, " ")
        steps = parse(Int, steps)
        for s in 1:steps
            headPos = addCoordinate(headPos, DIRECTIONS[direction])
            distHeadTail = subCoordinate(headPos, tailPos)
            if max(abs(distHeadTail.x), abs(distHeadTail.y)) > 1
                if distHeadTail.x == 0
                    tailPos = addCoordinate(tailPos, Point(0, sign(distHeadTail.y)))
                elseif distHeadTail.y == 0
                    tailPos = addCoordinate(tailPos, Point(sign(distHeadTail.x), 0))
                else
                    tailPos = addCoordinate(tailPos, Point(sign(distHeadTail.x), sign(distHeadTail.y)))
                end
            end
            push!(visitedPoint, tailPos)
        end
    end
    return length(visitedPoint)
end

#########################################

function main()
    res, time, memory = @timed run(ARGS[1])
    println("_duration:$(time * 1000)")
    println(res)
end

main()
