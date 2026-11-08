#!/bin/bash

printf "%-15s %-7s %-7s %-7s %-7s\n" "Файл" "A" "T" "G" "C"

for file in *.fasta; do
    if [ ! -s "$file" ]; then # инвертирование условия
        continue
    fi

    sequence=$(grep -v "^>" "$file" | tr -d '\n') # вывод всех строк кроме заголовков и склеиваем в 1 строку

    count_A=$(echo "$sequence" | grep -o "A" | wc -l)# выводит вхождения на каждой строке и считает число строк(вхождений)
    count_T=$(echo "$sequence" | grep -o "T" | wc -l)
    count_G=$(echo "$sequence" | grep -o "G" | wc -l)
    count_C=$(echo "$sequence" | grep -o "C" | wc -l)

    printf "%-15s %-7s %-7s %-7s %-7s\n" "$file" "$count_A" "$count_T" "$count_G" "$count_C"

done
