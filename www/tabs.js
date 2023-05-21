var tabsState = 1;

var run;
var loadFileFlag = false

function clearAll(){
    document.getElementById("tau").value = ""
    document.getElementById("ans").value = ""
}

function changeState(st){
	if(st == tabsState) return
	document.getElementById("tab"+tabsState).style = "";
	document.getElementById("tab"+st).style = "background-color: #ccc;";
    tabsState = st
    clearAll()
    switch(tabsState){
        case 0:
            document.getElementById("tau").readOnly = true       
            document.getElementById("tau").required = false       
            document.getElementById("ans").required = true       
            document.getElementById("ans").readOnly = false       
            document.getElementById("file").disabled = true
            break
        case 1:
            document.getElementById("tau").readOnly = false       
            document.getElementById("tau").required = true      
            document.getElementById("ans").required = false       
            document.getElementById("ans").readOnly = true       
            document.getElementById("file").disabled = true
            break
        case 2:
            document.getElementById("tau").readOnly = true       
            document.getElementById("tau").required = false       
            document.getElementById("ans").required = false       
            document.getElementById("ans").readOnly = true       
            document.getElementById("file").disabled = false
    }
}

function parseFile(file) {
    let parsed = file.split(/\r?\n|\n/g)
    parsed.pop()
    parsed = parsed.map(x => parseFloat(x.split(",")[0]))
    return parsed
}

function readFile(){
    let file = document.getElementById("file").files[0]
    if(file != undefined){
        return file.text().then(t => {return parseFile(t)})
    }
    else
        alert("Please enter valid file!")
}

/*
const n = 200
var zeros = new Float64Array(n*n)
async function test() {
    var mod = await Module()
    for (let i = 0; i < n; i++) {
        for (let j = 1; j <= n; j++) {
            zeros[i*n + j - 1] = mod._gsl_sf_bessel_zero_Jnu(i, j)
        }
    }
}
*/