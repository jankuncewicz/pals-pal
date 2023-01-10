# tau.rs
Zaimplementowanie wzoru na $\tau$ są tutaj dwie funckje odbiegające
od normy.
## pnm
Liczy oczywiście $P_{nm}$ ze wzoru. Funkcja `int::gauss`
jest zaimportowana z `src/bessel/bessel_integral.rs`,
ponieważ całkowanie
$$\int_{Z_{nm}\frac{R}{R+\Delta}}^{Z_{nm}} J_m(r)^2 r dr$$
musi być liczone za każdym razem tutaj i dla liczenia (ale tylko raz)
całek od $0$ do $Z_{nm}$ użyte jest liczenie przez kwadratury gaussa.
Daje to bardzo dobry i szybki wynik, jeżeli liczenie funckji Bessela też
jest szybkie (a jest całkiem szybkie).
## tau
Poza kawałkiem kodu służącym do doliczania zer i całek w razie
potrzeby nie ma tu nic zdumiewającego. Jest tu oczywiście zastosowana
optymalizacja w stosunku do zapisu matematycznego, ponieważ
sumuję jednocześnie i licznik i mianownik, ponieważ sumowanie odbywa
się po tym samym zakresie. Na końcu zwracam iloraz obu tych sum, tak
jak powinno być.

# approx_r
Stosuję tutaj zwykłą metodę Newtona-Raphsona do wyliczenia wartości
$R$. Dla zakresu gdzie pochodna z ma dużą wartość ta metoda jest
szybko zbieżna, niestety dla fragmentów od ok. 130 ns (i wysokiej temperatury)
zaczynają się schody. Dla wartości o małej pochodnej (w tym wypadku mniejszej
od 0.1) przekazuję dalej liczenie do innego algorytmu - metody Brenta.

W głównej funckji `approx` wykonuje jakieś początkowe odgadnięcie tego
$R$. Po prostu "wymyśliłem" sobie funkcję podobną do $\tau(R)$ i używając
wxMaxima policzyłem jej funkcję odwrotną (po konkretny wzór proszę się
zwrócić na koniec pliku `tex_code/notes.pdf`.
