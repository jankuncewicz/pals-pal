set term epslatex
set logscale x
set output "out.tex"
set xlabel "$R$[nm]"
set ylabel "$\\tau$[ns]"
plot 'delta.txt' using 1:2 title '0.193 nm' with lines, 'delta.txt' using 1:3 title '0.18 nm' with lines, 'delta.txt' using 1:4 title '0.16 mn' with lines
