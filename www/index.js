import init, { calculate_array_r, calculate_array_tau, calculate_r, calculate_tau } from "../pals-pal/pkg/pals_pal.js";

function get(s){
	return document.getElementById(s).value;
}

async function run_wasm(){
	await init();
    run = () => {
		let r, temp, delta, tau;
		let state = document.getElementById("check").checked
		document.getElementById("submit").disabled = true
		switch(tabsState){
			case 0:
 	 	  		r = parseFloat(get("ans"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
				if(state){
					document.getElementById("file").disabled = true
					readFile().then((times) => {
						let rs = new Float64Array(times)
						calculate_array_tau(rs, delta, temp, "canvas")
					})
				}
				else{
	    			tau = calculate_tau(r, delta, temp);
    				document.getElementById("tau").value = tau;
				}
				document.getElementById("submit").disabled = false
				break
			case 1:
 	 	  		tau = parseFloat(get("tau"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
				if(state){
					document.getElementById("file").disabled = true
					readFile().then((times) => {
						let rs = new Float64Array(times)
						calculate_array_r(rs, delta, temp, "canvas")
					})
				}
				else{
	    			r = calculate_r(tau, delta, temp);
    				document.getElementById("ans").value = r;
				}
				document.getElementById("submit").disabled = false
    }}
    //let chart = Chart.power("canvas", Number(2));
}
run_wasm();
