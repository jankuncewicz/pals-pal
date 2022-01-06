package main

import (
	"fmt"
	"pals_pal/bessel"
	"pals_pal/ete"
	"strconv"
	"syscall/js"
)

func getFloat(val string, doc js.Value) float64 {
	ans, _ := strconv.ParseFloat(doc.Call("getElementById", val).Get("value").String(), 64)
	return ans
}

func getInt(val string, doc js.Value) int {
	ans, _ := strconv.Atoi(doc.Call("getElementById", val).Get("value").String())
	return ans
}

func main() {
	document := js.Global().Get("document")
	tau := getFloat("tau", document)
	T := getFloat("temp", document)
	delta := getFloat("delta", document)
	n := getInt("n", document)
	m := getInt("m", document)
	zeros := bessel.Zeros(n, m)
	ints := bessel.IntegralList(n, m, zeros)
	ans := ete.ApproxR(tau, n, m, delta, T, zeros, ints)
	fmt.Println(ans)
	document.Call("getElementById", "ans").Set("value", ans)
}
