package wasm

import (
	"strconv"
	"syscall/js"
)

func GetFloatText(val string) float64 {
	doc := js.Global().Get("document")
	ans, _ := strconv.ParseFloat(doc.Call("getElementById", val).Get("value").String(), 64)
	return ans
}

func GetIntText(val string) int {
	doc := js.Global().Get("document")
	ans, _ := strconv.Atoi(doc.Call("getElementById", val).Get("value").String())
	return ans
}

func GetIntGlobal(val string) int {
	doc := js.Global().Get("document")
	return doc.Get(val).Int()
}

func SetAns(ans float64) {
	doc := js.Global().Get("document")
	doc.Call("getElementById", "ans").Set("value", ans)
}
