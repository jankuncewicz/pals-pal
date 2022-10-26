var tabsState = 1;

var run;
var parser;

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

var parseFile;

async function readFile(){
    let file = await document.getElementById("file").files[0].text()
    parseFile(file) 
}