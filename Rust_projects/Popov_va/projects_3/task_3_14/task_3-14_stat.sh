#!/bin/bash

echo "Сумма всех оценок"
awk 'NF {sum += $2} END {print sum}' students.txt


echo " Средvняя оценка"
awk 'NF {sum += $2; count++} END {print sum / count}' students.txt

#NF пропускаем пустые строки


echo " Максимальная оценка "
awk 'NR==1{max=$2} $2>max{max=$2} END{print max}' students.txt
