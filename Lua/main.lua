function merge(left, right)
    local result = {}
    local leftIndex, rightIndex = 1, 1

    while leftIndex <= #left and rightIndex <= #right do
        if left[leftIndex] < right[rightIndex] then
            table.insert(result, left[leftIndex])
            leftIndex = leftIndex + 1
        else
            table.insert(result, right[rightIndex])
            rightIndex = rightIndex + 1
        end
    end

    while leftIndex <= #left do
        table.insert(result, left[leftIndex])
        leftIndex = leftIndex + 1
    end

    while rightIndex <= #right do
        table.insert(result, right[rightIndex])
        rightIndex = rightIndex + 1
    end

    return result
end

function mergeSort(arr)
    local length = #arr

    if length <= 1 then
        return arr
    end

    local mid = math.floor(length / 2)
    local left = {}
    local right = {}

    for i = 1, mid do
        table.insert(left, arr[i])
    end

    for i = mid + 1, length do
        table.insert(right, arr[i])
    end

    left = mergeSort(left)
    right = mergeSort(right)

    return merge(left, right)
end

function readWordsFromFile(filename)
    local file = io.open(filename, "r")
    if file then
        local content = file:read("*a")
        file:close()
        return content
    else
        return nil
    end
end

function main()
    local content = readWordsFromFile("input.txt")

    if content then
        local inputArray = {}
        for word in content:gmatch("%S+") do
            table.insert(inputArray, word)
        end

        if #inputArray > 0 then
            print("Введенные слова:")
            print(table.concat(inputArray, " "))

            local sortedArray = mergeSort(inputArray)

            print("\nОтсортированные слова:")
            print(table.concat(sortedArray, " "))
        else
            print("Массив пуст.")
        end
    else
        print("Не удалось прочитать файл 'input.txt'")
    end
end

main()