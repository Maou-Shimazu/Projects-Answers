fun main(){
    42.getCollatz()
}

fun Int.getCollatz(){
    println("Starting Collatz on: $this")
    var a = this
    var b = 0
    while (a != 1){
        when {
            a % 2 == 0 -> a /= 2
            else -> a = (a * 3) + 1
        }
        print("$a ")
        b++
    }
    print("\nSteps required: $b\n")
}
