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
    nKnots = 10
    visitedPoint::Set = Set()
    knots = [Point(0, 0) for i in 1:nKnots]
    for line in split(s, "\n")
        direction, steps = split(line, " ")
        steps = parse(Int, steps)
        for s in 1:steps
            knots[nKnots] = addCoordinate(knots[nKnots], DIRECTIONS[direction])
            for i in nKnots-1:-1:1
                distHeadTail = subCoordinate(knots[i+1], knots[i])
                if max(abs(distHeadTail.x), abs(distHeadTail.y)) > 1
                    if distHeadTail.x == 0
                        knots[i] = addCoordinate(knots[i], Point(0, sign(distHeadTail.y)))
                    elseif distHeadTail.y == 0
                        knots[i] = addCoordinate(knots[i], Point(sign(distHeadTail.x), 0))
                    else
                        knots[i] = addCoordinate(knots[i], Point(sign(distHeadTail.x), sign(distHeadTail.y)))
                    end
                end
            push!(visitedPoint, knots[1])
            end
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
