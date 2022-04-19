//Uppercase A starts at 65 | Ends at 90
val upper = 65..90
//Lowercase a starts at 97 | Ends at 122
val lower = 97..122

//Encrypt/Decrypt are just shifting by opposite directions (It works with negatives)
fun main(){
    val original = "Hello World"
    val encrypted = original.shiftRight(2)
    val decrypted = encrypted.shiftLeft(2)
    println(encrypted) //Outputs Jgnnq Yqtnf
    println(decrypted) //Outputs Hello World
}

//It shifts the Ascii value, and uses modulus to keep it within bounds
fun String.shiftRight(amount:Int) = map { c ->
    val d = c.code

    when {
        upper.contains(d) -> {
            val shift = (d + amount) % upper.last
            when {
                shift < upper.first -> (shift + upper.first).toChar()
                else -> shift.toChar()
            }
        }
        lower.contains(d) -> {
            val shift = (d + amount) % lower.last
            when {
                shift < lower.first -> (shift + lower.first).toChar()
                else -> shift.toChar()
            }
        }
        else -> c
    }
}.joinToString("")

fun String.shiftLeft(amount:Int) = map { c ->
    val d = c.code

    when {
        upper.contains(d) -> if (d - amount < upper.first) (upper.last + upper.first - amount - d).toChar() else (d - amount).toChar()
        lower.contains(d) -> if (d - amount < lower.first) (lower.last + lower.first - amount - d).toChar() else (d - amount).toChar()
        else -> c
    }
}.joinToString("")
