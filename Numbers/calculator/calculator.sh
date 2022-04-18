echo "ez" >> test.txt
while :
do

read -p "equation to do or whatever: " eq

clear && curl -s -d "{\"math_to_do_ig_bestie\": \"${eq}\"}" -X POST https://hinyanicheatedstfuandcope-API.13-05.repl.co -D test.txt
grep "ans: " test.txt

done
