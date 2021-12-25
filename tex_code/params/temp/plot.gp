set term epslatex
set logscale x
set output "out.tex"
set xlabel "R[nm]"
set ylabel "$\\tau$[ns]"
plot 'test.txt' using 1:2 title '300K' with linespoints, 'test.txt' using 1:3 title '200K' with lines, 'test.txt' using 1:4 title '100K' with lines
