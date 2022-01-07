package main

// TODO: don't use fmt in the final project
import (
	//"fmt"
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
	//fmt.Println(ans)

	//TODO: zrobić z tego cały moduł do operowania wartościami js
	//Zapisywanie wartości
	arr := js.Global().Get("Array")
	zeros_js := js.Global().Get("zeros")
	for i := 0; i < n; i++ {
		hold := arr.New(m);
		for j := 0; j < m; j++ {
			hold.SetIndex(j, zeros[i][j])
		}
		zeros_js.SetIndex(i, hold)
	}

	// Pobieranie wartości
	//test := zeros_js.Index(0).Index(0).Float()
	//fmt.Println(test + 1)
	document.Call("getElementById", "ans").Set("value", ans)
}
