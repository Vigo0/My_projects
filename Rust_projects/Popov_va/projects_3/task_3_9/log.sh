#!/bin/bash

FILE_PATH="./system.log"
ERROR_CODE=1

# 1. Файловый тест
if [ -f "$FILE_PATH" ]; then
    echo "Лог-файл найден."
    ERROR_CODE=0
else
    echo "Ошибка: файл не существует."
fi

# 2. Числовое сравнение через Case
case $ERROR_CODE in
    0)
        echo "Статус: Ошибок нет." ;;
    1)
        echo "Статус: Критическая ошибка!" ;;
    *)
        echo "Статус: Неизвестный код." ;;
esac
