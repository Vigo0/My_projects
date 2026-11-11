#!/bin/bash

#Заменяем все пробелы (\t)
#  -i сохраняет изменения сразу 
sed -i 's/ /\t/g' sequences.txt

echo "Готово!"
