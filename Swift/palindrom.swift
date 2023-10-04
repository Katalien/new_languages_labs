import Swift
import Foundation

func isPalindrome(_ s: String) -> Bool {
    if s.count < 2 {
        return true
    } else {
        if s.first! == s.last! {
            let startIndex = s.index(after: s.startIndex)
            let endIndex = s.index(before: s.endIndex)
            return isPalindrome(String(s[startIndex..<endIndex]))
        } else {
            return false
        }
    }
}



print("You're going to check if your word is a palindrom.\n")
print("You can enter a word from console or read it from file.\n Type 'c' for conslole and 'f' for file")
let choice = readLine()

if choice == "c"{
    while true {
        print("Enter a string or click Enter to finish:")
        if let input = readLine() {
            if input.isEmpty {
                break
            }
            let cleanedInput = input.lowercased().filter { $0.isLetter }
            if isPalindrome(cleanedInput) {
                print("\(input) is a palindrom!\n")
            } else {
                print("\(input) is NOT a palindrom!\n")
            }
        }
    }
}
if choice == "f"{
    print("Enter a filename (with extension)")
    if let fileName = readLine() {
        if !fileName.isEmpty {
           do {
                let content = try String(contentsOfFile: fileName, encoding: .utf8)
                let fileContent = content
            
                if isPalindrome(fileContent) {
                    print("\(fileContent) is a palindrom!")
                }
                else{
                    print("\(fileContent) is NOT a palindrom!")
                }
            } catch {
                print("Error while reading a file:\n", error)
            }
        }
    }
    
}
print("Goodbye!")