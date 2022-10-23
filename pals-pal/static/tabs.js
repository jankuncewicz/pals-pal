var tabsState = 1;

var run;

function changeState(st){
	if(st == tabsState) return
	document.getElementById("tab"+tabsState).style = "";
	document.getElementById("tab"+st).style = "background-color: #ccc;";
    tabsState = st
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