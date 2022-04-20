import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.*
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

fun main(){
    //IPv4 & IPv6 both work
    println("Please enter an IPv4 or IPv6 address or type Self, for your own IP")
    var ip = readLine().takeUnless { it.equals("self",true) } ?: getDeviceIP()

    //A checker if you want a validation check
    while (!ip.isValidIP()){
        println("Invalid IP, Try again")
        ip = readLine().takeUnless { it.equals("self",true) } ?: getDeviceIP()
    }

    //Enum info lookup for individual data
    val info = ip.getIPInfo()
    println(info.getInfo(IP.CITY))
    println(info.getInfo(IP.REGION))
    println(info.getInfo(IP.COUNTRY_NAME))
}

//Gets your current devices' public IP
fun getDeviceIP(): String = BufferedReader(InputStreamReader(URL("http://checkip.amazonaws.com").openStream())).readLine()

//Gets JSON data from public IP lookup
fun String.getIPInfo():String{
    val client = HttpClient.newBuilder().build()
    val request = HttpRequest.newBuilder()
        .uri(URI.create("https://ipapi.co/$this/json"))
        .build()

    val response = client.send(request, HttpResponse.BodyHandlers.ofString())
    return response.body()
}

//Regex validations
val ipv4 = Regex("^((25[0-5]|(2[0-4]|1\\d|[1-9]|)\\d)(\\.(?!\$)|\$)){4}\$")
val ipv6 = Regex("(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|" +         //1:2:3:4:5:6:7:8
                "([0-9a-fA-F]{1,4}:){1,7}:|" +                          // 1::                              1:2:3:4:5:6:7::
                "([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|" +          // 1::8             1:2:3:4:5:6::8  1:2:3:4:5:6::8
                "([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|" +   // 1::7:8           1:2:3:4:5::7:8  1:2:3:4:5::8
                "([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|" +   // 1::6:7:8         1:2:3:4::6:7:8  1:2:3:4::8
                "([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|" +   // 1::5:6:7:8       1:2:3::5:6:7:8  1:2:3::8
                "([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|" +   // 1::4:5:6:7:8     1:2::4:5:6:7:8  1:2::8
                "[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|" +        // 1::3:4:5:6:7:8   1::3:4:5:6:7:8  1::8
                ":((:[0-9a-fA-F]{1,4}){1,7}|:)|" +                      // ::2:3:4:5:6:7:8  ::2:3:4:5:6:7:8 ::8       ::
                "fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|" +      // fe80::7:8%eth0   fe80::7:8%1     (link-local IPv6 addresses with zone index)
                "::(ffff(:0{1,4}){0,1}:){0,1}" +
                "((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\\.){3,3}" +
                "(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|" +           // ::255.255.255.255   ::ffff:255.255.255.255  ::ffff:0:255.255.255.255  (IPv4-mapped IPv6 addresses and IPv4-translated addresses)
                "([0-9a-fA-F]{1,4}:){1,4}:" +
                "((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\\.){3,3}" +
                "(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))")            // 2001:db8:3:4::192.0.2.33  64:ff9b::192.0.2.33 (IPv4-Embedded IPv6 Address)
fun String.isValidIP() = matches(ipv4) || matches(ipv6)

//Gets data from the lookup, without using external JSON library
fun String.getInfo(data:IP) = lines().first{ it.contains("\"${data.name.lowercase()}\"") }.split(": ").last().trimEnd(',').trim('"')

//Used for JSON lookup, if you want specific data only
enum class IP{
    IP,
    VERSION,
    CITY,
    REGION,
    REGION_CODE,
    COUNTRY,
    COUNTRY_NAME,
    COUNTRY_CODE,
    COUNTRY_CODE_ISO3,
    COUNTRY_CAPITAL,
    COUNTRY_TLD,
    CONTINENT_CODE,
    IN_EU,
    POSTAL,
    LATITUDE,
    LONGITUDE,
    TIMEZONE,
    UTC_OFFSET,
    COUNTRY_CALLING_CODE,
    CURRENCY,
    CURRENCY_NAME,
    LANGUAGES,
    COUNTRY_AREA,
    COUNTRY_POPULATION,
    ASN,
    ORG
}
