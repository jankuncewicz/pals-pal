# bessel
Są to moduły do liczenia funkcji Bessela. Tak jak wspominałem jest to w całości
implementacja z biblioteki GSL (GNU Scientific Library).

Tutaj w ramach wyjaśnienia szybko wspomnę o jednej rzeczy. Sam kod niestety musiałem
sam dosłownie przekonwertować z C++ do Rust, ponieważ nikt tego za bardzo nie zrobił.
Inne biblioteki implementujące GSL w Rustcie tylko i wyłącznie linkują już skompilowaną
bibliotekę w C++. Taka praktyka jest niestety zabroniona w WebAssembly. Z tego co szukałem
najłatwiejszym (albo nawet jedynym) rozwiązaniem było to po prostu przepisać.
Z tego powodu podczas kompilacji będzie widać dużo ostrzeżeń, które tyczą się jedynie
samej konwencji nazw, która nie pasuje Rustowi.
# ete
Implementacja rozszerzonego modelu Tao-Eldrupa i funkcji odwrotnej liczącej promień.
Jest to dla cylindryczny porów. Wzorowałem się praktycznie całkowicie na wzorze
w artykule "Principles of positron porosimetry"
# plotting
Moduł do rysowania wykresów na stronie.
# lib.rs
Odpowiednik pliku "main" dla kompilacji do wasm. Wszystkie funkcje i
struktury z nagłówkiem `#[wasm_bindgen]` są udostępniane i mogą być użyte
w pliku `www/index.js`.
