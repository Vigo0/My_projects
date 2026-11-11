#!/bin/bash

#!/bin/bash

echo " Только имена студентов "
awk '{print $1}' students.txt

echo " Только оценки "
awk '{print $2}' students.txt


echo " Номер строки и имя"
awk '{print NR, $1}' students.txt
