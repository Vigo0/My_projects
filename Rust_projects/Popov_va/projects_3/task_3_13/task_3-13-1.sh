#!/bin/bash

# Использую '|' вместо '/', чтобы не экранировать пути
sed -i 's|/var/lib/mysql/data|/mnt/ssd/mysql|' settings.php

echo "Путь успешно заменен!"
