# bessel
Są to moduły do liczenia funkcji Bessela. Tak jak wspominałem jest to w całości
implementacja z biblioteki GSL (GNU Scientific Library).

Tutaj w ramach wyjaśnienia szybko wspomnę o jednej rzeczy. Sam kod niestety musiał
sam dosłownie przekonwertować z C++ do Rust, ponieważ nikt tego za bardzo nie zrobił.
Inne biblioteki implementujące GSL w Rustcie tylko i wyłącznie linkują już skompilowaną
bibliotekę w C++. Taka praktyka jest niestety zabroniona w WebAssembly. Z tego co szukałem
najłatwiejszym (albo nawet jedynym) rozwiązaniem było tp po prostu przepisać.
Z tego powodu podczas kompilacji będzie Pan widział dużo ostrzeżeń, które tyczą się jedynie
samej konwencji nazw, która nie pasuje Rustowi.
# ete
Implementacja rozszerzonego modelu Tao-Eldrupa i funkcji odwrotnej liczącej promień.
Jest to dla cylindryczny porów. Wzorowałem się praktycznie całkowicie na wzorze
w Pana artykule "Principles of positron porosimetry"
# plotting
Jeszcze we wczesnej fazie moduł do wszelkiego rodzaju rysowania. Na tę chwilę są
tam tylko przydatne definicje obiektów i przykładowa funkcja rysująca wykres $x^n$
dla podanego $n$.
# lib.rs
Odpowiednik pliku "main" dla kompilacji do wasm. Wszystkie funkcje i
struktury z nagłówkiem `#[wasm_bindgen]` są udostępniane i mogą być użyte
w pliku `js/index.js`.

Funkcje liczące $R$ i $\tau$ mają ustawione "na sztywno" 3 parametry.
Jest to maksymalna ilość generowanych poziomów i "dokładność".
Ustawiona ona na 0.000001 mówi, że wkład do sumy przy liczeniu $\tau$
jest mniejszy od 0.000001.