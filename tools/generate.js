let fs = require("fs");

let wasi = JSON.parse(fs.readFileSync("wasi.json", "utf8"));

function processFunction(f){

}

let namespaces = {};
for(var i in wasi){
  let f = wasi[i];
  let nSplit = f.name.split("_");
  let namespace = nSplit[3];
  nSplit.shift();
  nSplit.shift();
  nSplit.shift()
  nSplit.shift();
  let name = nSplit.join("_");
  if(!namespaces[namespace]){
    namespaces[namespace] = [];
  }
  namespaces[namespace].push({
    "import":f.name,
    name:name,
    inputs:f.inputs,
    outputs:f.outputs,
  });
}

function finalName(n){
  if(n == "yield"){
    return "r#yield";
  }
  return n;
}

for(var i in namespaces){
  let s = "#[allow(unused_imports)]\nuse crate::*;\n";
  let namespace = namespaces[i];
  for(var j in namespace){
    let func = namespace[j];
    console.log(func)
    s += `extern "C" {\n`;
    s += `fn ${func.import}(${func.inputs?"input_start:i32":""})${func.outputs?" -> i32":""};\n`;
    s+= `}\n\n`;
    s += `pub fn ${finalName(func.name)}(){\n`;
    s += `  unsafe{${func.import}(${func.inputs?"0":""});}`;
    s+= `}\n\n`;
  }
  fs.writeFileSync(
    "src/" + i + ".rs",
    s
  );
}

let s = ""
for(var i in namespaces){
  s += `pub mod ${i};\n`;
}
fs.writeFileSync(
  "src/lib.rs",
  s
);
