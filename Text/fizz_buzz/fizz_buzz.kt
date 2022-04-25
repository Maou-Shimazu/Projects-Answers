fun main(){
    //Direct usage
    0.fizzBuzz()

    //An input usage
    println("Enter a number to start FizzBuzz")
    (readLine()?.takeIf {it.isNotEmpty() && it.all(Char::isDigit)}?.toInt() ?: 0).fizzBuzz()
}

fun Int.fizzBuzz(){
    for (i in 0..this){
        println(when {
            i % 15 == 0 -> "FizzBuzz"
            i % 3  == 0 -> "Fizz"
            i % 5  == 0 -> "Buzz"
            else -> i
        })
    }
}
