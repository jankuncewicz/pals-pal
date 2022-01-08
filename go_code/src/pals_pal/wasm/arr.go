package wasm

import "syscall/js"

func GetArr(val string) [][]float64 {
	zeros_js := js.Global().Get(val)

	n := zeros_js.Get("length").Int()
	ans := make([][]float64, n)
	for i := 0; i < n; i++ {
		m := zeros_js.Index(i).Get("length").Int()
		hold := make([]float64, m)
		for j := 0; j < m; j++ {
			hold[j] = zeros_js.Index(i).Index(j).Float()
		}
		ans[i] = hold
	}
	return ans
}

func SaveArr(val string, src [][]float64) {
	arr := js.Global().Get("Array")
	target := js.Global().Get(val)
	for i := 0; i < len(src); i++ {
		m := len(src[i])
		hold := arr.New(m)
		for j := 0; j < m; j++ {
			hold.SetIndex(j, src[i][j])
		}
		target.SetIndex(i, hold)
	}
}
