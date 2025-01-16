## LedgeRustyДанный сервис - простой пример построения блокчейна в Rust.### Некоторые детали реализации1) Функция `new` создает экземпляр Blockchain, содержащий сгенерированный `genesis block` (первый блок в цепочке) добавленный в цепочку.2) Функция `update` обновляет некоторые поля у созданного экземпляра Blockchain (поле `timestamp` в `genesis block`, поле `difficulty`, поле `chain`).3) Функция `add_block` принимающая текущий экземпляр Blockchain. Создает новый блок, вычисляем его хеш, добавляем в цепочку.4) Функция `get_all_blocks` принимающий текущий экземпляр Blockchain, и возвращающий всю цепочку имеющихся блоков.UPD: По дефолту установлен уровень сложности 1 (желаемого количества нулей, что является сложностью для майнинга нового блока).### Локальный запуск проектаДля запуска необходимо найти и запустить бинарник (в директории `/target/release`). Либо (при наличии установленных `rust` и `cargo`) запустить в терминале.### Взаимодействие с юзером- У сервиса реализован CLI. Для просмотра доступных команд введите в терминале `help`.- По дефолту установлен уровень сложности 1 (желаемого количества нулей, что является сложностью для майнинга нового блока).- Изменить уровень сложности можно командой `diff <value>`.- Создать `genesis block` можно командой `create`.- Добавить блок в цепочку, и просмотреть ее целиком (можно командами `check` и `add`).### Необходимые доработки1) Добавление хранения полезной нагрузки.2) Проверка целостности цепочки.3) Общий рефакторинг кода, и структуры проекта.