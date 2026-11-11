#!/bin/bash

echo " Студенты с оценкой выше 80 "
awk 'NF && $2 > 80 {print $1, $2}' students.txt


echo "Студенты с оценкой ниже 70 "
awk 'NF && $2 < 70 {print $1, $2}' students.txt

echo "Только первая строка файла "
awk 'NR == 1 {print $0}' students.txt
