fun main(){
    println(121.sieve())
}

fun Int.sieve(): MutableList<Int> {
    val primes = mutableListOf<Int>()
    var nums = (2..this).toMutableList()

    while (nums.isNotEmpty()){
        val removeBy = nums.first()

        primes.add(removeBy)

        val newNums = mutableListOf<Int>()

        for (i in nums){
            if (i % removeBy != 0) newNums.add(i)
        }
        nums = newNums
    }

    return primes
}
