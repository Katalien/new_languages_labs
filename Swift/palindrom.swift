import Foundation

func isPalindrome(_ s: String, _ startIdx: String.Index, _ endIdx: String.Index) -> Bool {
    if startIdx >= endIdx {
        return true
    } else {
        if s[startIdx] == s[endIdx] {
            let nextStartIdx = s.index(after: startIdx)
            let prevEndIdx = s.index(before: endIdx)
            return isPalindrome(s, nextStartIdx, prevEndIdx)
        } else {
            return false
        }
    }
}

print("You're going to check if your word is a palindrome.\n")
print("You can enter a word from console or read it from a file.\nType 'c' for console and 'f' for file")
if let choice = readLine() {
    if choice == "c" {
        while true {
            print("Enter a string or click Enter to finish:")
            if let input = readLine() {
                if input.isEmpty {
                    break
                }
                let cleanedInput = input.lowercased().filter { $0.isLetter }
                let startIndex = cleanedInput.startIndex
                let endIndex = cleanedInput.index(before: cleanedInput.endIndex)
                if isPalindrome(cleanedInput, startIndex, endIndex) {
                    print("\(input) is a palindrome!\n")
                } else {
                    print("\(input) is NOT a palindrome!\n")
                }
            }
        }
    } else if choice == "f" {
        print("Enter a filename (with extension)")
        if let fileName = readLine(), !fileName.isEmpty {
            do {
                let content = try String(contentsOfFile: fileName, encoding: .utf8)
                let fileContent = content.lowercased().filter { $0.isLetter }
                let startIndex = fileContent.startIndex
                let endIndex = fileContent.index(before: fileContent.endIndex)
                if isPalindrome(fileContent, startIndex, endIndex) {
                    print("\(fileContent) is a palindrome!")
                } else {
                    print("\(fileContent) is NOT a palindrome!")
                }
            } catch {
                print("Error while reading a file:\n", error)
            }
        }
    }
}
print("Goodbye!")


