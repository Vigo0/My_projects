#!/bin/bash

read -p "Введите ваш рост (в см): " USER_HEIGHT
echo "Ваш рост: $USER_HEIGHT"

read -p "Введите ваш вес (в кг): " USER_WEIGHT
echo "Ваш вес: $USER_WEIGHT"

imt=$(( (USER_WEIGHT * 10000) / (USER_HEIGHT * USER_HEIGHT) ))

echo "Ваш ИМТ: $imt"
