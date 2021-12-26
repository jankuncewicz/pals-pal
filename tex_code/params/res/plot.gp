set term epslatex
set logscale x
set output "out.tex"
set xlabel "$R$[nm]"
set ylabel "$\\tau$[ns]"
plot 'res.txt' using 1:2 title '$n$=100' with lines, 'res.txt' using 1:3 title '$n$=50' with lines, 'res.txt' using 1:4 title '$n$=20' with lines
