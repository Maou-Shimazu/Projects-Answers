fun main(){
    //Example usage:
    5.fibonacci()
    //Outputs
    //0,1,1,2,3
}

fun Int.fibonacci() = fibonacciSequence(this)

fun fibonacciSequence(n:Int){
    val list = mutableListOf(0,1)

    when {
        n <= 0 -> println("Undefined")
        n <= 2 -> println(list.take(n).joinToString(","))
        else -> {
            repeat(n - 2){
                val last = list.size - 1
                list.add(list[last] + list[last - 1])
            }
            println(list.joinToString(","))
        }
    }
}
