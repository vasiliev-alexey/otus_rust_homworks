## ToDo

### Цель: В этом ДЗ вы реализуете библиотеку для работы с матрицами произвольного размера и их наборами.

### Описание / Пошаговая инструкция выполнения домашнего задания:

Матрицы:

* Матрица представлена в виде массива элементов.
* Длина строки матрицы - const generic параметер.
* Элемент матрицы - обобщённый тип.
* Для матриц, элементы которых можно складывать, реализовать метод: добавление заданного элемента к каждому элементу в
  матрице. Подсказка: возможность сложения типов определяется трейтом std::ops::Add.
* Для матриц, элементы которых можно умножать, реализовать метод: умножение каждого элемента в матрице на заданный
  элемент. Подсказка: возможность умножения типов определяется трейтом std::ops::Mul.

Набор матриц:

* Реализовать структуру, представляющую собой набор матриц.
* Набор матриц - это слайс, элементы которого - ссылка на матрицу.
* Реализовать метод - получение ссылки на матрицу по порядковому номеру в слайсе. Убедиться, что полученная ссылка живёт
  дольше экземпляра набора матриц. Её время жизни должно определяться временем жизни матрицы, а не набора матриц.
* Если элементы матриц можно складывать, реализовать метод - сложение всех элементов всех матриц.
* Если элементы матриц можно перемножать, реализовать метод - перемножение всех элементов всех матриц.

#### Требования

1. Все перечисленные методы реализованы.
2. Все методы протестированы.
3. ""cargo clippy"" и ""cargo fmt --check"" не выдают предупреждений и ошибок.

---

<details>

<summary> Покрытие тестов</summary>

[How to do code coverage in Rust](https://blog.rng0.io/how-to-do-code-coverage-in-rust)

setup:
```sh
cargo install grcov cargo-tarpaulin
rustup component add llvm-tools-preview
```



Тесты для проверки покрытия (в директории проекта)
```shell
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
```
Формирование отчета о покрытии (в директории проекта)
```shell
grcov . --binary-path ../target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o ../target/coverage/html 
```

</details>