Poza plikiem `bessel_integral.rs` wszytskie są kalką z
biblioteki GSL, jak to już wcześniej wyjaśniłem.
Znajdzie je Pan konkretnie pod adresem
https://git.savannah.gnu.org/cgit/gsl.git/tree/specfunc

# bessel_integral.rs
Tak jak tłumaczyłem to przy okazji liczenia tau, całka tutaj jest
policzona za pomocą kwadratur Gaussa. Jest to zaimplementowane bezpośrednio
bez żadnych modyfikacji.
Zauważyłem, że jest ona lepsza od tradycyjnych, iteracyjnych metod całkowania
pod tym względem, że dla większego zakresu całkowania daje ciągle dobre
wyniki przy małej ilości iteracji.
