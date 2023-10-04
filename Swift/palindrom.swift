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



