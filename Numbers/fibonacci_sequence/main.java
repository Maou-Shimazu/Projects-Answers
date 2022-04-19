import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

public class Fibonacci {
    public static void main(String[] args){
        //Example
        fibonacci(5);
        //Would output
        //0,1,1,2,3
    }

    public static void fibonacci(int n){
        List<Integer> list = new ArrayList<>(Arrays.asList(0, 1));

        //Since cases don't allow for ranges, we'll assign it within the condition
        switch((n <= 0) ? 0 : (n <= 2) ? 1 : 2){
            case 0 -> System.out.println("Undefined");
            case 1 -> System.out.println(join(list.subList(0,n)));
            case 2 -> {
                for(int i = 2;i < n;i++){
                    int last = list.size() - 1;
                    list.add(list.get(last) + list.get(last - 1));
                }
                System.out.println(join(list));
            }
        }
    }

    //Joiner method
    public static <T> String join(List<T> list){
        StringBuilder stringBuilder  = new StringBuilder();
        Iterator<T> iterator = list.iterator();

        while(iterator.hasNext()){
            stringBuilder.append(iterator.next());
            if(iterator.hasNext()){
                stringBuilder.append(",");
            }
        }

        return stringBuilder.toString();
    }
}
