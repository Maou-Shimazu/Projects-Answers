fun main(){
    println("test".isPalindrome()) //False
    println("racecar".isPalindrome()) //True
    println("2442".isPalindrome()) //True
}

//No new string variables are needed
//Uses half length iteration to reduce lookups by half
//Uses predicate search to fail early if any aren't true
fun String.isPalindrome():Boolean{
    return (0..(length/2)).none { this[length - 1 - it] != this[it] }
}
