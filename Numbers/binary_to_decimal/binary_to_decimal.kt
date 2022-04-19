import java.util.*

fun main(){
    val input = Scanner(System.`in`)

    //Binary to Decimal
    //Using input
    println("Enter Binary to Convert to Decimal")
    println(input.next().asDecimal())

    //Direct usage
    println("1011".asDecimal())

    //Decimal to Binary
    //Using input
    println("Enter Integer to Convert to Binary")
    println(input.nextInt().asBinary())

    //Direct usage
    println(1024.asBinary())
}

fun String.asDecimal() = try{ "${this.toInt(2)}" } catch (e:NumberFormatException){ "Not a valid Input" }
fun Int.asBinary():String = Integer.toBinaryString(this)
