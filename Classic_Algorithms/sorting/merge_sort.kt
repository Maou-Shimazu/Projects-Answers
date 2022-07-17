import kotlin.random.Random //Only used for Testing

/**
 * Inline function to void usage of [IntRange] and [IntProgression]
 */
inline fun Int.repeatIndexed(action: (Int) -> Unit){
    var i = 0

    while (i < this){
        action(i)
        ++i
    }
}

/**
 * Sorts and merges two sub-arrays within an [Array]
 *
 * First sub-array is the range `start .. mid`
 * Second sub-array is of range `mid+1 .. end`
 */
private fun merge(col: Array<Int>, start: Int, mid: Int, end: Int) {
    /**
     * Gets each range for the Sub-Arrays
     */
    val left = mid - start + 1
    val right = end - mid

    /**
     * Creates temporary arrays for each Sub-Array
     */
    val tempLeft = IntArray(left)
    val tempRight = IntArray(right)

    /**
     * Copies data to each Sub-Array
     */
    left.repeatIndexed { tempLeft[it] = col[start + it] }
    right.repeatIndexed { tempRight[it] = col[mid + 1 + it] }

    //=========================================================
    //Below is to compare, sort, and merge the temporary arrays
    //=========================================================

    /**
     * Variables used to index the Sub-Arrays
     */
    var i = 0
    var j = 0
    var k = start

    /**
     * Quick-Sorts each element from the currently split Sub-Arrays
     */
    while (i < left && j < right) {
        if (tempLeft[i] <= tempRight[j]) {
            col[k] = tempLeft[i]
            ++i
        } else {
            col[k] = tempRight[j]
            ++j
        }
        ++k
    }

    /**
     * Copies remaining elements, from each Sub-Array, back to the parent [Array]
     */
    while (i < left) {
        col[k] = tempLeft[i]
        ++i
        ++k
    }

    while (j < right) {
        col[k] = tempRight[j]
        ++j
        ++k
    }
}

/**
 * Sorts and merges two sub-arrays within an [IntArray]
 *
 * First sub-array is the range `start .. mid`
 * Second sub-array is of range `mid+1 .. end`
 */
private fun merge(col: IntArray, start: Int, mid: Int, end: Int) {
    /**
     * Gets each range for the Sub-Arrays
     */
    val left = mid - start + 1
    val right = end - mid

    /**
     * Creates temporary arrays for each Sub-Array
     */
    val tempLeft = IntArray(left)
    val tempRight = IntArray(right)

    /**
     * Copies data to each Sub-Array
     */
    left.repeatIndexed { tempLeft[it] = col[start + it] }
    right.repeatIndexed { tempRight[it] = col[mid + 1 + it] }

    //=========================================================
    //Below is to compare, sort, and merge the temporary arrays
    //=========================================================

    /**
     * Variables used to index the Sub-Arrays
     */
    var i = 0
    var j = 0
    var k = start

    /**
     * Quick-Sorts each element from the currently split Sub-Arrays
     */
    while (i < left && j < right) {
        if (tempLeft[i] <= tempRight[j]) {
            col[k] = tempLeft[i]
            ++i
        } else {
            col[k] = tempRight[j]
            ++j
        }
        ++k
    }

    /**
     * Copies remaining elements, from each Sub-Array, back to the parent [IntArray]
     */
    while (i < left) {
        col[k] = tempLeft[i]
        ++i
        ++k
    }

    while (j < right) {
        col[k] = tempRight[j]
        ++j
        ++k
    }
}

/**
 * Sorts and merges two sub-arrays within a [List]
 *
 * First sub-array is the range `start .. mid`
 * Second sub-array is of range `mid+1 .. end`
 */
private fun merge(col: MutableList<Int>, start: Int, mid: Int, end: Int){
    /**
     * Gets each range for the Sub-Arrays
     */
    val left = mid - start + 1
    val right = end - mid

    /**
     * Creates temporary arrays for each Sub-Array
     */
    val tempLeft = IntArray(left)
    val tempRight = IntArray(right)

    /**
     * Copies data to each Sub-Array
     */
    left.repeatIndexed { tempLeft[it] = col[start + it] }
    right.repeatIndexed { tempRight[it] = col[mid + 1 + it] }

    //=========================================================
    //Below is to compare, sort, and merge the temporary arrays
    //=========================================================

    /**
     * Variables used to index the Sub-Arrays
     */
    var i = 0
    var j = 0
    var k = start

    /**
     * Quick-Sorts each element from the currently split Sub-Arrays
     */
    while (i < left && j < right) {
        if (tempLeft[i] <= tempRight[j]) {
            col[k] = tempLeft[i]
            ++i
        } else {
            col[k] = tempRight[j]
            ++j
        }
        ++k
    }

    /**
     * Copies remaining elements, from each Sub-Array, back to the parent [List]
     */
    while (i < left) {
        col[k] = tempLeft[i]
        ++i
        ++k
    }

    while (j < right) {
        col[k] = tempRight[j]
        ++j
        ++k
    }
}

private fun mergeSort(col: Array<Int>, start:Int = 0, end:Int) {
    if (start < end){
        //Gets mid-point
        val mid = (start + end) / 2

        //Sort each half
        mergeSort(col, start, mid)
        mergeSort(col, mid + 1, end)

        //Combine each half
        merge(col, start, mid, end)
    }
}
private fun mergeSort(col: IntArray, start:Int = 0, end:Int){
    if (start < end){
        //Gets mid-point
        val mid = (start + end) / 2

        //Sort each half
        mergeSort(col, start, mid)
        mergeSort(col, mid + 1, end)

        //Combine each half
        merge(col, start, mid, end)
    }
}
private fun mergeSort(col: MutableList<Int>, start:Int = 0, end:Int){
    if (start < end){
        //Gets mid-point
        val mid = (start + end) / 2

        //Sort each half
        mergeSort(col, start, mid)
        mergeSort(col, mid + 1, end)

        //Combine each half
        merge(col, start, mid, end)
    }
}

fun Array<Int>.mergeSort() = mergeSort(this, end = size - 1)
fun IntArray.mergeSort() = mergeSort(this, end = size - 1)
fun MutableList<Int>.mergeSort() = mergeSort(this, end = size - 1)

//If you use mergeSort() as an extension, it has to be called separately from usage (Or it will be considered as a Unit)
fun main() {
    val randomSize = Random.nextInt(3,20)

    val a = Array(randomSize){Random.nextInt(0, 100)}
    val b = IntArray(randomSize){Random.nextInt(0, 100)}
    val c = MutableList(randomSize){Random.nextInt(0, 100)}

    println("Random Merge-Sort of size: $randomSize")

    println("Array<Int> - Unsorted: ${a.contentToString()}")
    a.mergeSort()
    println("Array<Int> - Sorted  : ${a.contentToString()}")

    println("IntArray - Unsorted  : ${b.contentToString()}")
    b.mergeSort()
    println("IntArray - Sorted    : ${b.contentToString()}")

    println("List<Int> - Unsorted : $c")
    c.mergeSort()
    println("List<Int> - Sorted   : $c")
}
