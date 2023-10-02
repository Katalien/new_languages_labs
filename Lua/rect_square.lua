function calculateRectangleArea(topLeftX, topLeftY, bottomRightX, bottomRightY)
    local width = bottomRightX - topLeftX
    local height = topLeftY - bottomRightY
    return math.abs(width * height)
end

function processFile(inputFileName)
    local inputFile = io.open("./input/" .. inputFileName, "r")
    if inputFile then
        local coordinates = {}
        for line in inputFile:lines() do
            for num in line:gmatch("%S+") do
                table.insert(coordinates, tonumber(num))
            end
        end
        inputFile:close()

        local area = calculateRectangleArea(unpack(coordinates))
        print(inputFileName)
        local outputFileName = "output/" .. inputFileName
        local outputFile = io.open(outputFileName, "w")
        if outputFile then
            outputFile:write("Area of the rectangle: " .. area)
            outputFile:close()
            print("Area calculated and written to " .. outputFileName)
        else
            print("Error: Unable to open output file.")
        end
    else
        print("Error: Unable to open input file.")
    end
end


print("Do you want to read coordinates from .txt file or console? \nType 'c' for console or 'f' for file")
mode = io.read("*l")
if mode == 'f' then
    print("Enter filename.\nIt should be located in folder ./input/")
    filename = io.read("*l")
    local inputFileName = "./input/" .. filename
    processFile(filename)
elseif mode == 'c' then
    print("Enter for coordinates of the rectangle.\nEx. x1 y1 x2 y2")
    x_top, y_top, x_bot, y_bot = io.read("*n","*n", "*n","*n")
    print("Rectangle area:", calculateRectangleArea(x_top, y_top, x_bot, y_bot))
else
    print("No such command")
end