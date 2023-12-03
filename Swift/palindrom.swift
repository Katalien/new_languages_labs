import Swift
import Foundation

func isPalindrome(_ s: inout String, _ startIdx: String.Index, _ endIdx: String.Index) -> Bool {
    guard startIdx < endIdx else {
        return true
    }
    
    if s[startIdx] == s[endIdx] {
        let nextStartIdx = s.index(after: startIdx)
        let prevEndIdx = s.index(before: endIdx)
        return isPalindrome(&s, nextStartIdx, prevEndIdx)
    } else {
        return false
    }
}

print("You're going to check if your word is a palindrome.\n")
print("You can enter a word from console or read it from a file.\nType 'c' for console and 'f' for file")
if let choice = readLine() {
    if choice == "c" {
        while true {
            print("Enter a string or click Enter to finish:")
            if let input = readLine(), !input.isEmpty {
                var stringToCheck = input.lowercased().filter { $0.isLetter }
                let startIndex = stringToCheck.startIndex
                let endIndex = stringToCheck.index(before: stringToCheck.endIndex)
                if isPalindrome(&stringToCheck, startIndex, endIndex) {
                    print("\(input) is a palindrome!\n")
                } else {
                    print("\(input) is NOT a palindrome!\n")
                }
            } else {
                break
            }
        }
    } else if choice == "f" {
        print("Enter a filename (with extension)")
        if let fileName = readLine(), !fileName.isEmpty {
            do {
                let content = try String(contentsOfFile: fileName, encoding: .utf8)
                var stringToCheck = content.lowercased().filter { $0.isLetter }
                let startIndex = stringToCheck.startIndex
                let endIndex = stringToCheck.index(before: stringToCheck.endIndex)
                if isPalindrome(&stringToCheck, startIndex, endIndex) {
                    print("\(content) is a palindrome!")
                } else {
                    print("\(content) is NOT a palindrome!")
                }
            } catch {
                print("Error while reading a file:\n", error)
            }
        }
    }
}
print("Goodbye!")


