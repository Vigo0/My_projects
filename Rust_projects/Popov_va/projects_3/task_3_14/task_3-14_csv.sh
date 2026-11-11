#!/bin/bash


echo " Названия товаров "
awk -F, '{print $2}' data.csv

echo " Товары дороже 20 "
awk -F, '$3 > 20 {print $2, $3}' data.csv

echo " Общая стоимость "
awk -F, '{sum += $3} END {print sum}' data.csv

# F используется для смены разделителя на запятую
