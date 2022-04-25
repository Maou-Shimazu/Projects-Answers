fun main(){
    println("Type anything to get the Vowel Count:")
    println(readLine()?.getVowelCount() ?: 0)
}

val vowels = listOf('a','A','e','E','i','I','o','O','u','U') //and sometimes yY ;p
fun String.getVowelCount() = count { c -> vowels.contains(c) }
