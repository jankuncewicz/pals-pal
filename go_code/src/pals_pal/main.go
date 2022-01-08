package main

// WARNING: don't use fmt in the final project
import (
	"pals_pal/bessel"
	"pals_pal/ete"
	"pals_pal/wasm"
)

func main() {
	tau := wasm.GetFloatText("tau")
	T := wasm.GetFloatText("temp")
	delta := wasm.GetFloatText("delta")
	n := wasm.GetIntText("n")
	m := wasm.GetIntText("m")

	zeros := wasm.GetArr("zeros")
	ints := wasm.GetArr("ints")
	if n > len(zeros) || m > len(zeros[len(zeros)-1]) {
		zeros = bessel.Zeros(n, m, zeros)
		ints = bessel.IntegralList(n, m, zeros, ints)
		wasm.SaveArr("zeros", zeros)
		wasm.SaveArr("ints", ints)
	}

	ans := ete.ApproxR(tau, n, m, delta, T, zeros, ints)
	//fmt.Println(ans)
	wasm.SetAns(ans)
}
